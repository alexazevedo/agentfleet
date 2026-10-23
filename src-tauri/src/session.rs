//! Session registry and Tauri commands: worktree + PTY per tab.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use serde::Serialize;
use tauri::ipc::{Channel, InvokeResponseBody};
use tauri::{AppHandle, Manager, State};
use tokio::sync::watch;
use uuid::Uuid;

use crate::agent;
use crate::collab::{self, CollabNotify, CollabState};
use crate::project::{self, Project, Projects};
use crate::runtime::Runtime;
use crate::settings::SettingsState;
use crate::state::{self, PersistedSession, PersistedState};
use crate::status::{self, Status};
use crate::worktree::{self, Worktree, WorktreeError};
use crate::{platform, pty};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Agent {
    Claude,
    Codex,
    Shell,
}

impl Agent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Agent::Claude => "claude",
            Agent::Codex => "codex",
            Agent::Shell => "shell",
        }
    }

    pub fn parse(s: &str) -> Result<Agent, String> {
        match s {
            "claude" => Ok(Agent::Claude),
            "codex" => Ok(Agent::Codex),
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
    /// Broker auth token; empty until a PTY is spawned, regenerated per spawn.
    pub token: String,
    /// Last known status. Stored as `Unknown` on spawn; updates land in ticket 12.
    pub status: Status,
    /// Bytes typed since the last Enter (backend shortcut: counts raw bytes, not characters).
    pub draft_len: usize,
    /// Persisted label shown in the UI; empty when unset.
    pub name: String,
    /// Extra agent CLI args appended to the launch command.
    pub agent_args: Vec<String>,
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
    pub name: String,
}

pub type Sessions = Mutex<HashMap<Uuid, Session>>;

/// Resolve a full session UUID or a unique id8 prefix to a live session id.
pub fn resolve_id(sessions: &HashMap<Uuid, Session>, s: &str) -> Option<Uuid> {
    if let Ok(id) = Uuid::parse_str(s) {
        return sessions.contains_key(&id).then_some(id);
    }
    if s.is_empty() || s.len() > 8 {
        return None;
    }
    let mut it = sessions.values().filter(|x| x.id8.starts_with(s));
    let first = it.next()?;
    it.next().is_none().then_some(first.id)
}

/// (project, agent) of a session.
pub fn lookup(sessions: &HashMap<Uuid, Session>, id: Uuid) -> Option<(Uuid, Agent)> {
    sessions.get(&id).map(|s| (s.project_id, s.agent))
}

pub fn worktree_error_message(e: &WorktreeError) -> String {
    match e {
        WorktreeError::NotARepo => "not a git repository".to_string(),
        WorktreeError::NoCommits => "repository has no commits".to_string(),
        WorktreeError::BranchExists(b) => format!("branch {b} already exists"),
        WorktreeError::Git(m) => m.clone(),
        WorktreeError::Io(e) => e.to_string(),
    }
}

/// Create the worktree and registry entry without a PTY.
pub fn register(
    app: &AppHandle,
    project_id: Uuid,
    agent: Agent,
    name: &str,
    agent_args: Vec<String>,
) -> Result<SessionInfo, String> {
    if !agent_args.is_empty() && agent == Agent::Shell {
        return Err("agent args need a claude or codex session".into());
    }
    let sessions = app.state::<Sessions>();
    let projects = app.state::<Projects>();
    let repo = projects
        .lock()
        .map_err(|e| e.to_string())?
        .iter()
        .find(|p| p.id == project_id)
        .map(|p| p.path.clone())
        .ok_or("unknown project")?;
    let id = Uuid::new_v4();
    let id8: String = id.simple().to_string().chars().take(8).collect();

    let root = app
        .state::<SettingsState>()
        .lock()
        .map_err(|e| e.to_string())?
        .worktrees_root
        .clone();
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let wt = worktree::create(&repo, agent.as_str(), &id8, &root)
        .map_err(|e| worktree_error_message(&e))?;

    let session = Session {
        id,
        id8,
        project_id,
        agent,
        repo,
        worktree: wt,
        pty: None,
        poller_stop: None,
        token: String::new(),
        status: Status::Unknown,
        draft_len: 0,
        name: name.to_string(),
        agent_args,
    };
    let info = info_for(&session);
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    map.insert(id, session);
    persist(&map, &projects.lock().map_err(|e| e.to_string())?);
    Ok(info)
}

// Lock order everywhere: sessions, then projects.
#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub fn create_session(
    project_id: String,
    agent: String,
    cols: u16,
    rows: u16,
    name: Option<String>,
    agent_args: Option<Vec<String>>,
    on_data: Channel<InvokeResponseBody>,
    sessions: State<'_, Sessions>,
    projects: State<'_, Projects>,
    app: AppHandle,
) -> Result<SessionInfo, String> {
    let agent = Agent::parse(&agent)?;
    let project_id = Uuid::parse_str(&project_id).map_err(|_| "unknown project".to_string())?;
    let info = register(
        &app,
        project_id,
        agent,
        &name.unwrap_or_default(),
        agent_args.unwrap_or_default(),
    )?;
    let id = Uuid::parse_str(&info.id).map_err(|e| e.to_string())?;
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    let s = map.get_mut(&id).ok_or("no such session")?;
    if let Err(e) = spawn_for(s, &app, cols, rows, on_data, false) {
        map.remove(&id);
        persist(&map, &projects.lock().map_err(|e| e.to_string())?);
        return Err(e);
    }
    Ok(info)
}

