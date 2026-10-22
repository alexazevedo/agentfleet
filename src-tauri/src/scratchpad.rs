//! Per-project Markdown scratchpad with a revision counter (optimistic locking).
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::platform;

const MAX_BYTES: usize = 256 * 1024;

/// Process-wide write lock so concurrent CLI + UI writes serialise.
static WRITE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Scratchpad {
    pub revision: u64,
    pub text: String,
}

fn dir() -> PathBuf {
    platform::home_dir().join(".agentfleet/scratchpads")
}

pub fn read_in(dir: &Path, project: Uuid) -> io::Result<Scratchpad> {
    match fs::read_to_string(dir.join(format!("{project}.json"))) {
        Ok(t) => serde_json::from_str(&t).map_err(io::Error::other),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Scratchpad::default()),
        Err(e) => Err(e),
    }
}

fn store(dir: &Path, project: Uuid, pad: &Scratchpad) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let target = dir.join(format!("{project}.json"));
    let tmp = dir.join(format!("{project}.json.tmp"));
    fs::write(&tmp, serde_json::to_string(pad).map_err(io::Error::other)?)?;
    fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))?;
    fs::rename(&tmp, target)
}

fn update(
    dir: &Path,
    project: Uuid,
    if_revision: Option<u64>,
    f: impl FnOnce(&str) -> String,
) -> Result<u64, String> {
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let cur = read_in(dir, project).map_err(|e| e.to_string())?;
    if let Some(r) = if_revision {
        if r != cur.revision {
            return Err(format!("revision mismatch: current {}", cur.revision));
        }
    }
    let text = f(&cur.text);
    if text.len() > MAX_BYTES {
        return Err("scratchpad exceeds 256 KB".into());
    }
    let pad = Scratchpad {
        revision: cur.revision + 1,
        text,
    };
    store(dir, project, &pad).map_err(|e| e.to_string())?;
    Ok(pad.revision)
}

pub fn write_in(
    dir: &Path,
    project: Uuid,
    text: &str,
    if_revision: Option<u64>,
) -> Result<u64, String> {
    update(dir, project, if_revision, |_| text.to_string())
}

pub fn append_in(
    dir: &Path,
    project: Uuid,
    text: &str,
    if_revision: Option<u64>,
) -> Result<u64, String> {
    update(dir, project, if_revision, |old| {
        if old.is_empty() {
            text.to_string()
        } else {
            format!("{old}\n{text}")
        }
    })
}

pub fn read(project: Uuid) -> io::Result<Scratchpad> {
    read_in(&dir(), project)
}

pub fn write(project: Uuid, text: &str, if_revision: Option<u64>) -> Result<u64, String> {
    write_in(&dir(), project, text, if_revision)
}

pub fn append(project: Uuid, text: &str, if_revision: Option<u64>) -> Result<u64, String> {
    append_in(&dir(), project, text, if_revision)
}

pub fn emit_changed(app: &AppHandle, project: Uuid, revision: u64) {
    let _ = app.emit(
        "scratchpad-changed",
        serde_json::json!({ "projectId": project, "revision": revision }),
    );
}

fn parse_id(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_scratchpad(project_id: String) -> Result<Scratchpad, String> {
    read(parse_id(&project_id)?).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_scratchpad(
    project_id: String,
    text: String,
    if_revision: Option<u64>,
    app: AppHandle,
) -> Result<u64, String> {
    let id = parse_id(&project_id)?;
    let rev = write(id, &text, if_revision)?;
    emit_changed(&app, id, rev);
    Ok(rev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_is_empty() {
        let d = tempfile::tempdir().unwrap();
        let p = read_in(d.path(), Uuid::new_v4()).unwrap();
        assert_eq!((p.revision, p.text.as_str()), (0, ""));
    }

    #[test]
    fn write_bumps_and_locks() {
        let d = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        assert_eq!(write_in(d.path(), id, "a", None), Ok(1));
        assert_eq!(read_in(d.path(), id).unwrap().text, "a");
        let mode = fs::metadata(d.path().join(format!("{id}.json")))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(
            write_in(d.path(), id, "b", Some(0)),
            Err("revision mismatch: current 1".into())
        );
        assert_eq!(write_in(d.path(), id, "b", Some(1)), Ok(2));
    }

    #[test]
    fn append_separator_only_when_non_empty() {
        let d = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        append_in(d.path(), id, "one", None).unwrap();
        assert_eq!(read_in(d.path(), id).unwrap().text, "one");
        append_in(d.path(), id, "two", Some(1)).unwrap();
        assert_eq!(read_in(d.path(), id).unwrap().text, "one\ntwo");
    }

    #[test]
    fn cap_enforced() {
        let d = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        let big = "x".repeat(MAX_BYTES + 1);
        assert_eq!(
            write_in(d.path(), id, &big, None),
            Err("scratchpad exceeds 256 KB".into())
        );
        assert!(write_in(d.path(), id, &"x".repeat(MAX_BYTES), None).is_ok());
    }
}
