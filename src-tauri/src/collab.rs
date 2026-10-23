//! Collaboration runs (agent-to-agent requests): pure logic over `Collab` plus persistence
//! at ~/.agentfleet/collaboration.json. Auto-delivery and status transitions land in ticket 12.
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Notify;
use uuid::Uuid;

use crate::platform;
use crate::session::{self, Agent, Sessions};

pub const MAX_PROMPT: usize = 256 * 1024;
pub const MAX_RESPONSE: usize = 128 * 1024;
const MAX_RUNS: usize = 500;
const TERMINAL_MAX_AGE: u64 = 30 * 24 * 3600;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RunStatus {
    Queued,
    Running,
    Completed,
    Cancelled,
    Interrupted,
    NeedsAttention,
}

impl RunStatus {
    /// Anything other than queued/running is final.
    pub fn is_terminal(&self) -> bool {
        !matches!(self, RunStatus::Queued | RunStatus::Running)
    }

    fn as_str(&self) -> &'static str {
        match self {
            RunStatus::Queued => "queued",
            RunStatus::Running => "running",
            RunStatus::Completed => "completed",
            RunStatus::Cancelled => "cancelled",
            RunStatus::Interrupted => "interrupted",
            RunStatus::NeedsAttention => "needsAttention",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub id: Uuid,
    pub project_id: Uuid,
    pub from_session_id: Uuid,
    pub to_session_id: Uuid,
    pub prompt: String,
    pub status: RunStatus,
    pub response: Option<String>,
    pub detail: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Default)]
pub struct Collab {
    pub runs: Vec<Run>,
    /// session -> run it is blocked on (`run.wait`)
    pub waiting_on: HashMap<Uuid, Uuid>,
    /// Where `save` writes; `None` (tests) disables persistence.
    pub path: Option<PathBuf>,
}

pub type CollabState = Mutex<Collab>;

/// Wakes `run.wait` callers. Kept outside the `CollabState` mutex so it is never awaited under the lock.
#[derive(Clone, Default)]
pub struct CollabNotify(pub Arc<Notify>);

#[derive(Serialize, Deserialize)]
struct Persisted {
    version: u32,
    runs: Vec<Run>,
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn collab_path() -> PathBuf {
    platform::home_dir().join(".agentfleet/collaboration.json")
}

fn write_runs(runs: &[Run], path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let p = Persisted {
        version: 1,
        runs: runs.to_vec(),
    };
    fs::write(
        &tmp,
        serde_json::to_string_pretty(&p).map_err(io::Error::other)?,
    )?;
    fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))?;
    fs::rename(&tmp, path)
}

/// Persist after a mutation; errors are logged, never fatal.
pub fn save(c: &Collab) {
    if let Some(path) = &c.path {
        if let Err(e) = write_runs(&c.runs, path) {
            eprintln!("agentfleet: failed to save collaboration: {e}");
        }
    }
}

/// Load from `path`; unreadable or wrong version yields an empty (logged) collab.
pub fn load(path: &Path) -> Collab {
    let runs = match fs::read_to_string(path) {
        Ok(t) => match serde_json::from_str::<Persisted>(&t) {
            Ok(p) if p.version == 1 => p.runs,
            Ok(p) => {
                eprintln!(
                    "agentfleet: unsupported collaboration version {}",
                    p.version
                );
                Vec::new()
            }
            Err(e) => {
                eprintln!("agentfleet: ignoring unreadable collaboration: {e}");
                Vec::new()
            }
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            eprintln!("agentfleet: ignoring unreadable collaboration: {e}");
            Vec::new()
        }
    };
    let mut c = Collab {
        runs,
        waiting_on: HashMap::new(),
        path: Some(path.to_path_buf()),
    };
    let mut interrupted = false;
    for r in &mut c.runs {
        if !r.status.is_terminal() {
            r.status = RunStatus::Interrupted;
            r.detail = Some("application restarted".into());
            r.updated_at = now();
            interrupted = true;
        }
    }
    prune(&mut c, now());
    if interrupted {
        save(&c);
    }
    c
}

