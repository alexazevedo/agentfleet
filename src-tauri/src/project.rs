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

/// Error returned by `add_project` for a folder without a repo. The frontend
/// matches it verbatim to offer `init_project` instead of showing an error.
pub const NOT_A_REPO: &str = "not a git repository";

fn git_ok(dir: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn git(dir: &Path, args: &[&str]) -> Result<(), String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

/// `git init` plus an empty first commit: sessions branch a worktree off HEAD,
/// so a bare `git init` would still fail with "repository has no commits".
pub fn init_repo(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err("not a directory".into());
    }
    git(path, &["init", "-q"])?;
    if git_ok(path, &["rev-parse", "--verify", "-q", "HEAD"]) {
        return Ok(());
    }
    let commit = ["commit", "-q", "--allow-empty", "-m", "Initial commit"];
    if git(path, &commit).is_ok() {
        return Ok(());
    }
    // No user.name / user.email configured: commit under an app identity.
    let mut with_identity = vec![
        "-c",
        "user.name=agentfleet",
        "-c",
        "user.email=agentfleet@localhost",
    ];
    with_identity.extend_from_slice(&commit);
    git(path, &with_identity)
}

pub fn add_to(list: &mut Vec<Project>, path: &Path) -> Result<Project, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    if !git_ok(&path, &["rev-parse", "--show-toplevel"]) {
        return Err(NOT_A_REPO.into());
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

fn register(path: &str, projects: &Projects, sessions: &Sessions) -> Result<Project, String> {
    // Lock order everywhere: sessions, then projects.
    let map = sessions.lock().map_err(|e| e.to_string())?;
    let mut list = projects.lock().map_err(|e| e.to_string())?;
    let before = list.len();
    let p = add_to(&mut list, Path::new(path))?;
    if list.len() != before {
        crate::session::persist(&map, &list);
    }
    Ok(p)
}

#[tauri::command]
pub fn add_project(
    path: String,
    projects: State<'_, Projects>,
    sessions: State<'_, Sessions>,
) -> Result<Project, String> {
    register(&path, &projects, &sessions)
}

/// Turns a plain folder into a repo (see `init_repo`), then adds it.
#[tauri::command]
pub fn init_project(
    path: String,
    projects: State<'_, Projects>,
    sessions: State<'_, Sessions>,
) -> Result<Project, String> {
    init_repo(Path::new(&path))?;
    register(&path, &projects, &sessions)
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
    fn init_repo_makes_addable_with_head() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("fresh");
        std::fs::create_dir_all(&dir).unwrap();
        let mut list = Vec::new();
        assert_eq!(add_to(&mut list, &dir).unwrap_err(), NOT_A_REPO);
        init_repo(&dir).unwrap();
        // Worktrees branch from HEAD, so init must leave a commit behind.
        assert!(git_ok(&dir, &["rev-parse", "--verify", "-q", "HEAD"]));
        assert_eq!(add_to(&mut list, &dir).unwrap().name, "fresh");
        // Idempotent: a second init neither fails nor adds a commit.
        init_repo(&dir).unwrap();
        let count = Command::new("git")
            .current_dir(&dir)
            .args(["rev-list", "--count", "HEAD"])
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&count.stdout).trim(), "1");
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
