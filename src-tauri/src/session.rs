//! Session registry and Tauri commands: worktree + PTY per tab.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serde::Serialize;
use tauri::ipc::{Channel, InvokeResponseBody};
use tauri::{AppHandle, State};
use tokio::sync::watch;
use uuid::Uuid;

use crate::agent;
use crate::project::{Project, Projects};
use crate::state::{self, PersistedSession, PersistedState};
use crate::status::{self, Status};
use crate::worktree::{self, Worktree, WorktreeError};
use crate::{platform, pty};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Agent {
    Claude,
    Shell,
}

impl Agent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Agent::Claude => "claude",
            Agent::Shell => "shell",
        }
    }

    fn parse(s: &str) -> Result<Agent, String> {
        match s {
            "claude" => Ok(Agent::Claude),
            "shell" => Ok(Agent::Shell),
            other => Err(format!("unknown agent: {other}")),
        }
    }
}

pub struct Session {
    pub id: Uuid,
    pub id8: String,
    pub project_id: Uuid,
    pub agent: Agent,
    pub repo: PathBuf,
    pub worktree: Worktree,
    pub pty: Option<pty::PtyHandle>,
    /// Send `true` to stop the status poller (Claude sessions only).
    pub poller_stop: Option<watch::Sender<bool>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub id: String,
    pub id8: String,
    pub project_id: String,
    pub agent: String,
    pub branch: String,
    pub worktree_path: String,
    pub repo_name: String,
}

pub type Sessions = Mutex<HashMap<Uuid, Session>>;

pub fn worktree_error_message(e: &WorktreeError) -> String {
    match e {
        WorktreeError::NotARepo => "not a git repository".to_string(),
        WorktreeError::NoCommits => "repository has no commits".to_string(),
        WorktreeError::BranchExists(b) => format!("branch {b} already exists"),
        WorktreeError::Git(m) => m.clone(),
        WorktreeError::Io(e) => e.to_string(),
    }
}

// Lock order everywhere: sessions, then projects.
#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub fn create_session(
    project_id: String,
    agent: String,
    cols: u16,
    rows: u16,
    on_data: Channel<InvokeResponseBody>,
    sessions: State<'_, Sessions>,
    projects: State<'_, Projects>,
    app: AppHandle,
) -> Result<SessionInfo, String> {
    let agent = Agent::parse(&agent)?;
    let project_id = Uuid::parse_str(&project_id).map_err(|_| "unknown project".to_string())?;
    let repo = projects
        .lock()
        .map_err(|e| e.to_string())?
        .iter()
        .find(|p| p.id == project_id)
        .map(|p| p.path.clone())
        .ok_or("unknown project")?;
    let id = Uuid::new_v4();
    let id8: String = id.simple().to_string().chars().take(8).collect();

    let root = platform::home_dir().join(".agentfleet/worktrees");
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let wt = worktree::create(&repo, agent.as_str(), &id8, &root)
        .map_err(|e| worktree_error_message(&e))?;

    let mut session = Session {
        id,
        id8: id8.clone(),
        project_id,
        agent,
        repo: repo.clone(),
        worktree: wt.clone(),
        pty: None,
        poller_stop: None,
    };
    spawn_for(&mut session, &app, cols, rows, on_data, false)?;

    let repo_name = repo
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".into());
    let info = SessionInfo {
        id: id.to_string(),
        id8: id8.clone(),
        project_id: project_id.to_string(),
        agent: agent.as_str().to_string(),
        branch: wt.branch.clone(),
        worktree_path: wt.path.to_string_lossy().to_string(),
        repo_name,
    };
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    map.insert(id, session);
    persist(&map, &projects.lock().map_err(|e| e.to_string())?);
    Ok(info)
}

