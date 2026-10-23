//! Claude Code / Codex launch strings and status-hook settings.
use std::path::Path;

use serde_json::json;
use uuid::Uuid;

pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

pub fn claude_launch(
    session_id: Uuid,
    settings_path: &Path,
    resume: bool,
    auto_permission: bool,
    extra: &[String],
) -> String {
    let id_flag = if resume { "--resume" } else { "--session-id" };
    let perm = if auto_permission {
        "--permission-mode auto "
    } else {
        ""
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

pub fn write_hooks_settings(status_file: &Path, dest: &Path) -> std::io::Result<()> {
    let q = shell_quote(&status_file.to_string_lossy());
    let hook =
        |word: &str| json!([{ "type": "command", "command": format!("printf {word} > {q}") }]);
    let settings = json!({"hooks": {
        "SessionStart": [{"hooks": hook("waiting")}],
        "UserPromptSubmit": [{"hooks": hook("working")}],
        "PreToolUse": [{"matcher": "", "hooks": hook("working")}],
        "PermissionRequest": [{"hooks": hook("waiting")}],
        "Stop": [{"hooks": hook("waiting")}],
    }});
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

pub fn codex_launch(
    script: &Path,
    resume_id: Option<&str>,
    auto_permission: bool,
    extra: &[String],
) -> String {
    let mut s = String::from("codex --dangerously-bypass-hook-trust");
    if auto_permission {
        s.push_str(" --approve-for-me");
    }
    for o in codex_hook_overrides(script) {
        s.push_str(" -c ");
        s.push_str(&shell_quote(&o));
    }
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

/// Codex session id from the last hook payload, if present.
pub fn codex_session_id(payload_file: &Path) -> Option<String> {
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
        let s = claude_launch(id, &p, false, true, &[]);
        assert!(s.starts_with("claude --permission-mode auto --settings "));
        assert!(s.contains(&format!("--session-id {id}")));
        assert!(!s.contains("--resume"));
        let r = claude_launch(id, &p, true, true, &[]);
        assert!(r.contains(&format!("--resume {id}")));
        assert!(!r.contains("--session-id"));
        let n = claude_launch(id, &p, false, false, &[]);
        assert!(n.starts_with("claude --settings "));
        assert!(!n.contains("--permission-mode"));
        assert!(n.ends_with(&id.to_string()));
        let x = claude_launch(id, &p, false, false, &["--model".into(), "o'p us".into()]);
        assert!(x.ends_with(&format!("{id} '--model' 'o'\\''p us'")), "{x}");
    }

    #[test]
    fn settings_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let status = dir.path().join("it's.status");
        let dest = dir.path().join("s.json");
        write_hooks_settings(&status, &dest).unwrap();
        let v: serde_json::Value = serde_json::from_slice(&std::fs::read(&dest).unwrap()).unwrap();
        let quoted = shell_quote(&status.to_string_lossy());
        let hooks = v["hooks"].as_object().unwrap();
        assert_eq!(hooks.len(), 5);
        for (_, groups) in hooks {
            let cmd = groups[0]["hooks"][0]["command"].as_str().unwrap();
            assert!(cmd.contains(&quoted), "{cmd}");
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
    fn codex_launch_flags() {
        let p = PathBuf::from("/h/x.sh");
        let a = codex_launch(&p, None, true, &[]);
        assert!(a.starts_with(
            "codex --dangerously-bypass-hook-trust --approve-for-me -c 'hooks.SessionStart="
        ));
        assert_eq!(a.matches(" -c ").count(), 6);
        assert!(!a.contains("resume"));
        let n = codex_launch(&p, None, false, &[]);
        assert!(n.starts_with("codex --dangerously-bypass-hook-trust -c "));
        assert!(!n.contains("--approve-for-me"));
        let r = codex_launch(&p, Some("abc-123"), false, &["-m".into(), "o'3".into()]);
        assert!(r.ends_with("}]}]' '-m' 'o'\\''3' resume 'abc-123'"), "{r}");
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
        assert_eq!(codex_session_id(&payload), None);
        std::fs::write(
            &payload,
            r#"{"session_id":"019a-x","cwd":"/w","hook_event_name":"Stop"}"#,
        )
        .unwrap();
        assert_eq!(codex_session_id(&payload).as_deref(), Some("019a-x"));
        std::fs::write(&payload, r#"{"session_id":""}"#).unwrap();
        assert_eq!(codex_session_id(&payload), None);
        std::fs::write(&payload, "not json").unwrap();
        assert_eq!(codex_session_id(&payload), None);
    }
}
