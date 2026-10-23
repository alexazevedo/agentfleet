//! `agentfleet` CLI (the app binary run with `--cli`). Sync std I/O only: no Tauri, no tokio.
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Map, Value};

const USAGE: &str = "usage: agentfleet <command>

commands:
  list                      list the sessions in this project
  send <session> <text...>  send a request to an agent session (id8 or full id)
  spawn [--agent claude|codex|shell] [--name <label>] [--role <name>] [--agent-arg <arg>]... <prompt...|->
                            start a sibling session, queue the prompt, print '<id8> <run>'
  close <session>           close a sibling session in this project
  inbox                     list requests to/from this session
  read <run>                show a request and its response
  reply <run> <text...>     answer a request addressed to this session
  cancel <run>              cancel a queued/running request
  wait <run>                block until a request finishes, print its response
  scratchpad read           print this project's scratchpad (revision on stderr)
  scratchpad write [--if-revision N] <text...|->
                            replace the scratchpad ('-' reads stdin)
  scratchpad append [--if-revision N] <text...|->
                            append a line to the scratchpad ('-' reads stdin)
  guide                     print the agentfleet skill (works outside sessions)
  skill install             write the skill to ~/.claude/skills and ~/.codex/skills (outside sessions too)
  hook-context              print the skill as SessionStart hook JSON (used by the app's hooks)
  help                      show this help
";

/// The agent skill, printed by `agentfleet guide`.
const GUIDE: &str = include_str!("../../skills/agentfleet/SKILL.md");

#[derive(Debug, PartialEq)]
pub enum Cmd {
    List,
    Help,
    Guide,
    HookContext,
    SkillInstall,
    Send {
        session: String,
        text: String,
    },
    Inbox,
    Read {
        run: String,
    },
    Reply {
        run: String,
        text: String,
    },
    Cancel {
        run: String,
    },
    Wait {
        run: String,
    },
    /// `text: None` means read stdin.
    Spawn {
        agent: Option<String>,
        name: Option<String>,
        role: Option<String>,
        agent_args: Vec<String>,
        text: Option<String>,
    },
    Close {
        session: String,
    },
    ScratchRead,
    /// `text: None` means read stdin.
    ScratchWrite {
        text: Option<String>,
        if_revision: Option<u64>,
        append: bool,
    },
}

fn parse_scratch(rest: &[String]) -> Result<Cmd, String> {
    let (verb, mut args) = match rest.split_first() {
        Some((v, a)) => (v.as_str(), a),
        None => return Err("scratchpad needs read, write or append".into()),
    };
    if verb == "read" {
        return if args.is_empty() {
            Ok(Cmd::ScratchRead)
        } else {
            Err("scratchpad read takes no arguments".into())
        };
    }
    if verb != "write" && verb != "append" {
        return Err(format!("unknown scratchpad verb: {verb}"));
    }
    let mut if_revision = None;
    if args.first().map(String::as_str) == Some("--if-revision") {
        let n = args.get(1).ok_or("--if-revision needs a number")?;
        if_revision = Some(n.parse().map_err(|_| "--if-revision needs a number")?);
        args = &args[2..];
    }
    let text = match args {
        [] => return Err(format!("scratchpad {verb} needs text or -")),
        [d] if d == "-" => None,
        _ => Some(args.join(" ")),
    };
    Ok(Cmd::ScratchWrite {
        text,
        if_revision,
        append: verb == "append",
    })
}

fn parse_spawn(rest: &[String]) -> Result<Cmd, String> {
    let (mut agent, mut name, mut role) = (None, None, None);
    let mut agent_args = Vec::new();
    let mut i = 0;
    while i < rest.len() && rest[i] != "-" && rest[i].starts_with("--") {
        let flag = rest[i].as_str();
        let val = rest
            .get(i + 1)
            .cloned()
            .ok_or(format!("{flag} needs a value"))?;
        match flag {
            "--agent" => agent = Some(val),
            "--name" => name = Some(val),
            "--role" => role = Some(val),
            "--agent-arg" => agent_args.push(val),
            other => return Err(format!("unknown option: {other}")),
        }
        i += 2;
    }
    let words = &rest[i..];
    let text = match words {
        [] => None,
        [d] if d == "-" => None,
        _ => Some(words.join(" ")),
    };
    if agent.as_deref() == Some("shell") {
        if !words.is_empty() {
            return Err("shell sessions take no prompt".into());
        }
    } else if words.is_empty() {
        return Err("spawn needs a prompt or -".into());
    }
    Ok(Cmd::Spawn {
        agent,
        name,
        role,
        agent_args,
        text,
    })
}

/// Prepend a role's text (plus a blank line) to the prompt.
pub fn with_role(role_text: Option<&str>, prompt: &str) -> String {
    match role_text {
        Some(r) => format!("{}\n\n{}", r.trim_end(), prompt),
        None => prompt.to_string(),
    }
}

pub fn parse(args: &[String]) -> Result<Cmd, String> {
    let rest = &args[args.len().min(1)..];
    let one = |name: &str| -> Result<String, String> {
        match rest {
            [x] => Ok(x.clone()),
            _ => Err(format!("{name} takes exactly one argument")),
        }
    };
    let with_text = |name: &str| -> Result<(String, String), String> {
        match rest {
            [x, t @ ..] if !t.is_empty() => Ok((x.clone(), t.join(" "))),
            _ => Err(format!("{name} needs an id and some text")),
        }
    };
    match args.first().map(String::as_str) {
        None | Some("help") | Some("--help") | Some("-h") => Ok(Cmd::Help),
        Some("list") if rest.is_empty() => Ok(Cmd::List),
        Some("list") => Err("list takes no arguments".into()),
        Some("inbox") if rest.is_empty() => Ok(Cmd::Inbox),
        Some("inbox") => Err("inbox takes no arguments".into()),
        Some("guide") if rest.is_empty() => Ok(Cmd::Guide),
        Some("guide") => Err("guide takes no arguments".into()),
        Some("hook-context") if rest.is_empty() => Ok(Cmd::HookContext),
        Some("hook-context") => Err("hook-context takes no arguments".into()),
        Some("skill") => match rest {
            [v] if v == "install" => Ok(Cmd::SkillInstall),
            _ => Err("skill needs install".into()),
        },
        Some("send") => with_text("send").map(|(session, text)| Cmd::Send { session, text }),
        Some("reply") => with_text("reply").map(|(run, text)| Cmd::Reply { run, text }),
        Some("read") => one("read").map(|run| Cmd::Read { run }),
        Some("cancel") => one("cancel").map(|run| Cmd::Cancel { run }),
        Some("scratchpad") => parse_scratch(rest),
        Some("spawn") => parse_spawn(rest),
        Some("close") => one("close").map(|session| Cmd::Close { session }),
        Some("wait") => one("wait").map(|run| Cmd::Wait { run }),
        Some(other) => Err(format!("unknown command: {other}")),
    }
}

/// Exit codes: 0 ok, 1 error / terminal failure, 2 usage or not in a session, 3 wait cycle, 4 scratchpad revision mismatch.
pub fn main(args: Vec<String>) -> i32 {
    let cmd = match parse(&args) {
        Ok(Cmd::Help) => {
            eprint!("{USAGE}");
            return 2;
        }
        Ok(Cmd::Guide) => {
            print!("{GUIDE}");
            return 0;
        }
        Ok(Cmd::HookContext) => {
            println!("{}", hook_context_json());
            return 0;
        }
        Ok(Cmd::SkillInstall) => {
            return match install_skill(&crate::platform::home_dir()) {
                Ok(paths) => {
                    for p in paths {
                        println!("wrote {}", p.display());
                    }
                    0
                }
                Err(e) => {
                    eprintln!("agentfleet: {e}");
                    1
                }
            };
        }
        Ok(c) => c,
        Err(e) => {
            eprintln!("agentfleet: {e}");
            eprint!("{USAGE}");
            return 2;
        }
    };
    if env().is_none() {
        eprintln!("agentfleet: not running inside an agentfleet session");
        return 2;
    }
    match run(cmd) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("agentfleet: {e}");
            if e == "wait cycle" {
                3
            } else if e.starts_with("revision mismatch") {
                4
            } else {
                1
            }
        }
    }
}