/// Spawn the PTY (and status poller for agent sessions) for an existing session.
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
    let rt = app.state::<Runtime>();
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let env = vec![
        (
            "AGENTFLEET_SOCKET".to_string(),
            rt.socket.to_string_lossy().to_string(),
        ),
        ("AGENTFLEET_SESSION".to_string(), id.to_string()),
        ("AGENTFLEET_TOKEN".to_string(), token.clone()),
    ];
    let path_prefix = format!(
        "export PATH={}:\"$PATH\"; ",
        pty::shell_quote(&rt.shim_dir().to_string_lossy())
    );
    let mut command = format!("{path_prefix}exec \"$SHELL\" -i");
    let mut poller_stop = None;
    if session.agent != Agent::Shell {
        let run = platform::home_dir().join(".agentfleet/run");
        std::fs::create_dir_all(&run).map_err(|e| e.to_string())?;
        let status_file = run.join(format!("{id}.status"));
        let auto_permission = app
            .state::<SettingsState>()
            .lock()
            .map_err(|e| e.to_string())?
            .auto_permission;
        let launch = if session.agent == Agent::Codex {
            let script = run.join(format!("{id}.codex-hook.sh"));
            let payload = run.join(format!("{id}.codex-hook.json"));
            let sp = script.to_string_lossy();
            if sp.contains('"') || sp.contains('\\') {
                return Err(format!("codex hook path contains '\"' or '\\': {sp}"));
            }
            agent::write_codex_hook(&status_file, &payload, &script).map_err(|e| e.to_string())?;
            let resume_id = if resume {
                agent::codex_session_id(&payload)
            } else {
                None
            };
            agent::codex_launch(
                &script,
                resume_id.as_deref(),
                auto_permission,
                &session.agent_args,
            )
        } else {
            let settings = run.join(format!("{id}.settings.json"));
            agent::write_hooks_settings(&status_file, &rt.shim, &settings)
                .map_err(|e| e.to_string())?;
            agent::claude_launch(id, &settings, resume, auto_permission, &session.agent_args)
        };
        command = format!("{path_prefix}exec {launch}");
        status::spawn_poller(app.clone(), id, status_file, stop_rx);
        poller_stop = Some(stop_tx.clone());
    }
    let exit_app = app.clone();
    // Codex: no hook fires before the first turn, so readiness (→ `waiting`, which triggers
    // auto-delivery) comes from the TUI's OSC 7501 idle signal, with a timer as fallback.
    let on_chunk: Box<dyn Fn(Vec<u8>) + Send + 'static> = if session.agent == Agent::Codex {
        let ready_app = app.clone();
        // (seen_working, readied); the PTY reader is the only caller, the mutex is uncontended.
        let ready = Mutex::new((false, false));
        Box::new(move |chunk: Vec<u8>| {
            if let Ok(mut st) = ready.lock() {
                if !st.1 && agent::codex_ready_scan(&mut st.0, &chunk) {
                    st.1 = true;
                    mark_waiting_if_unknown(&ready_app, id);
                }
            }
            let _ = on_data.send(InvokeResponseBody::Raw(chunk));
        })
    } else {
        Box::new(move |chunk: Vec<u8>| {
            let _ = on_data.send(InvokeResponseBody::Raw(chunk));
        })
    };
    if session.agent == Agent::Codex {
        let fallback_app = app.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(6)).await;
            mark_waiting_if_unknown(&fallback_app, id);
        });
    }
    let handle = pty::spawn(
        &platform::login_shell(),
        &session.worktree.path,
        Some(&command),
        &env,
        cols,
        rows,
        on_chunk,
        Box::new(move |_code| {
            // Stop the poller first so it cannot overwrite `ended`.
            let _ = stop_tx.send(true);
            status::emit(&exit_app, id, Status::Ended);
        }),
    )
    .map_err(|e| e.to_string())?;
    session.pty = Some(handle);
    session.poller_stop = poller_stop;
    session.token = token;
    session.status = Status::Unknown;
    session.draft_len = 0;
    Ok(())
}

