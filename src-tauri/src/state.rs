//! Persisted session state at ~/.agentfleet/state.json.
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::platform;
use crate::project::Project;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedSession {
    pub id: Uuid,
    pub id8: String,
    pub project_id: Uuid,
    pub agent: String,
    pub repo: PathBuf,
    pub worktree_path: PathBuf,
    pub branch: String,
    pub base_commit: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub agent_args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedState {
    pub version: u32,
    pub projects: Vec<Project>,
    pub sessions: Vec<PersistedSession>,
}

impl PersistedState {
    pub fn empty() -> Self {
        PersistedState {
            version: 1,
            projects: Vec::new(),
            sessions: Vec::new(),
        }
    }
}

pub fn state_path() -> PathBuf {
    platform::home_dir().join(".agentfleet/state.json")
}

pub fn save(state: &PersistedState, path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let json = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
    fs::write(&tmp, json)?;
    fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))?;
    fs::rename(&tmp, path)
}

pub fn load(path: &Path) -> Result<PersistedState, String> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(PersistedState::empty()),
        Err(e) => return Err(e.to_string()),
    };
    let st: PersistedState = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    if st.version != 1 {
        return Err(format!("unsupported state version {}", st.version));
    }
    Ok(st)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> PersistedState {
        let project = Project {
            id: Uuid::new_v4(),
            name: "r".into(),
            path: "/r".into(),
        };
        PersistedState {
            version: 1,
            sessions: vec![PersistedSession {
                project_id: project.id,
                id: Uuid::new_v4(),
                id8: "abcd1234".into(),
                agent: "claude".into(),
                repo: "/r".into(),
                worktree_path: "/w".into(),
                branch: "agentfleet/claude-abcd1234".into(),
                base_commit: "deadbeef".into(),
                name: "label".into(),
                agent_args: vec!["--x".into(), "a b".into()],
            }],
            projects: vec![project],
        }
    }

    #[test]
    fn round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("sub/state.json");
        let s = sample();
        save(&s, &p).unwrap();
        assert_eq!(load(&p).unwrap(), s);
        let mode = fs::metadata(&p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn old_rows_default_name_and_args() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("state.json");
        let pid = Uuid::new_v4();
        let row = format!(
            r#"{{"id":"{}","id8":"abcd1234","project_id":"{pid}","agent":"claude","repo":"/r","worktree_path":"/w","branch":"b","base_commit":"c"}}"#,
            Uuid::new_v4()
        );
        let json = format!(r#"{{"version":1,"projects":[],"sessions":[{row}]}}"#);
        fs::write(&p, json).unwrap();
        let st = load(&p).unwrap();
        assert_eq!(st.sessions[0].name, "");
        assert!(st.sessions[0].agent_args.is_empty());
    }

    #[test]
    fn missing_file_is_empty() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(
            load(&tmp.path().join("nope.json")).unwrap(),
            PersistedState::empty()
        );
    }

    #[test]
    fn bad_json_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("state.json");
        fs::write(&p, "{not json").unwrap();
        assert!(load(&p).is_err());
    }

    #[test]
    fn wrong_version_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("state.json");
        fs::write(&p, r#"{"version":2,"projects":[],"sessions":[]}"#).unwrap();
        assert!(load(&p).is_err());
    }
}