/// SessionStart hook output that injects the guide into the agent's context
/// (`hookSpecificOutput.additionalContext`, understood by Claude Code and Codex).
pub fn hook_context_json() -> String {
    serde_json::json!({
        "hookSpecificOutput": {
            "hookEventName": "SessionStart",
            "additionalContext": GUIDE,
        }
    })
    .to_string()
}

/// Where `skill install` writes: always Claude's skills dir, Codex's too when `~/.codex` exists.
pub fn skill_targets(home: &Path, codex_exists: bool) -> Vec<PathBuf> {
    let mut v = vec![home.join(".claude/skills/agentfleet/SKILL.md")];
    if codex_exists {
        v.push(home.join(".codex/skills/agentfleet/SKILL.md"));
    }
    v
}

fn install_skill(home: &Path) -> Result<Vec<PathBuf>, String> {
    let targets = skill_targets(home, home.join(".codex").is_dir());
    for p in &targets {
        if let Some(dir) = p.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        std::fs::write(p, GUIDE).map_err(|e| format!("{}: {e}", p.display()))?;
    }
    Ok(targets)
}

fn obj(pairs: &[(&str, &str)]) -> Map<String, Value> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), (*v).into()))
        .collect()
}

fn read_stdin() -> Result<String, String> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Full id of a session in this project, given an id8 or full id.
fn resolve_session(session: &str) -> Result<String, String> {
    let list = call("session.list", Map::new())?;
    list["sessions"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|s| s["id8"] == session || s["id"] == session)
        .and_then(|s| s["id"].as_str())
        .map(str::to_string)
        .ok_or(format!("no such session in this project: {session}"))
}

