//! Claude Code / Codex / pi / opencode launch strings and status-hook settings.
use std::path::Path;

use serde_json::json;
use uuid::Uuid;

pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Escape `s` as a double-quoted JS string literal (JSON escaping), quotes included.
fn js_string(s: &str) -> String {
    serde_json::Value::String(s.to_string()).to_string()
}

/// Write the agentfleet guide for agents that take it as a file:
/// pi via `--append-system-prompt`, opencode via config `instructions`.
pub fn write_guide_file(dest: &Path) -> std::io::Result<()> {
    std::fs::write(dest, crate::cli::GUIDE)
}

/// How an agent handles tool-call permission prompts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    /// The agent's own default: prompt in the terminal.
    Ask,
    /// Auto-approve through the agent's reviewer; it may still prompt.
    Auto,
    /// Never prompt.
    Bypass,
}

impl Permission {
    pub fn from_settings(auto_permission: bool, bypass_permissions: bool) -> Permission {
        if bypass_permissions {
            Permission::Bypass
        } else if auto_permission {
            Permission::Auto
        } else {
            Permission::Ask
        }
    }
}

pub fn claude_launch(
    session_id: Uuid,
    settings_path: &Path,
    resume: bool,
    permission: Permission,
    extra: &[String],
) -> String {
    let id_flag = if resume { "--resume" } else { "--session-id" };
    let perm = match permission {
        Permission::Ask => "",
        Permission::Auto => "--permission-mode auto ",
        Permission::Bypass => "--dangerously-skip-permissions ",
    };
    let mut s = format!(
        "claude {perm}--settings {} {} {}",
        shell_quote(&settings_path.to_string_lossy()),
        id_flag,
        session_id
    );
    for a in extra {
        s.push(' ');
        s.push_str(&shell_quote(a));
    }
    s
}

/// Claude hook settings. `shim` is the absolute path of the `agentfleet` CLI shim: SessionStart
/// also runs `agentfleet hook-context`, which injects the skill guide into the model context.
/// The `agentfleet` CLI is pre-allowed so replying to a request never needs a permission click.
pub fn write_hooks_settings(status_file: &Path, shim: &Path, dest: &Path) -> std::io::Result<()> {
    let q = shell_quote(&status_file.to_string_lossy());
    let hook = |word: &str| json!({ "type": "command", "command": format!("printf {word} > {q}") });
    let context = json!({
        "type": "command",
        "command": format!("{} hook-context", shell_quote(&shim.to_string_lossy())),
    });
    let settings = json!({
        "permissions": {"allow": ["Bash(agentfleet:*)"]},
        "hooks": {
            "SessionStart": [{"hooks": [hook("waiting"), context]}],
            "UserPromptSubmit": [{"hooks": [hook("working")]}],
            "PreToolUse": [{"matcher": "", "hooks": [hook("working")]}],
            "PermissionRequest": [{"hooks": [hook("waiting")]}],
            "Stop": [{"hooks": [hook("waiting")]}],
        },
    });
    std::fs::write(dest, serde_json::to_vec_pretty(&settings)?)
}

/// Codex hook events and the status word each one writes.
const CODEX_HOOKS: [(&str, &str); 6] = [
    ("SessionStart", "waiting"),
    ("UserPromptSubmit", "working"),
    ("PreToolUse", "working"),
    ("PermissionRequest", "waiting"),
    ("Stop", "waiting"),
    ("Interrupt", "waiting"),
];