/// Spawn the PTY (and status poller for Claude) for an existing session.
fn spawn_for(
    session: &mut Session,
    app: &AppHandle,
    cols: u16,
    rows: u16,
    on_data: Channel<InvokeResponseBody>,
    resume: bool,
) -> Result<(), String> {
    let id = session.id;
    let (stop_tx, stop_rx) = watch::channel(false);
    let mut command: Option<String> = None;
    let mut poller_stop = None;
    if session.agent == Agent::Claude {
        let run = platform::home_dir().join(".agentfleet/run");
        std::fs::create_dir_all(&run).map_err(|e| e.to_string())?;
        let status_file = run.join(format!("{id}.status"));
        let settings = run.join(format!("{id}.settings.json"));
        agent::write_hooks_settings(&status_file, &settings).map_err(|e| e.to_string())?;
        command = Some(agent::claude_launch(id, &settings, resume));
        status::spawn_poller(app.clone(), id, status_file, stop_rx);
        poller_stop = Some(stop_tx.clone());
    }
    let exit_app = app.clone();
    let handle = pty::spawn(
        &platform::login_shell(),
        &session.worktree.path,
        command.as_deref(),
        &[],
        cols,
        rows,
        Box::new(move |chunk| {
            let _ = on_data.send(InvokeResponseBody::Raw(chunk));
        }),
        Box::new(move |_code| {
            // Stop the poller first so it cannot overwrite `ended`.
            let _ = stop_tx.send(true);
            status::emit(&exit_app, id, Status::Ended);
        }),
    )
    .map_err(|e| e.to_string())?;
    session.pty = Some(handle);
    session.poller_stop = poller_stop;
    Ok(())
}

fn info_for(s: &Session) -> SessionInfo {
    SessionInfo {
        id: s.id.to_string(),
        id8: s.id8.clone(),
        project_id: s.project_id.to_string(),
        agent: s.agent.as_str().to_string(),
        branch: s.worktree.branch.clone(),
        worktree_path: s.worktree.path.to_string_lossy().to_string(),
        repo_name: s
            .repo
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "repo".into()),
    }
}

/// Write the registry to state.json; errors are logged, never fatal.
pub fn persist(sessions: &HashMap<Uuid, Session>, projects: &[Project]) {
    let mut list: Vec<PersistedSession> = sessions
        .values()
        .map(|s| PersistedSession {
            id: s.id,
            id8: s.id8.clone(),
            project_id: s.project_id,
            agent: s.agent.as_str().to_string(),
            repo: s.repo.clone(),
            worktree_path: s.worktree.path.clone(),
            branch: s.worktree.branch.clone(),
            base_commit: s.worktree.base_commit.clone(),
        })
        .collect();
    list.sort_by(|a, b| a.id8.cmp(&b.id8));
    let st = PersistedState {
        version: 1,
        projects: projects.to_vec(),
        sessions: list,
    };
    if let Err(e) = state::save(&st, &state::state_path()) {
        eprintln!("agentfleet: failed to save state: {e}");
    }
}

static LOADED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub fn list_restorable(
    sessions: State<'_, Sessions>,
    projects: State<'_, Projects>,
) -> Result<Vec<SessionInfo>, String> {
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    let mut plist = projects.lock().map_err(|e| e.to_string())?;
    if !LOADED.swap(true, Ordering::SeqCst) {
        let st = match state::load(&state::state_path()) {
            Ok(st) => st,
            Err(e) => {
                eprintln!("agentfleet: ignoring unreadable state: {e}");
                state::PersistedState::empty()
            }
        };
        // Shortcut: a state file lacking `projects` fails to parse and is treated as empty.
        *plist = st.projects;
        let mut dropped = false;
        for p in st.sessions {
            if !plist.iter().any(|pr| pr.id == p.project_id) {
                dropped = true;
                continue;
            }
            let agent = match Agent::parse(&p.agent) {
                Ok(a) => a,
                Err(_) => {
                    dropped = true;
                    continue;
                }
            };
            let wt = Worktree {
                repo: p.repo.clone(),
                path: p.worktree_path,
                branch: p.branch,
                base_commit: p.base_commit,
            };
            if !worktree::exists(&wt) {
                dropped = true;
                continue;
            }
            map.insert(
                p.id,
                Session {
                    id: p.id,
                    id8: p.id8,
                    project_id: p.project_id,
                    agent,
                    repo: p.repo,
                    worktree: wt,
                    pty: None,
                    poller_stop: None,
                },
            );
        }
        if dropped {
            persist(&map, &plist);
        }
    }
    let mut out: Vec<SessionInfo> = map
        .values()
        .filter(|s| s.pty.is_none())
        .map(info_for)
        .collect();
    out.sort_by(|a, b| a.id8.cmp(&b.id8));
    Ok(out)
}