/// Wake waiters and tell the UI which projects changed. Call after every mutation.
pub fn changed(app: &AppHandle, notify: &Notify, projects: &[Uuid]) {
    notify.notify_waiters();
    for p in projects {
        let _ = app.emit("collab-changed", serde_json::json!({ "projectId": p }));
    }
}

/// Keep at most 500 runs; drop terminal runs untouched for 30 days.
pub fn prune(c: &mut Collab, now: u64) {
    c.runs.retain(|r| {
        !(r.status.is_terminal() && now.saturating_sub(r.updated_at) > TERMINAL_MAX_AGE)
    });
    while c.runs.len() > MAX_RUNS {
        let Some(i) = c.runs.iter().position(|r| r.status.is_terminal()) else {
            break;
        };
        c.runs.remove(i);
    }
}

pub fn request(
    c: &mut Collab,
    from: Uuid,
    to: Uuid,
    prompt: &str,
    now: u64,
    lookup: &dyn Fn(Uuid) -> Option<(Uuid, Agent)>,
) -> Result<Run, String> {
    if from == to {
        return Err("cannot send a request to yourself".into());
    }
    if prompt.len() > MAX_PROMPT {
        return Err("prompt too large (max 256 KB)".into());
    }
    let (project, _) = lookup(from).ok_or("unknown source session")?;
    let (tproject, tagent) = lookup(to).ok_or("no such session")?;
    if tproject != project {
        return Err("target session is in a different project".into());
    }
    if tagent == Agent::Shell {
        return Err("target is not an agent session".into());
    }
    prune(c, now);
    if c.runs.iter().filter(|r| !r.status.is_terminal()).count() >= MAX_RUNS {
        return Err("too many active requests (max 500); complete or cancel a run first".into());
    }
    let run = Run {
        id: Uuid::new_v4(),
        project_id: project,
        from_session_id: from,
        to_session_id: to,
        prompt: prompt.to_string(),
        status: RunStatus::Queued,
        response: None,
        detail: None,
        created_at: now,
        updated_at: now,
    };
    c.runs.push(run.clone());
    prune(c, now);
    Ok(run)
}

fn find_mut(c: &mut Collab, id: Uuid) -> Result<&mut Run, String> {
    c.runs
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| "no such run".to_string())
}

pub fn reply(c: &mut Collab, run: Uuid, from: Uuid, text: &str, now: u64) -> Result<Run, String> {
    if text.len() > MAX_RESPONSE {
        return Err("response too large (max 128 KB)".into());
    }
    let r = find_mut(c, run)?;
    if r.to_session_id != from {
        return Err("only the target session can reply".into());
    }
    if r.status.is_terminal() {
        return Err(format!("run is already {}", r.status.as_str()));
    }
    r.status = RunStatus::Completed;
    r.response = Some(text.to_string());
    r.updated_at = now;
    Ok(r.clone())
}

pub fn cancel(c: &mut Collab, run: Uuid, by: Uuid, now: u64) -> Result<Run, String> {
    let r = find_mut(c, run)?;
    if r.from_session_id != by && r.to_session_id != by {
        return Err("only the sender or target can cancel".into());
    }
    if r.status.is_terminal() {
        return Err(format!("run is already {}", r.status.as_str()));
    }
    r.status = RunStatus::Cancelled;
    r.updated_at = now;
    Ok(r.clone())
}

pub fn set_status(c: &mut Collab, run: Uuid, status: RunStatus, detail: Option<&str>, now: u64) {
    if let Some(r) = c.runs.iter_mut().find(|r| r.id == run) {
        r.status = status;
        r.detail = detail.map(str::to_string);
        r.updated_at = now;
    }
}

/// Mark live runs to/from `session` interrupted and drop its wait; returns touched project ids.
pub fn interrupt_all_for(c: &mut Collab, session: Uuid, now: u64) -> Vec<Uuid> {
    let mut projects = Vec::new();
    for r in c.runs.iter_mut() {
        if !r.status.is_terminal() && (r.from_session_id == session || r.to_session_id == session) {
            r.status = RunStatus::Interrupted;
            r.updated_at = now;
            if !projects.contains(&r.project_id) {
                projects.push(r.project_id);
            }
        }
    }
    c.waiting_on.remove(&session);
    projects
}

