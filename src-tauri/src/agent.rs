//! Claude Code launch string and status-hook settings.
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
) -> String {
    let id_flag = if resume { "--resume" } else { "--session-id" };
    let perm = if auto_permission {
        "--permission-mode auto "
    } else {
        ""
    };
    format!(
        "claude {perm}--settings {} {} {}",
        shell_quote(&settings_path.to_string_lossy()),
        id_flag,
        session_id
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn launch_flags() {
        let id = Uuid::new_v4();
        let p = PathBuf::from("/tmp/x.settings.json");
        let s = claude_launch(id, &p, false, true);
        assert!(s.starts_with("claude --permission-mode auto --settings "));
        assert!(s.contains(&format!("--session-id {id}")));
        assert!(!s.contains("--resume"));
        let r = claude_launch(id, &p, true, true);
        assert!(r.contains(&format!("--resume {id}")));
        assert!(!r.contains("--session-id"));
        let n = claude_launch(id, &p, false, false);
        assert!(n.starts_with("claude --settings "));
        assert!(!n.contains("--permission-mode"));
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
}