#[tauri::command]
pub fn attach_session(
    id: String,
    cols: u16,
    rows: u16,
    on_data: Channel<InvokeResponseBody>,
    app: AppHandle,
    sessions: State<'_, Sessions>,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    let s = map.get_mut(&uuid).ok_or("no such session")?;
    if s.pty.is_some() {
        return Err("session already attached".into());
    }
    spawn_for(s, &app, cols, rows, on_data, true)
}

fn with_pty<F>(sessions: &Sessions, id: &str, f: F) -> Result<(), String>
where
    F: FnOnce(&pty::PtyHandle) -> anyhow::Result<()>,
{
    let id = Uuid::parse_str(id).map_err(|e| e.to_string())?;
    let map = sessions.lock().map_err(|e| e.to_string())?;
    let s = map.get(&id).ok_or("no such session")?;
    let p = s.pty.as_ref().ok_or("session has no pty")?;
    f(p).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_session(
    id: String,
    data: Vec<u8>,
    sessions: State<'_, Sessions>,
) -> Result<(), String> {
    with_pty(&sessions, &id, |p| p.write(&data))
}

#[tauri::command]
pub fn resize_session(
    id: String,
    cols: u16,
    rows: u16,
    sessions: State<'_, Sessions>,
) -> Result<(), String> {
    with_pty(&sessions, &id, |p| p.resize(cols, rows))
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum CloseOutcome {
    Removed,
    Kept { path: String },
}

/// Pure decision: remove only when clean and the removal succeeded.
/// `remove_result` is `None` when removal was not attempted (dirty tree).
fn close_decision(
    path: &str,
    clean: bool,
    remove_result: Option<Result<(), String>>,
) -> CloseOutcome {
    match (clean, remove_result) {
        (true, Some(Ok(()))) => CloseOutcome::Removed,
        (true, Some(Err(e))) => CloseOutcome::Kept {
            path: format!("{path} \u{2014} {e}"),
        },
        _ => CloseOutcome::Kept { path: path.into() },
    }
}

#[tauri::command]
pub fn close_session(
    id: String,
    sessions: State<'_, Sessions>,
    projects: State<'_, Projects>,
) -> Result<CloseOutcome, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    // Take ownership so the mutex is not held across kill (which joins the reader).
    let mut s = {
        let mut map = sessions.lock().map_err(|e| e.to_string())?;
        let s = map.remove(&uuid).ok_or("no such session")?;
        persist(&map, &projects.lock().map_err(|e| e.to_string())?);
        s
    };
    if let Some(tx) = &s.poller_stop {
        let _ = tx.send(true);
    }
    if let Some(mut p) = s.pty.take() {
        let _ = p.kill();
    }
    let run = platform::home_dir().join(".agentfleet/run");
    for ext in ["status", "settings.json"] {
        let _ = std::fs::remove_file(run.join(format!("{uuid}.{ext}")));
    }
    let path = s.worktree.path.to_string_lossy().to_string();
    let outcome = match worktree::is_clean(&s.worktree.path) {
        Ok(true) => {
            let r = worktree::remove(&s.worktree).map_err(|e| worktree_error_message(&e));
            close_decision(&path, true, Some(r))
        }
        Ok(false) => close_decision(&path, false, None),
        Err(e) => CloseOutcome::Kept {
            path: format!("{path} \u{2014} {}", worktree_error_message(&e)),
        },
    };
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_decisions() {
        assert_eq!(
            close_decision("/p", true, Some(Ok(()))),
            CloseOutcome::Removed
        );
        assert_eq!(
            close_decision("/p", false, None),
            CloseOutcome::Kept { path: "/p".into() }
        );
        assert_eq!(
            close_decision("/p", true, Some(Err("boom".into()))),
            CloseOutcome::Kept {
                path: "/p \u{2014} boom".into()
            }
        );
    }

    #[test]
    fn maps_worktree_errors() {
        assert_eq!(
            worktree_error_message(&WorktreeError::NotARepo),
            "not a git repository"
        );
        assert_eq!(
            worktree_error_message(&WorktreeError::NoCommits),
            "repository has no commits"
        );
        assert_eq!(
            worktree_error_message(&WorktreeError::BranchExists("b".into())),
            "branch b already exists"
        );
        assert_eq!(
            worktree_error_message(&WorktreeError::Git("boom".into())),
            "boom"
        );
    }
}