/// Resolve a full run id or a unique id8 prefix.
pub fn resolve_run(c: &Collab, s: &str) -> Option<Uuid> {
    if let Ok(id) = Uuid::parse_str(s) {
        return c.runs.iter().any(|r| r.id == id).then_some(id);
    }
    if s.is_empty() || s.len() > 8 {
        return None;
    }
    let mut it = c
        .runs
        .iter()
        .filter(|r| r.id.simple().to_string().starts_with(s));
    let first = it.next()?;
    it.next().is_none().then_some(first.id)
}

pub fn begin_wait(c: &mut Collab, session: Uuid, run: Uuid) -> Result<(), String> {
    let requested = c.runs.iter().find(|r| r.id == run).ok_or("no such run")?;
    if requested.status.is_terminal() {
        return Ok(());
    }
    let mut cur = requested.to_session_id;
    let mut seen = HashSet::new();
    loop {
        if cur == session {
            return Err("wait cycle".into());
        }
        if !seen.insert(cur) {
            break;
        }
        let next = c
            .waiting_on
            .get(&cur)
            .and_then(|rid| c.runs.iter().find(|r| r.id == *rid))
            .filter(|r| !r.status.is_terminal())
            .map(|r| r.to_session_id);
        match next {
            Some(n) => cur = n,
            None => break,
        }
    }
    c.waiting_on.insert(session, run);
    Ok(())
}

pub fn clear_wait(c: &mut Collab, session: Uuid) {
    c.waiting_on.remove(&session);
}

/// Runs to or from `session_id`, newest first (UI).
#[tauri::command]
pub fn list_runs(session_id: String, collab: State<CollabState>) -> Result<Vec<Run>, String> {
    let id = Uuid::parse_str(&session_id).map_err(|e| e.to_string())?;
    let c = collab.lock().map_err(|e| e.to_string())?;
    let mut runs: Vec<Run> = c
        .runs
        .iter()
        .rev()
        .filter(|r| r.from_session_id == id || r.to_session_id == id)
        .cloned()
        .collect();
    runs.sort_by_key(|r| std::cmp::Reverse(r.created_at)); // stable: newest-inserted first on ties
    Ok(runs)
}