fn run(cmd: Cmd) -> Result<i32, String> {
    match cmd {
        Cmd::Help | Cmd::Guide | Cmd::HookContext | Cmd::SkillInstall => unreachable!(),
        Cmd::List => {
            print_list(&call("session.list", Map::new())?);
            Ok(0)
        }
        Cmd::Spawn {
            agent,
            name,
            role,
            agent_args,
            text,
        } => {
            let text = match text {
                Some(t) => Some(t),
                None if agent.as_deref() == Some("shell") => None,
                None => Some(read_stdin()?),
            };
            let text = match (text, role) {
                (Some(t), Some(role)) => {
                    let path = crate::platform::home_dir()
                        .join(".agentfleet/roles")
                        .join(format!("{role}.md"));
                    let file = std::fs::read_to_string(&path)
                        .map_err(|_| format!("no such role: {}", path.display()))?;
                    Some(with_role(Some(&file), &t))
                }
                (t, _) => t,
            };
            let mut extra = Map::new();
            if let Some(a) = agent {
                extra.insert("agent".into(), a.into());
            }
            if let Some(n) = name {
                extra.insert("name".into(), n.into());
            }
            if let Some(t) = text {
                extra.insert("text".into(), t.into());
            }
            if !agent_args.is_empty() {
                extra.insert("agentArgs".into(), agent_args.into());
            }
            let v = call_timeout("session.spawn", extra, 30)?;
            println!(
                "{} {}",
                v["session"]["id8"].as_str().unwrap_or(""),
                v["run"]["id"].as_str().unwrap_or("")
            );
            Ok(0)
        }
        Cmd::Close { session } => {
            let id = resolve_session(&session)?;
            let v = call_timeout("session.close", obj(&[("target", &id)]), 30)?;
            if v["outcome"]["kind"] == "kept" {
                println!("kept {}", v["outcome"]["path"].as_str().unwrap_or(""));
            }
            Ok(0)
        }
        Cmd::Send { session, text } => {
            let id = resolve_session(&session)?;
            let v = call("session.request", obj(&[("target", &id), ("text", &text)]))?;
            println!("{}", v["run"]["id"].as_str().unwrap_or(""));
            Ok(0)
        }
        Cmd::Inbox => {
            print_inbox(&call("session.inbox", Map::new())?);
            Ok(0)
        }
        Cmd::Read { run } => {
            let v = call("run.read", obj(&[("requestId", &run)]))?;
            let r = &v["run"];
            let s = |k: &str| r[k].as_str().unwrap_or("").to_string();
            println!("status: {}", s("status"));
            println!("prompt:\n{}", s("prompt"));
            if let Some(resp) = r["response"].as_str() {
                println!("response:\n{resp}");
            }
            Ok(0)
        }
        Cmd::Reply { run, text } => {
            call("run.reply", obj(&[("requestId", &run), ("text", &text)]))?;
            Ok(0)
        }
        Cmd::Cancel { run } => {
            call("run.cancel", obj(&[("requestId", &run)]))?;
            Ok(0)
        }
        Cmd::ScratchRead => {
            let v = call("scratchpad.read", Map::new())?;
            print!("{}", v["text"].as_str().unwrap_or(""));
            eprintln!("revision: {}", v["revision"].as_u64().unwrap_or(0));
            Ok(0)
        }
        Cmd::ScratchWrite {
            text,
            if_revision,
            append,
        } => {
            let text = match text {
                Some(t) => t,
                None => read_stdin()?,
            };
            let mut extra = obj(&[("text", &text)]);
            if let Some(r) = if_revision {
                extra.insert("revision".into(), r.into());
            }
            let m = if append {
                "scratchpad.append"
            } else {
                "scratchpad.write"
            };
            let v = call(m, extra)?;
            println!("revision: {}", v["revision"].as_u64().unwrap_or(0));
            Ok(0)
        }
        Cmd::Wait { run } => loop {
            // The broker holds the call for up to 25 s; allow headroom over its budget.
            let v = call_timeout("run.wait", obj(&[("requestId", &run)]), 40)?;
            let r = &v["run"];
            match r["status"].as_str().unwrap_or("") {
                "completed" => {
                    println!("{}", r["response"].as_str().unwrap_or(""));
                    return Ok(0);
                }
                "queued" | "running" => continue,
                other => {
                    eprintln!("agentfleet: run {other}");
                    return Ok(1);
                }
            }
        },
    }
}