/// Emit `waiting` for `id` only while its status is still `Unknown` (lock-and-release first).
fn mark_waiting_if_unknown(app: &AppHandle, id: Uuid) {
    let unknown = app
        .state::<Sessions>()
        .lock()
        .map(|m| {
            m.get(&id)
                .map(|s| s.status == Status::Unknown)
                .unwrap_or(false)
        })
        .unwrap_or(false);
    if unknown {
        status::emit(app, id, Status::Waiting);
    }
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
        name: s.name.clone(),
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
            name: s.name.clone(),
            agent_args: s.agent_args.clone(),
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
                    token: String::new(),
                    status: Status::Unknown,
                    draft_len: 0,
                    name: p.name,
                    agent_args: p.agent_args,
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

/// Remove every session without a PTY; returns how many were removed.
fn drain_unattached(map: &mut HashMap<Uuid, Session>) -> usize {
    let before = map.len();
    map.retain(|_, s| s.pty.is_some());
    before - map.len()
}

/// Forget restorable sessions (worktrees and branches stay on disk).
#[tauri::command]
pub fn forget_restorable(
    sessions: State<'_, Sessions>,
    projects: State<'_, Projects>,
) -> Result<usize, String> {
    let mut map = sessions.lock().map_err(|e| e.to_string())?;
    let n = drain_unattached(&mut map);
    persist(&map, &projects.lock().map_err(|e| e.to_string())?);
    Ok(n)
}

#[tauri::command]
pub fn attach_session(
    id: String,
    cols: u16,
    rows: u16,
    resume: bool,
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
    spawn_for(s, &app, cols, rows, on_data, resume)
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
    app: AppHandle,
) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let retrigger = {
        let mut map = sessions.lock().map_err(|e| e.to_string())?;
        let s = map.get_mut(&id).ok_or("no such session")?;
        let before = s.draft_len;
        let after = crate::deliver::apply_draft(before, &data);
        s.draft_len = after;
        let p = s.pty.as_ref().ok_or("session has no pty")?;
        p.write(&data).map_err(|e| e.to_string())?;
        before > 0
            && after == 0
            && !data.iter().any(|b| *b == b'\r' || *b == b'\n')
            && s.status == Status::Waiting
    };
    if retrigger {
        crate::deliver::on_status(&app, id, Status::Waiting);
    }
    Ok(())
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

#[derive(Serialize, Debug, Clone, PartialEq)]
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
pub fn close_session(id: String, app: AppHandle) -> Result<CloseOutcome, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    close_one(&app, uuid)
}