/// Human-authored request from the UI; same rules as `session.request`.
#[tauri::command]
pub fn send_request(from: String, to: String, text: String, app: AppHandle) -> Result<Run, String> {
    let run = {
        let sessions = app.state::<Sessions>();
        let collab_state = app.state::<CollabState>();
        let map = sessions.lock().map_err(|e| e.to_string())?;
        let mut c = collab_state.lock().map_err(|e| e.to_string())?;
        let from = session::resolve_id(&map, &from).ok_or("unknown source session")?;
        let to = session::resolve_id(&map, &to).ok_or("no such session")?;
        let r = request(&mut c, from, to, &text, now(), &|id| {
            session::lookup(&map, id)
        })?;
        save(&c);
        r
    };
    changed(&app, &app.state::<CollabNotify>().0, &[run.project_id]);
    crate::deliver::on_request_created(&app, run.id);
    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct W {
        p: Uuid,
        a: Uuid,
        b: Uuid,
        sh: Uuid,
        other: Uuid,
        cx: Uuid,
        map: HashMap<Uuid, (Uuid, Agent)>,
    }

    fn world() -> W {
        let (p, q) = (Uuid::new_v4(), Uuid::new_v4());
        let ids: Vec<Uuid> = (0..5).map(|_| Uuid::new_v4()).collect();
        let mut map = HashMap::new();
        map.insert(ids[0], (p, Agent::Claude));
        map.insert(ids[1], (p, Agent::Claude));
        map.insert(ids[2], (p, Agent::Shell));
        map.insert(ids[3], (q, Agent::Claude));
        map.insert(ids[4], (p, Agent::Codex));
        W {
            p,
            a: ids[0],
            b: ids[1],
            sh: ids[2],
            other: ids[3],
            cx: ids[4],
            map,
        }
    }

    fn req(c: &mut Collab, w: &W, from: Uuid, to: Uuid, text: &str) -> Result<Run, String> {
        request(c, from, to, text, 100, &|id| w.map.get(&id).copied())
    }

    #[test]
    fn request_rules() {
        let w = world();
        let mut c = Collab::default();
        let r = req(&mut c, &w, w.a, w.b, "hi").unwrap();
        assert_eq!(r.status, RunStatus::Queued);
        assert_eq!(r.project_id, w.p);
        assert_eq!(
            req(&mut c, &w, w.a, w.sh, "hi").unwrap_err(),
            "target is not an agent session"
        );
        assert!(req(&mut c, &w, w.a, w.other, "hi").is_err());
        assert!(req(&mut c, &w, w.a, w.a, "hi").is_err());
        assert!(req(&mut c, &w, w.a, Uuid::new_v4(), "hi").is_err());
        let big = "x".repeat(MAX_PROMPT + 1);
        assert!(req(&mut c, &w, w.a, w.b, &big).is_err());
        assert_eq!(c.runs.len(), 1);
        let rc = req(&mut c, &w, w.a, w.cx, "review").unwrap();
        assert_eq!(rc.to_session_id, w.cx);
        assert_eq!(c.runs.len(), 2);
    }

    #[test]
    fn reply_and_cancel_transitions() {
        let w = world();
        let mut c = Collab::default();
        let r = req(&mut c, &w, w.a, w.b, "hi").unwrap();
        assert!(reply(&mut c, r.id, w.a, "no", 101).is_err()); // sender cannot reply
        assert!(reply(&mut c, r.id, w.b, &"x".repeat(MAX_RESPONSE + 1), 101).is_err());
        let done = reply(&mut c, r.id, w.b, "pong", 102).unwrap();
        assert_eq!(done.status, RunStatus::Completed);
        assert_eq!(done.response.as_deref(), Some("pong"));
        assert!(reply(&mut c, r.id, w.b, "again", 103).is_err());
        assert!(cancel(&mut c, r.id, w.a, 103).is_err());

        let r2 = req(&mut c, &w, w.a, w.b, "hi").unwrap();
        assert!(cancel(&mut c, r2.id, w.sh, 104).is_err());
        assert_eq!(
            cancel(&mut c, r2.id, w.b, 104).unwrap().status,
            RunStatus::Cancelled
        );
        assert!(cancel(&mut c, Uuid::new_v4(), w.a, 104).is_err());
    }

    #[test]
    fn interrupt_all() {
        let w = world();
        let mut c = Collab::default();
        let live = req(&mut c, &w, w.a, w.b, "1").unwrap();
        let done = req(&mut c, &w, w.b, w.a, "2").unwrap();
        reply(&mut c, done.id, w.a, "ok", 101).unwrap();
        begin_wait(&mut c, w.b, live.id).unwrap_err(); // target == self is a cycle
        begin_wait(&mut c, w.a, live.id).unwrap();
        let touched = interrupt_all_for(&mut c, w.b, 200);
        assert_eq!(touched, vec![w.p]);
        assert_eq!(c.runs[0].status, RunStatus::Interrupted);
        assert_eq!(c.runs[1].status, RunStatus::Completed);
        assert_eq!(c.runs[0].updated_at, 200);
        interrupt_all_for(&mut c, w.a, 201);
        assert!(c.waiting_on.is_empty());
    }

    #[test]
    fn set_status_updates() {
        let w = world();
        let mut c = Collab::default();
        let r = req(&mut c, &w, w.a, w.b, "x").unwrap();
        set_status(&mut c, r.id, RunStatus::NeedsAttention, Some("why"), 150);
        assert_eq!(c.runs[0].status, RunStatus::NeedsAttention);
        assert_eq!(c.runs[0].detail.as_deref(), Some("why"));
        assert_eq!(c.runs[0].updated_at, 150);
    }

    #[test]
    fn completed_runs_do_not_create_wait_cycles() {
        let w = world();
        let mut c = Collab::default();
        let ab = req(&mut c, &w, w.a, w.b, "1").unwrap();
        reply(&mut c, ab.id, w.b, "done", 101).unwrap();
        let ba = req(&mut c, &w, w.b, w.a, "2").unwrap();
        begin_wait(&mut c, w.b, ba.id).unwrap();
        assert_eq!(begin_wait(&mut c, w.a, ab.id), Ok(()));
        assert!(!c.waiting_on.contains_key(&w.a));
    }

    #[test]
    fn capacity_rejects_new_requests_without_dropping_live_runs() {
        let w = world();
        let mut c = Collab::default();
        let first = req(&mut c, &w, w.a, w.b, "first").unwrap();
        for _ in 1..MAX_RUNS {
            req(&mut c, &w, w.a, w.b, "live").unwrap();
        }
        assert!(req(&mut c, &w, w.a, w.b, "overflow").is_err());
        assert_eq!(c.runs.len(), MAX_RUNS);
        assert_eq!(c.runs[0].id, first.id);
        reply(&mut c, first.id, w.b, "done", 101).unwrap();
        assert!(req(&mut c, &w, w.a, w.b, "accepted").is_ok());
        assert_eq!(c.runs.len(), MAX_RUNS);
    }

    #[test]
    fn restart_interrupts_active_runs_but_preserves_history() {
        let w = world();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("collab.json");
        let mut c = Collab {
            path: Some(path.clone()),
            ..Collab::default()
        };
        let done = req(&mut c, &w, w.a, w.b, "done").unwrap();
        reply(&mut c, done.id, w.b, "answer", now()).unwrap();
        req(&mut c, &w, w.a, w.b, "queued").unwrap();
        let running = req(&mut c, &w, w.a, w.b, "running").unwrap();
        set_status(&mut c, running.id, RunStatus::Running, None, now());
        save(&c);
        let loaded = load(&path);
        assert_eq!(loaded.runs[0].response.as_deref(), Some("answer"));
        assert!(loaded.runs[1..]
            .iter()
            .all(|r| r.status == RunStatus::Interrupted));
        assert_eq!(load(&path).runs, loaded.runs);
    }

    #[test]
    fn wait_cycle() {
        let w = world();
        let mut c = Collab::default();
        let ab = req(&mut c, &w, w.a, w.b, "1").unwrap();
        let ba = req(&mut c, &w, w.b, w.a, "2").unwrap();
        begin_wait(&mut c, w.a, ab.id).unwrap();
        assert_eq!(begin_wait(&mut c, w.b, ba.id), Err("wait cycle".into()));
        clear_wait(&mut c, w.a);
        begin_wait(&mut c, w.b, ba.id).unwrap();
        assert!(begin_wait(&mut c, w.a, ab.id).is_err());
        assert!(begin_wait(&mut c, w.a, Uuid::new_v4()).is_err());
    }

    #[test]
    fn prune_rules() {
        let w = world();
        let mut c = Collab::default();
        let old = req(&mut c, &w, w.a, w.b, "old").unwrap();
        cancel(&mut c, old.id, w.a, 100).unwrap();
        let live = req(&mut c, &w, w.a, w.b, "live").unwrap();
        prune(&mut c, 100 + TERMINAL_MAX_AGE + 1);
        assert_eq!(c.runs.len(), 1);
        assert_eq!(c.runs[0].id, live.id);

        let mut c = Collab::default();
        for i in 0..MAX_RUNS {
            req(&mut c, &w, w.a, w.b, &i.to_string()).unwrap();
        }
        assert_eq!(c.runs.len(), MAX_RUNS);
        assert_eq!(c.runs[0].prompt, "0");
        assert!(req(&mut c, &w, w.a, w.b, "overflow").is_err());
    }

    #[test]
    fn resolve_prefix_and_persist() {
        let w = world();
        let mut c = Collab::default();
        let r = req(&mut c, &w, w.a, w.b, "hi").unwrap();
        let id8 = &r.id.simple().to_string()[..8];
        assert_eq!(resolve_run(&c, id8), Some(r.id));
        assert_eq!(resolve_run(&c, &r.id.to_string()), Some(r.id));
        assert_eq!(resolve_run(&c, ""), None);
        reply(&mut c, r.id, w.b, "done", now()).unwrap();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sub/collaboration.json");
        c.path = Some(path.clone());
        save(&c);
        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let loaded = load(&path);
        assert_eq!(loaded.runs, c.runs);
        assert!(fs::read_to_string(&path)
            .unwrap()
            .contains("\"version\": 1"));
    }
}