/// (socket, session id, token) from the PTY environment.
fn env() -> Option<(String, String, String)> {
    let get = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
    Some((
        get("AGENTFLEET_SOCKET")?,
        get("AGENTFLEET_SESSION")?,
        get("AGENTFLEET_TOKEN")?,
    ))
}

/// Send one request (`method` + auth + `extra` fields) and return the response.
/// A `{"error": e}` response becomes `Err(e)`.
pub fn call(method: &str, extra: Map<String, Value>) -> Result<Value, String> {
    call_timeout(method, extra, 10)
}

fn call_timeout(method: &str, extra: Map<String, Value>, secs: u64) -> Result<Value, String> {
    let (socket, source, token) =
        env().ok_or("not running inside an agentfleet session".to_string())?;
    let mut req = extra;
    req.insert("method".into(), method.into());
    req.insert("source".into(), source.into());
    req.insert("token".into(), token.into());
    let mut line = serde_json::to_string(&Value::Object(req)).map_err(|e| e.to_string())?;
    line.push('\n');

    let mut stream =
        UnixStream::connect(&socket).map_err(|e| format!("cannot reach agentfleet: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(secs)))
        .map_err(|e| e.to_string())?;
    stream
        .write_all(line.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut out = String::new();
    BufReader::new(stream)
        .read_line(&mut out)
        .map_err(|e| format!("no response from agentfleet: {e}"))?;
    let v: Value = serde_json::from_str(&out).map_err(|e| format!("bad response: {e}"))?;
    if let Some(e) = v.get("error") {
        return Err(e
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| e.to_string()));
    }
    Ok(v)
}

fn print_list(v: &Value) {
    let s = |x: &Value, k: &str| x[k].as_str().unwrap_or("").to_string();
    println!("{:<8}  {:<6}  {:<8}  id", "id8", "agent", "status");
    for row in v["sessions"].as_array().into_iter().flatten() {
        println!(
            "{:<8}  {:<6}  {:<8}  {}",
            s(row, "id8"),
            s(row, "agent"),
            s(row, "status"),
            s(row, "id")
        );
    }
}

fn print_inbox(v: &Value) {
    let s = |x: &Value, k: &str| x[k].as_str().unwrap_or("").to_string();
    let id8 = |x: &Value, k: &str| {
        s(x, k)
            .chars()
            .filter(|c| *c != '-')
            .take(8)
            .collect::<String>()
    };
    for r in v["runs"].as_array().into_iter().flatten() {
        let prompt: String = s(r, "prompt").replace('\n', " ").chars().take(60).collect();
        println!(
            "{}  {:<14}  {}\u{2192}{}  {}",
            id8(r, "id"),
            s(r, "status"),
            id8(r, "fromSessionId"),
            id8(r, "toSessionId"),
            prompt
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_verbs() {
        assert_eq!(parse(&a(&[])), Ok(Cmd::Help));
        assert_eq!(parse(&a(&["--help"])), Ok(Cmd::Help));
        assert_eq!(parse(&a(&["list"])), Ok(Cmd::List));
        assert!(parse(&a(&["list", "x"])).is_err());
        assert_eq!(
            parse(&a(&["send", "ab12", "hello", "there"])),
            Ok(Cmd::Send {
                session: "ab12".into(),
                text: "hello there".into()
            })
        );
        assert!(parse(&a(&["send", "ab12"])).is_err());
        assert_eq!(parse(&a(&["inbox"])), Ok(Cmd::Inbox));
        assert_eq!(parse(&a(&["guide"])), Ok(Cmd::Guide));
        assert!(parse(&a(&["guide", "x"])).is_err());
        assert_eq!(parse(&a(&["hook-context"])), Ok(Cmd::HookContext));
        assert!(parse(&a(&["hook-context", "x"])).is_err());
        assert_eq!(parse(&a(&["skill", "install"])), Ok(Cmd::SkillInstall));
        assert!(parse(&a(&["skill"])).is_err());
        assert!(parse(&a(&["skill", "remove"])).is_err());
        assert_eq!(
            parse(&a(&["read", "r1"])),
            Ok(Cmd::Read { run: "r1".into() })
        );
        assert_eq!(
            parse(&a(&["reply", "r1", "pong"])),
            Ok(Cmd::Reply {
                run: "r1".into(),
                text: "pong".into()
            })
        );
        assert_eq!(
            parse(&a(&["cancel", "r1"])),
            Ok(Cmd::Cancel { run: "r1".into() })
        );
        assert_eq!(
            parse(&a(&["wait", "r1"])),
            Ok(Cmd::Wait { run: "r1".into() })
        );
        assert!(parse(&a(&["wait"])).is_err());
        assert_eq!(
            parse(&a(&[
                "spawn",
                "--agent",
                "claude",
                "--name",
                "rev",
                "--role",
                "r",
                "--agent-arg",
                "--model",
                "--agent-arg",
                "opus",
                "do",
                "it"
            ])),
            Ok(Cmd::Spawn {
                agent: Some("claude".into()),
                name: Some("rev".into()),
                role: Some("r".into()),
                agent_args: vec!["--model".into(), "opus".into()],
                text: Some("do it".into())
            })
        );
        assert_eq!(
            parse(&a(&["spawn", "-"])),
            Ok(Cmd::Spawn {
                agent: None,
                name: None,
                role: None,
                agent_args: vec![],
                text: None
            })
        );
        assert!(parse(&a(&["spawn", "--agent", "shell", "hi"])).is_err());
        assert!(parse(&a(&["spawn", "--agent", "shell"])).is_ok());
        assert_eq!(
            parse(&a(&["spawn", "--name", "x"])),
            Err("spawn needs a prompt or -".into())
        );
        assert!(parse(&a(&["spawn", "--bogus", "x", "hi"])).is_err());
        assert_eq!(
            parse(&a(&["close", "ab12"])),
            Ok(Cmd::Close {
                session: "ab12".into()
            })
        );
        assert!(parse(&a(&["close"])).is_err());
        assert_eq!(parse(&a(&["scratchpad", "read"])), Ok(Cmd::ScratchRead));
        assert_eq!(
            parse(&a(&[
                "scratchpad",
                "append",
                "--if-revision",
                "3",
                "hi",
                "x"
            ])),
            Ok(Cmd::ScratchWrite {
                text: Some("hi x".into()),
                if_revision: Some(3),
                append: true
            })
        );
        assert_eq!(
            parse(&a(&["scratchpad", "write", "-"])),
            Ok(Cmd::ScratchWrite {
                text: None,
                if_revision: None,
                append: false
            })
        );
        assert!(parse(&a(&["scratchpad", "write"])).is_err());
        assert!(parse(&a(&["scratchpad"])).is_err());
        assert_eq!(
            parse(&a(&["frobnicate"])),
            Err("unknown command: frobnicate".into())
        );
    }

    #[test]
    fn hook_context_is_valid_json_with_guide() {
        let v: Value = serde_json::from_str(&hook_context_json()).unwrap();
        let out = &v["hookSpecificOutput"];
        assert_eq!(out["hookEventName"], "SessionStart");
        let ctx = out["additionalContext"].as_str().unwrap();
        assert!(ctx.contains("agentfleet spawn") && ctx.contains("agentfleet send"));
        assert!(!hook_context_json().contains('\n'));
    }

    #[test]
    fn skill_install_targets() {
        let home = Path::new("/home/u");
        let both = skill_targets(home, true);
        assert_eq!(
            both,
            vec![
                PathBuf::from("/home/u/.claude/skills/agentfleet/SKILL.md"),
                PathBuf::from("/home/u/.codex/skills/agentfleet/SKILL.md"),
            ]
        );
        assert_eq!(skill_targets(home, false).len(), 1);
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".codex")).unwrap();
        let written = install_skill(dir.path()).unwrap();
        assert_eq!(written.len(), 2);
        for p in written {
            assert_eq!(std::fs::read_to_string(p).unwrap(), GUIDE);
        }
    }

    #[test]
    fn with_role_prepends() {
        assert_eq!(with_role(Some("be brief\n"), "go"), "be brief\n\ngo");
        assert_eq!(with_role(None, "go"), "go");
    }
}
