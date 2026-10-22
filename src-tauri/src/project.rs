//! Projects: git repo folders that sessions belong to.
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::session::Sessions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
}

pub type Projects = Mutex<Vec<Project>>;

pub fn add_to(list: &mut Vec<Project>, path: &Path) -> Result<Project, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let ok = Command::new("git")
        .current_dir(&path)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !ok {
        return Err("not a git repository".into());
    }
    if let Some(p) = list.iter().find(|p| p.path == path) {
        return Ok(p.clone());
    }
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".into());
    let p = Project {
        id: Uuid::new_v4(),
        name,
        path,
    };
    list.push(p.clone());
    Ok(p)
}

pub fn remove_from(list: &mut Vec<Project>, id: Uuid, has_sessions: bool) -> Result<(), String> {
    if has_sessions {
        return Err("project has open sessions".into());
    }
    list.retain(|p| p.id != id);
    Ok(())
}

#[tauri::command]
pub fn list_projects(projects: State<'_, Projects>) -> Result<Vec<Project>, String> {
    Ok(projects.lock().map_err(|e| e.to_string())?.clone())
}

#[tauri::command]
pub fn add_project(
    path: String,
    projects: State<'_, Projects>,
    sessions: State<'_, Sessions>,
) -> Result<Project, String> {
    // Lock order everywhere: sessions, then projects.
    let map = sessions.lock().map_err(|e| e.to_string())?;
    let mut list = projects.lock().map_err(|e| e.to_string())?;
    let before = list.len();
    let p = add_to(&mut list, Path::new(&path))?;
    if list.len() != before {
        crate::session::persist(&map, &list);
    }
    Ok(p)
}

#[tauri::command]
pub fn remove_project(
    id: String,
    projects: State<'_, Projects>,
    sessions: State<'_, Sessions>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let map = sessions.lock().map_err(|e| e.to_string())?;
    let mut list = projects.lock().map_err(|e| e.to_string())?;
    let has_sessions = map.values().any(|s| s.project_id == uuid);
    remove_from(&mut list, uuid, has_sessions)?;
    crate::session::persist(&map, &list);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo(base: &Path) -> PathBuf {
        let r = base.join("myrepo");
        std::fs::create_dir_all(&r).unwrap();
        assert!(Command::new("git")
            .current_dir(&r)
            .arg("init")
            .output()
            .unwrap()
            .status
            .success());
        r
    }

    #[test]
    fn add_twice_same_id() {
        let tmp = tempfile::tempdir().unwrap();
        let r = repo(tmp.path());
        let mut list = Vec::new();
        let a = add_to(&mut list, &r).unwrap();
        assert_eq!(a.name, "myrepo");
        let b = add_to(&mut list, &r).unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn non_repo_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let mut list = Vec::new();
        assert_eq!(
            add_to(&mut list, tmp.path()).unwrap_err(),
            "not a git repository"
        );
        assert!(list.is_empty());
    }

    #[test]
    fn remove_rules() {
        let tmp = tempfile::tempdir().unwrap();
        let r = repo(tmp.path());
        let mut list = Vec::new();
        let p = add_to(&mut list, &r).unwrap();
        assert!(remove_from(&mut list, p.id, true).is_err());
        assert_eq!(list.len(), 1);
        remove_from(&mut list, p.id, false).unwrap();
        assert!(list.is_empty());
    }
}