/// Close one session: remove from registry, stop poller/PTY, clean run files, remove clean worktree.
pub fn close_one(app: &AppHandle, uuid: Uuid) -> Result<CloseOutcome, String> {
    let sessions = app.state::<Sessions>();
    let projects = app.state::<Projects>();
    // Take ownership so the mutex is not held across kill (which joins the reader).
    let mut s = {
        let mut map = sessions.lock().map_err(|e| e.to_string())?;
        let s = map.remove(&uuid).ok_or("no such session")?;
        persist(&map, &projects.lock().map_err(|e| e.to_string())?);
        s
    };
    {
        let collab_state = app.state::<CollabState>();
        let touched = {
            let mut c = collab_state.lock().map_err(|e| e.to_string())?;
            let t = collab::interrupt_all_for(&mut c, uuid, collab::now());
            collab::save(&c);
            t
        };
        collab::changed(app, &app.state::<CollabNotify>().0, &touched);
    }
    if let Some(tx) = &s.poller_stop {
        let _ = tx.send(true);
    }
    if let Some(mut p) = s.pty.take() {
        let _ = p.kill();
    }
    let run = platform::home_dir().join(".agentfleet/run");
    for ext in [
        "status",
        "settings.json",
        "codex-hook.sh",
        "codex-hook.json",
    ] {
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

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClosedSession {
    pub id: String,
    pub outcome: CloseOutcome,
}

/// Closes `ids` in order; stops at the first error (already-closed ones stay closed).
/// Only when every close succeeded does `finish` run (the project removal).
pub fn cascade(
    ids: Vec<Uuid>,
    mut close: impl FnMut(Uuid) -> Result<CloseOutcome, String>,
    finish: impl FnOnce() -> Result<(), String>,
) -> Result<Vec<ClosedSession>, String> {
    let mut out = Vec::with_capacity(ids.len());
    for id in ids {
        let outcome = close(id)?;
        out.push(ClosedSession {
            id: id.to_string(),
            outcome,
        });
    }
    finish()?;
    Ok(out)
}

#[tauri::command]
pub fn remove_project_cascade(id: String, app: AppHandle) -> Result<Vec<ClosedSession>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "no such project".to_string())?;
    let ids = {
        let sessions = app.state::<Sessions>();
        let projects = app.state::<Projects>();
        let map = sessions.lock().map_err(|e| e.to_string())?;
        if !projects
            .lock()
            .map_err(|e| e.to_string())?
            .iter()
            .any(|p| p.id == uuid)
        {
            return Err("no such project".into());
        }
        let mut v: Vec<(String, Uuid)> = map
            .values()
            .filter(|s| s.project_id == uuid)
            .map(|s| (s.id8.clone(), s.id))
            .collect();
        v.sort();
        v.into_iter().map(|(_, id)| id).collect::<Vec<_>>()
    };
    cascade(
        ids,
        |sid| close_one(&app, sid),
        || {
            let sessions = app.state::<Sessions>();
            let projects = app.state::<Projects>();
            let map = sessions.lock().map_err(|e| e.to_string())?;
            let mut list = projects.lock().map_err(|e| e.to_string())?;
            let has = map.values().any(|s| s.project_id == uuid);
            project::remove_from(&mut list, uuid, has)?;
            persist(&map, &list);
            Ok(())
        },
    )
}

/// Pure: whether closing the window should ask for confirmation.
pub fn should_confirm(confirm_quit: bool, running: usize) -> bool {
    confirm_quit && running > 0
}

/// Number of sessions with a live PTY (lock-and-release).
pub fn running_count(app: &AppHandle) -> usize {
    app.state::<Sessions>()
        .lock()
        .map(|m| m.values().filter(|s| s.pty.is_some()).count())
        .unwrap_or(0)
}