/// Write the Codex hook script: `$1` = status word, stdin = hook payload JSON.
/// Prints nothing on stdout.
pub fn write_codex_hook(
    status_file: &Path,
    payload_file: &Path,
    dest: &Path,
) -> std::io::Result<()> {
    let body = format!(
        "#!/bin/sh\n# $1 = status word; stdin = Codex hook payload (JSON with session_id).\ncat > {}\nprintf '%s' \"$1\" > {}\n",
        shell_quote(&payload_file.to_string_lossy()),
        shell_quote(&status_file.to_string_lossy()),
    );
    std::fs::write(dest, body)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dest, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

/// Six `-c` override values (unquoted), one per Codex hook event.
/// Caller guarantees `script` contains no `"` or `\\`.
pub fn codex_hook_overrides(script: &Path) -> Vec<String> {
    let path = script.to_string_lossy();
    CODEX_HOOKS
        .iter()
        .map(|(event, word)| {
            format!("hooks.{event}=[{{hooks=[{{type=\"command\",command=\"{path} {word}\"}}]}}]")
        })
        .collect()
}

/// Escape `s` as a TOML basic (double-quoted, single-line) string, quotes included.
pub fn toml_basic_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\u{:04X}", c as u32))
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Codex launch string. `--no-daemon` is required: attached to the shared daemon the TUI
/// ignores every `-c` override (hooks, instructions). SessionStart hooks only fire on the
/// first turn, so readiness comes from the terminal (see `codex_ready_scan`).
pub fn codex_launch(
    script: &Path,
    resume_id: Option<&str>,
    permission: Permission,
    extra: &[String],
) -> String {
    let mut s = String::from("codex --no-daemon --dangerously-bypass-hook-trust");
    match permission {
        Permission::Ask => {}
        Permission::Auto => s.push_str(" --approve-for-me"),
        Permission::Bypass => s.push_str(" --dangerously-bypass-approvals-and-sandbox"),
    }
    for o in codex_hook_overrides(script) {
        s.push_str(" -c ");
        s.push_str(&shell_quote(&o));
    }
    s.push_str(" -c ");
    s.push_str(&shell_quote(&format!(
        "developer_instructions={}",
        toml_basic_string(crate::cli::GUIDE)
    )));
    for a in extra {
        s.push(' ');
        s.push_str(&shell_quote(a));
    }
    if let Some(id) = resume_id {
        s.push_str(" resume ");
        s.push_str(&shell_quote(id));
    }
    s
}

const OSC_7501_WORKING: &[u8] = b"7501;state=working";
const OSC_7501_IDLE: &[u8] = b"7501;state=idle";

/// Codex TUI readiness from its terminal output: at startup it emits `OSC 7501 state=idle`,
/// then `state=working` while loading, then `state=idle` when the composer is ready.
/// Returns true for the first chunk that completes a working → idle transition.
pub fn codex_ready_scan(seen_working: &mut bool, chunk: &[u8]) -> bool {
    let idle_at = chunk
        .windows(OSC_7501_IDLE.len())
        .rposition(|w| w == OSC_7501_IDLE);
    let working_at = chunk
        .windows(OSC_7501_WORKING.len())
        .rposition(|w| w == OSC_7501_WORKING);
    match (*seen_working, working_at, idle_at) {
        (false, None, _) => false,
        (false, Some(w), Some(i)) if i > w => {
            *seen_working = true;
            true
        }
        (false, Some(_), _) => {
            *seen_working = true;
            false
        }
        (true, _, Some(_)) => true,
        (true, _, None) => false,
    }
}

/// Write the pi status extension: a JS module that mirrors the Claude/Codex status
/// hooks by writing a status word on lifecycle events. `session_start` fires at TUI
/// startup, so it doubles as the readiness signal.
pub fn write_pi_extension(status_file: &Path, dest: &Path) -> std::io::Result<()> {
    let status = js_string(&status_file.to_string_lossy());
    let body = format!(
        r#"import fs from "node:fs";
const STATUS = {status};
const write = (word) => {{ try {{ fs.writeFileSync(STATUS, word); }} catch {{}} }};
export default function (pi) {{
  pi.on("session_start", async () => write("waiting"));
  pi.on("agent_start", async () => write("working"));
  pi.on("agent_end", async () => write("waiting"));
}}
"#
    );
    std::fs::write(dest, body)
}

/// Pi launch string. `--session-id` takes our UUID and creates the session when it
/// does not exist yet, so restore is the same command as the first launch. Pi runs
/// tools without permission prompts; `Permission` does not apply.
pub fn pi_launch(session_id: Uuid, extension: &Path, guide: &Path, extra: &[String]) -> String {
    let mut s = format!(
        "pi --session-id {} -e {} --append-system-prompt {}",
        session_id,
        shell_quote(&extension.to_string_lossy()),
        shell_quote(&guide.to_string_lossy()),
    );
    for a in extra {
        s.push(' ');
        s.push_str(&shell_quote(a));
    }
    s
}

/// Write the opencode status plugin: it tracks the root session (the one without
/// `parentID`), stores its id in `payload_file` for resume, and writes status words
/// (`session.status` busy/idle, permission asked/replied, prompt received).
pub fn write_opencode_plugin(
    status_file: &Path,
    payload_file: &Path,
    dest: &Path,
) -> std::io::Result<()> {
    let status = js_string(&status_file.to_string_lossy());
    let payload = js_string(&payload_file.to_string_lossy());
    let body = format!(
        r#"import fs from "node:fs";
const STATUS = {status};
const PAYLOAD = {payload};
const write = (word) => {{ try {{ fs.writeFileSync(STATUS, word); }} catch {{}} }};
const root = () => {{
  try {{ return JSON.parse(fs.readFileSync(PAYLOAD, "utf8")).session_id; }} catch {{ return undefined; }}
}};
const mine = (id) => {{ const r = root(); return !r || r === id; }};
export const AgentfleetStatus = async () => ({{
  event: async ({{ event }}) => {{
    const p = event.properties;
    switch (event.type) {{
      case "session.created":
        if (p.info && !p.info.parentID && !root()) {{
          try {{ fs.writeFileSync(PAYLOAD, JSON.stringify({{ session_id: p.info.id }})); }} catch {{}}
          write("waiting");
        }}
        break;
      case "session.status":
        if (mine(p.sessionID)) write(p.status && p.status.type === "idle" ? "waiting" : "working");
        break;
      case "session.idle":
      case "permission.updated":
        if (mine(p.sessionID)) write("waiting");
        break;
      case "permission.replied":
        if (mine(p.sessionID)) write("working");
        break;
    }}
  }},
  "chat.message": async (input) => {{ if (mine(input.sessionID)) write("working"); }},
}});
"#
    );
    std::fs::write(dest, body)
}

/// Config overlay passed as `OPENCODE_CONFIG_CONTENT` (merged on top of the user's
/// config): load the status plugin, inject the guide as instructions, and on Bypass
/// allow every permission.
pub fn opencode_config_content(plugin: &Path, guide: &Path, permission: Permission) -> String {
    let mut cfg = json!({
        "plugin": [format!("file://{}", plugin.to_string_lossy())],
        "instructions": [guide.to_string_lossy()],
    });
    if permission == Permission::Bypass {
        cfg["permission"] = json!({
            "edit": "allow",
            "bash": "allow",
            "webfetch": "allow",
            "doom_loop": "allow",
            "external_directory": "allow",
        });
    }
    cfg.to_string()
}

/// opencode launch string. The TUI picks its own session id; the plugin stores it in
/// the payload file, and resume is `--session <id>`. `--auto` auto-approves
/// permissions that are not explicitly denied (opencode's closest analogue of both
/// Auto and Bypass; Bypass additionally allows everything via the config overlay).
pub fn opencode_launch(
    config_content: &str,
    resume_id: Option<&str>,
    permission: Permission,
    extra: &[String],
) -> String {
    let mut s = format!(
        "env OPENCODE_CONFIG_CONTENT={} opencode",
        shell_quote(config_content)
    );
    match permission {
        Permission::Ask => {}
        Permission::Auto | Permission::Bypass => s.push_str(" --auto"),
    }
    if let Some(id) = resume_id {
        s.push_str(" --session ");
        s.push_str(&shell_quote(id));
    }
    for a in extra {
        s.push(' ');
        s.push_str(&shell_quote(a));
    }
    s
}

/// Session id from the last hook/plugin payload (`{"session_id": …}`), if present.
/// Written by the Codex hook script and the opencode plugin.
pub fn payload_session_id(payload_file: &Path) -> Option<String> {
    let bytes = std::fs::read(payload_file).ok()?;
    let v: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    match v.get("session_id")?.as_str() {
        Some(id) if !id.is_empty() => Some(id.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn launch_flags() {
        let id = Uuid::new_v4();
        let p = PathBuf::from("/tmp/x.settings.json");
        let s = claude_launch(id, &p, false, Permission::Auto, &[]);
        assert!(s.starts_with("claude --permission-mode auto --settings "));
        assert!(s.contains(&format!("--session-id {id}")));
        assert!(!s.contains("--resume"));
        let r = claude_launch(id, &p, true, Permission::Auto, &[]);
        assert!(r.contains(&format!("--resume {id}")));
        assert!(!r.contains("--session-id"));
        let n = claude_launch(id, &p, false, Permission::Ask, &[]);
        assert!(n.starts_with("claude --settings "));
        assert!(!n.contains("--permission-mode"));
        assert!(n.ends_with(&id.to_string()));
        let b = claude_launch(id, &p, false, Permission::Bypass, &[]);
        assert!(b.starts_with("claude --dangerously-skip-permissions --settings "));
        assert!(!b.contains("--permission-mode"));
        let x = claude_launch(
            id,
            &p,
            false,
            Permission::Ask,
            &["--model".into(), "o'p us".into()],
        );
        assert!(x.ends_with(&format!("{id} '--model' 'o'\\''p us'")), "{x}");
    }

    #[test]
    fn settings_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let status = dir.path().join("it's.status");
        let dest = dir.path().join("s.json");
        let shim = PathBuf::from("/t/agentfleet-1/agentfleet");
        write_hooks_settings(&status, &shim, &dest).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&std::fs::read(&dest).unwrap()).unwrap();
        let quoted = shell_quote(&status.to_string_lossy());
        assert_eq!(v["permissions"]["allow"][0], "Bash(agentfleet:*)");
        let hooks = v["hooks"].as_object().unwrap();
        assert_eq!(hooks.len(), 5);
        for (event, groups) in hooks {
            let list = groups[0]["hooks"].as_array().unwrap();
            let cmd = list[0]["command"].as_str().unwrap();
            assert!(cmd.contains(&quoted), "{cmd}");
            if event == "SessionStart" {
                assert_eq!(list.len(), 2);
                assert_eq!(
                    list[1]["command"].as_str().unwrap(),
                    "'/t/agentfleet-1/agentfleet' hook-context"
                );
            } else {
                assert_eq!(list.len(), 1, "{event}");
            }
        }
    }

    #[test]
    fn codex_overrides() {
        let o = codex_hook_overrides(&PathBuf::from("/h/x.codex-hook.sh"));
        assert_eq!(o.len(), 6);
        for (event, word) in CODEX_HOOKS {
            let want = format!(
                "hooks.{event}=[{{hooks=[{{type=\"command\",command=\"/h/x.codex-hook.sh {word}\"}}]}}]"
            );
            assert!(o.contains(&want), "{want}");
        }
        assert!(o[1].contains("UserPromptSubmit") && o[1].ends_with("working\"}]}]"));
        assert!(o[5].contains("Interrupt") && o[5].ends_with("waiting\"}]}]"));
    }

    #[test]
    fn toml_strings() {
        assert_eq!(toml_basic_string("plain"), "\"plain\"");
        assert_eq!(
            toml_basic_string("a \"q\" b\\c\nd\te"),
            "\"a \\\"q\\\" b\\\\c\\nd\\te\""
        );
        assert_eq!(toml_basic_string("\u{1}"), "\"\\u0001\"");
        // The guide itself must come out as one line with no raw quotes left unescaped.
        let g = toml_basic_string(crate::cli::GUIDE);
        assert!(!g.contains('\n'));
        assert!(g.starts_with('"') && g.ends_with('"'));
        let v: toml_check::Value = toml_check::parse(&g);
        assert_eq!(v.0, crate::cli::GUIDE);
    }

    /// Minimal TOML basic-string decoder, enough to round-trip `toml_basic_string` in tests.
    mod toml_check {
        pub struct Value(pub String);
        pub fn parse(s: &str) -> Value {
            let inner = &s[1..s.len() - 1];
            let mut out = String::new();
            let mut it = inner.chars();
            while let Some(c) = it.next() {
                if c != '\\' {
                    out.push(c);
                    continue;
                }
                match it.next().unwrap() {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    'u' => {
                        let hex: String = (0..4).map(|_| it.next().unwrap()).collect();
                        out.push(char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap());
                    }
                    other => panic!("unexpected escape \\{other}"),
                }
            }
            Value(out)
        }
    }

    #[test]
    fn codex_launch_flags() {
        let p = PathBuf::from("/h/x.sh");
        let a = codex_launch(&p, None, Permission::Auto, &[]);
        assert!(a.starts_with(
            "codex --no-daemon --dangerously-bypass-hook-trust --approve-for-me -c 'hooks.SessionStart="
        ));
        assert_eq!(a.matches(" -c 'hooks.").count(), 6);
        assert_eq!(a.matches(" -c 'developer_instructions=").count(), 1);
        assert!(a.contains(" -c 'developer_instructions=\"---"));
        assert!(!a.contains("resume"));
        let n = codex_launch(&p, None, Permission::Ask, &[]);
        assert!(n.starts_with("codex --no-daemon --dangerously-bypass-hook-trust -c "));
        assert!(!n.contains("--approve-for-me"));
        let b = codex_launch(&p, None, Permission::Bypass, &[]);
        assert!(b.starts_with(
            "codex --no-daemon --dangerously-bypass-hook-trust --dangerously-bypass-approvals-and-sandbox -c "
        ));
        assert!(!b.contains("--approve-for-me"));
        let r = codex_launch(
            &p,
            Some("abc-123"),
            Permission::Ask,
            &["-m".into(), "o'3".into()],
        );
        assert!(r.ends_with("' '-m' 'o'\\''3' resume 'abc-123'"), "{r}");
    }

    #[test]
    fn permission_from_settings() {
        assert_eq!(Permission::from_settings(false, false), Permission::Ask);
        assert_eq!(Permission::from_settings(true, false), Permission::Auto);
        assert_eq!(Permission::from_settings(true, true), Permission::Bypass);
        // Bypass wins even if auto was switched off.
        assert_eq!(Permission::from_settings(false, true), Permission::Bypass);
    }

    #[test]
    fn codex_ready_from_osc_7501() {
        let mut w = false;
        // Observed startup sequence: idle, working, working, idle (ready).
        assert!(!codex_ready_scan(
            &mut w,
            b"\x1b]7501;state=idle:app=codex\x1b\\"
        ));
        assert!(!codex_ready_scan(
            &mut w,
            b"\x1b]7501;state=working:app=codex\x1b\\"
        ));
        assert!(!codex_ready_scan(
            &mut w,
            b"text \x1b]7501;state=working:app=codex\x1b\\ more"
        ));
        assert!(codex_ready_scan(
            &mut w,
            b"\x1b]7501;state=idle:app=codex\x1b\\"
        ));
        // Working and idle in one chunk, in that order, is ready too.
        let mut w2 = false;
        assert!(codex_ready_scan(
            &mut w2,
            b"\x1b]7501;state=working:app=codex\x1b\\..\x1b]7501;state=idle:app=codex\x1b\\"
        ));
        // Idle before working in the same chunk is not ready yet.
        let mut w3 = false;
        assert!(!codex_ready_scan(
            &mut w3,
            b"\x1b]7501;state=idle:app=codex\x1b\\..\x1b]7501;state=working:app=codex\x1b\\"
        ));
        assert!(w3);
        assert!(!codex_ready_scan(&mut w3, b"plain output"));
    }

    #[test]
    fn codex_hook_and_session_id() {
        let dir = tempfile::tempdir().unwrap();
        let status = dir.path().join("s.status");
        let payload = dir.path().join("p.json");
        let script = dir.path().join("h.sh");
        write_codex_hook(&status, &payload, &script).unwrap();
        let body = std::fs::read_to_string(&script).unwrap();
        assert!(body.starts_with("#!/bin/sh\n"));
        assert!(body.contains(&shell_quote(&status.to_string_lossy())));
        assert!(body.contains(&shell_quote(&payload.to_string_lossy())));
        assert_eq!(payload_session_id(&payload), None);
        std::fs::write(
            &payload,
            r#"{"session_id":"019a-x","cwd":"/w","hook_event_name":"Stop"}"#,
        )
        .unwrap();
        assert_eq!(payload_session_id(&payload).as_deref(), Some("019a-x"));
        std::fs::write(&payload, r#"{"session_id":""}"#).unwrap();
        assert_eq!(payload_session_id(&payload), None);
        std::fs::write(&payload, "not json").unwrap();
        assert_eq!(payload_session_id(&payload), None);
    }

    #[test]
    fn pi_launch_flags() {
        let id = Uuid::new_v4();
        let ext = PathBuf::from("/h/x.pi-ext.js");
        let guide = PathBuf::from("/h/x.guide.md");
        let s = pi_launch(id, &ext, &guide, &[]);
        assert_eq!(
            s,
            format!(
                "pi --session-id {id} -e '/h/x.pi-ext.js' --append-system-prompt '/h/x.guide.md'"
            )
        );
        let x = pi_launch(id, &ext, &guide, &["--model".into(), "o'p".into()]);
        assert!(x.ends_with("'--model' 'o'\\''p'"), "{x}");
    }

    #[test]
    fn pi_extension_contents() {
        let dir = tempfile::tempdir().unwrap();
        let status = dir.path().join("it's \"q\".status");
        let dest = dir.path().join("ext.js");
        write_pi_extension(&status, &dest).unwrap();
        let body = std::fs::read_to_string(&dest).unwrap();
        assert!(
            body.contains(&js_string(&status.to_string_lossy())),
            "{body}"
        );
        for ev in ["session_start", "agent_start", "agent_end"] {
            assert!(body.contains(ev), "{ev}");
        }
        assert!(body.contains(r#"write("working")"#));
        assert!(body.contains(r#"write("waiting")"#));
    }

    #[test]
    fn opencode_plugin_contents() {
        let dir = tempfile::tempdir().unwrap();
        let status = dir.path().join("s.status");
        let payload = dir.path().join("p.json");
        let dest = dir.path().join("plugin.js");
        write_opencode_plugin(&status, &payload, &dest).unwrap();
        let body = std::fs::read_to_string(&dest).unwrap();
        assert!(body.contains(&js_string(&status.to_string_lossy())));
        assert!(body.contains(&js_string(&payload.to_string_lossy())));
        for ev in [
            "session.created",
            "session.status",
            "session.idle",
            "permission.updated",
            "permission.replied",
            "chat.message",
        ] {
            assert!(body.contains(ev), "{ev}");
        }
        assert!(body.contains("session_id"));
    }

    #[test]
    fn opencode_config_overlay() {
        let plugin = PathBuf::from("/h/x.opencode-plugin.js");
        let guide = PathBuf::from("/h/x.guide.md");
        let c = opencode_config_content(&plugin, &guide, Permission::Ask);
        let v: serde_json::Value = serde_json::from_str(&c).unwrap();
        assert_eq!(v["plugin"][0], "file:///h/x.opencode-plugin.js");
        assert_eq!(v["instructions"][0], "/h/x.guide.md");
        assert!(v.get("permission").is_none());
        let a: serde_json::Value =
            serde_json::from_str(&opencode_config_content(&plugin, &guide, Permission::Auto))
                .unwrap();
        assert!(a.get("permission").is_none());
        let b: serde_json::Value = serde_json::from_str(&opencode_config_content(
            &plugin,
            &guide,
            Permission::Bypass,
        ))
        .unwrap();
        for key in [
            "edit",
            "bash",
            "webfetch",
            "doom_loop",
            "external_directory",
        ] {
            assert_eq!(b["permission"][key], "allow", "{key}");
        }
    }

    #[test]
    fn opencode_launch_flags() {
        let cfg = r#"{"plugin":["file:///h/p.js"]}"#;
        let n = opencode_launch(cfg, None, Permission::Ask, &[]);
        assert_eq!(
            n,
            format!("env OPENCODE_CONFIG_CONTENT={} opencode", shell_quote(cfg))
        );
        let a = opencode_launch(cfg, None, Permission::Auto, &[]);
        assert!(a.ends_with("opencode --auto"), "{a}");
        let b = opencode_launch(cfg, None, Permission::Bypass, &[]);
        assert!(b.ends_with("opencode --auto"), "{b}");
        let r = opencode_launch(
            cfg,
            Some("ses_abc"),
            Permission::Ask,
            &["-m".into(), "o'3".into()],
        );
        assert!(
            r.ends_with("opencode --session 'ses_abc' '-m' 'o'\\''3'"),
            "{r}"
        );
    }
}