/// Kill every PTY and stop pollers, then exit the app.
pub fn shutdown(app: &AppHandle) {
    let (handles, stops) = {
        let mut handles = Vec::new();
        let mut stops = Vec::new();
        if let Ok(mut map) = app.state::<Sessions>().lock() {
            for s in map.values_mut() {
                if let Some(h) = s.pty.take() {
                    handles.push(h);
                }
                if let Some(tx) = s.poller_stop.take() {
                    stops.push(tx);
                }
            }
        }
        (handles, stops)
    };
    for tx in &stops {
        let _ = tx.send(true);
    }
    for mut h in handles {
        let _ = h.kill();
    }
    app.exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake(pty: Option<pty::PtyHandle>) -> (Uuid, Session) {
        let id = Uuid::new_v4();
        (
            id,
            Session {
                id,
                id8: id.simple().to_string()[..8].to_string(),
                project_id: Uuid::new_v4(),
                agent: Agent::Shell,
                repo: PathBuf::from("/r"),
                worktree: Worktree {
                    repo: PathBuf::from("/r"),
                    path: PathBuf::from("/w"),
                    branch: "b".into(),
                    base_commit: "c".into(),
                },
                pty,
                poller_stop: None,
                token: String::new(),
                status: Status::Unknown,
                draft_len: 0,
                name: String::new(),
                agent_args: Vec::new(),
            },
        )
    }

    #[test]
    fn drain_unattached_keeps_live() {
        let mut live = pty::spawn(
            "/bin/sh",
            std::path::Path::new("/tmp"),
            None,
            &[],
            80,
            24,
            Box::new(|_| {}),
            Box::new(|_| {}),
        )
        .unwrap();
        let mut map = HashMap::new();
        let (a, sa) = fake(None);
        let (b, sb) = fake(None);
        let (c, sc) = fake(None);
        map.insert(a, sa);
        map.insert(b, sb);
        let mut sc = sc;
        sc.pty = Some(live);
        map.insert(c, sc);
        assert_eq!(drain_unattached(&mut map), 2);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&c));
        live = map.get_mut(&c).unwrap().pty.take().unwrap();
        let _ = live.kill();
    }

    #[test]
    fn confirm_decision() {
        assert!(should_confirm(true, 2));
        assert!(!should_confirm(true, 0));
        assert!(!should_confirm(false, 2));
    }

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

    fn ids(n: usize) -> Vec<Uuid> {
        (0..n).map(|_| Uuid::new_v4()).collect()
    }

    #[test]
    fn cascade_closes_all_then_finishes() {
        let v = ids(3);
        let mut finished = 0;
        let r = cascade(
            v.clone(),
            |_| Ok(CloseOutcome::Removed),
            || {
                finished += 1;
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(finished, 1);
        let got: Vec<String> = r.iter().map(|c| c.id.clone()).collect();
        assert_eq!(got, v.iter().map(|u| u.to_string()).collect::<Vec<_>>());
        assert!(r.iter().all(|c| c.outcome == CloseOutcome::Removed));
    }

    #[test]
    fn cascade_stops_at_first_error() {
        let mut calls = 0;
        let mut finished = false;
        let r = cascade(
            ids(3),
            |_| {
                calls += 1;
                if calls == 2 {
                    Err("boom".to_string())
                } else {
                    Ok(CloseOutcome::Removed)
                }
            },
            || {
                finished = true;
                Ok(())
            },
        );
        assert_eq!(r, Err("boom".to_string()));
        assert_eq!(calls, 2);
        assert!(!finished);
    }

    #[test]
    fn cascade_kept_is_not_an_error() {
        let mut finished = false;
        let r = cascade(
            ids(1),
            |_| Ok(CloseOutcome::Kept { path: "/w".into() }),
            || {
                finished = true;
                Ok(())
            },
        )
        .unwrap();
        assert!(finished);
        assert_eq!(r[0].outcome, CloseOutcome::Kept { path: "/w".into() });
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
