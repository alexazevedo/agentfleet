//! Broker: authenticated Unix-socket JSON-lines server for the `agentfleet` CLI.
use std::future::Future;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Notify;
use uuid::Uuid;

use crate::collab::{self, Collab, CollabNotify, CollabState, Run};
use crate::project::Projects;
use crate::runtime::Runtime;
use crate::scratchpad;
use crate::session::{self, Sessions};

const MAX_REQUEST: u64 = 2 * 1024 * 1024;
const DEADLINE: Duration = Duration::from_secs(10);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // before/revision are consumed by tickets 12-15.
pub struct Request {
    pub method: String,
    pub source: Uuid,
    pub token: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub before: Option<String>,
    #[serde(default)]
    pub revision: Option<u64>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub agent_args: Option<Vec<String>>,
    #[serde(default)]
    pub worktree: Option<bool>,
}

/// Constant-time byte compare: early exit only on length mismatch.
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn err(msg: &str) -> Value {
    json!({ "error": msg })
}

/// Check source + token; returns the source's project id.
fn auth(sessions: &Sessions, req: &Request) -> Result<Uuid, Value> {
    let map = sessions.lock().map_err(|e| err(&e.to_string()))?;
    // Closed sessions are removed from the map; restored-but-unattached ones have an
    // empty token. Both are rejected.
    let src = map.get(&req.source).ok_or_else(|| err("unauthorized"))?;
    if src.token.is_empty() || !ct_eq(src.token.as_bytes(), req.token.as_bytes()) {
        return Err(err("unauthorized"));
    }
    Ok(src.project_id)
}

fn text_of(req: &Request) -> Result<&str, Value> {
    req.text.as_deref().ok_or_else(|| err("missing text"))
}

fn run_id(c: &Collab, req: &Request) -> Result<Uuid, Value> {
    let s = req
        .request_id
        .as_deref()
        .ok_or_else(|| err("missing requestId"))?;
    collab::resolve_run(c, s).ok_or_else(|| err("no such run"))
}

fn participant(c: &Collab, id: Uuid, who: Uuid) -> Result<Run, Value> {
    let r = c
        .runs
        .iter()
        .find(|r| r.id == id)
        .ok_or_else(|| err("no such run"))?;
    if r.from_session_id != who && r.to_session_id != who {
        return Err(err("not a participant of this run"));
    }
    Ok(r.clone())
}

/// Authenticate and dispatch one (non-waiting) request. Lock order: sessions, projects, collab.
/// Returns the response and, when a run changed, its project id (caller saves-notifies-emits;
/// `save` already happened here).
pub fn handle(
    req: &Request,
    sessions: &Sessions,
    _projects: &Projects,
    collab: &CollabState,
) -> (Value, Option<Uuid>) {
    match handle_inner(req, sessions, collab) {
        Ok(r) => r,
        Err(e) => (e, None),
    }
}

fn handle_inner(
    req: &Request,
    sessions: &Sessions,
    collab: &CollabState,
) -> Result<(Value, Option<Uuid>), Value> {
    let map = sessions.lock().map_err(|e| err(&e.to_string()))?;
    let src = map.get(&req.source).ok_or_else(|| err("unauthorized"))?;
    if src.token.is_empty() || !ct_eq(src.token.as_bytes(), req.token.as_bytes()) {
        return Err(err("unauthorized"));
    }
    if req.method == "session.list" {
        let mut list: Vec<_> = map
            .values()
            .filter(|s| s.project_id == src.project_id)
            .collect();
        list.sort_by(|a, b| a.id8.cmp(&b.id8));
        let sessions: Vec<Value> = list
            .into_iter()
            .map(|s| {
                json!({
                    "id": s.id.to_string(),
                    "id8": s.id8,
                    "agent": s.agent.as_str(),
                    "status": s.status,
                })
            })
            .collect();
        return Ok((json!({ "sessions": sessions }), None));
    }
    if let Some(op) = req.method.strip_prefix("scratchpad.") {
        let (project, _) = session::lookup(&map, req.source).ok_or_else(|| err("unauthorized"))?;
        return Ok((scratchpad_op(op, project, req), None));
    }
    let me = req.source;
    let mut c = collab.lock().map_err(|e| err(&e.to_string()))?;
    let changed = |r: &Run| (json!({ "run": r }), Some(r.project_id));
    match req.method.as_str() {
        "session.request" => {
            let t = req.target.as_deref().ok_or_else(|| err("missing target"))?;
            let to = session::resolve_id(&map, t).ok_or_else(|| err("no such session"))?;
            let text = text_of(req)?;
            let r = collab::request(&mut c, me, to, text, collab::now(), &|id| {
                session::lookup(&map, id)
            })
            .map_err(|e| err(&e))?;
            collab::save(&c);
            Ok(changed(&r))
        }
        "session.inbox" => {
            let mut runs: Vec<&Run> = c
                .runs
                .iter()
                .rev()
                .filter(|r| r.from_session_id == me || r.to_session_id == me)
                .collect();
            runs.sort_by_key(|r| std::cmp::Reverse(r.created_at)); // stable: newest-inserted first on ties
            runs.truncate(100);
            Ok((json!({ "runs": runs }), None))
        }
        "run.read" => {
            let id = run_id(&c, req)?;
            let r = participant(&c, id, me)?;
            Ok((json!({ "run": r }), None))
        }
        "run.reply" => {
            let id = run_id(&c, req)?;
            let text = text_of(req)?;
            let r = collab::reply(&mut c, id, me, text, collab::now()).map_err(|e| err(&e))?;
            collab::save(&c);
            Ok(changed(&r))
        }
        "run.cancel" => {
            let id = run_id(&c, req)?;
            let r = collab::cancel(&mut c, id, me, collab::now()).map_err(|e| err(&e))?;
            collab::save(&c);
            Ok(changed(&r))
        }
        "run.wait.clear" => {
            collab::clear_wait(&mut c, me);
            Ok((json!({}), None))
        }
        other => Err(err(&format!("unknown method: {other}"))),
    }
}

fn scratchpad_op(op: &str, project: Uuid, req: &Request) -> Value {
    let res = match op {
        "read" => {
            return match scratchpad::read(project) {
                Ok(p) => json!({ "revision": p.revision, "text": p.text }),
                Err(e) => err(&e.to_string()),
            }
        }
        "write" | "append" => match text_of(req) {
            Ok(t) if op == "write" => scratchpad::write(project, t, req.revision),
            Ok(t) => scratchpad::append(project, t, req.revision),
            Err(e) => return e,
        },
        other => return err(&format!("unknown method: scratchpad.{other}")),
    };
    match res {
        Ok(revision) => json!({ "revision": revision }),
        Err(e) => err(&e),
    }
}

const WAIT_BUDGET: Duration = Duration::from_secs(25);

/// `run.wait`: register the wait (cycle check), then sleep on `notify` until the run is
/// terminal or the budget runs out. The std mutex is never held across an await.
pub async fn wait(
    req: &Request,
    sessions: &Sessions,
    collab: &CollabState,
    notify: &Notify,
) -> Value {
    if let Err(e) = auth(sessions, req) {
        return e;
    }
    let me = req.source;
    let id = {
        let mut c = match collab.lock() {
            Ok(c) => c,
            Err(e) => return err(&e.to_string()),
        };
        let id = match run_id(&c, req) {
            Ok(id) => id,
            Err(e) => return e,
        };
        let r = match participant(&c, id, me) {
            Ok(r) => r,
            Err(e) => return e,
        };
        if r.status.is_terminal() {
            return json!({ "run": r, "timedOut": false });
        }
        if let Err(e) = collab::begin_wait(&mut c, me, id) {
            return err(&e);
        }
        id
    };
    let deadline = tokio::time::Instant::now() + WAIT_BUDGET;
    let mut timed_out = false;
    let run = loop {
        // Register interest before checking so a notify between check and await is not lost.
        let notified = notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        let cur = {
            let c = match collab.lock() {
                Ok(c) => c,
                Err(e) => return err(&e.to_string()),
            };
            c.runs.iter().find(|r| r.id == id).cloned()
        };
        let Some(r) = cur else {
            break None;
        };
        if r.status.is_terminal() {
            break Some(r);
        }
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() || tokio::time::timeout(remaining, notified).await.is_err() {
            timed_out = true;
            break Some(r);
        }
    };
    if let Ok(mut c) = collab.lock() {
        collab::clear_wait(&mut c, me);
    }
    match run {
        Some(r) => json!({ "run": r, "timedOut": timed_out }),
        None => err("no such run"),
    }
}

/// Resolve a close target: same project as `source`, resolvable, and not `source` itself.
pub fn close_target(
    map: &std::collections::HashMap<Uuid, session::Session>,
    source: Uuid,
    target: &str,
) -> Result<Uuid, String> {
    const NONE: &str = "no such session in this project";
    let (project, _) = session::lookup(map, source).ok_or("unauthorized")?;
    let id = session::resolve_id(map, target).ok_or(NONE)?;
    if session::lookup(map, id).map(|(p, _)| p) != Some(project) {
        return Err(NONE.into());
    }
    if id == source {
        return Err("cannot close yourself".into());
    }
    Ok(id)
}

fn validate_spawn(agent: session::Agent, req: &Request) -> Result<(), String> {
    if agent == session::Agent::Shell {
        if req.text.is_some() {
            return Err("shell sessions take no prompt".into());
        }
        if req.agent_args.as_ref().is_some_and(|a| !a.is_empty()) {
            return Err("agent args need an agent session".into());
        }
    }
    if req
        .text
        .as_ref()
        .is_some_and(|t| t.len() > collab::MAX_PROMPT)
    {
        return Err("prompt too large (max 256 KB)".into());
    }
    Ok(())
}

/// Register a sibling, queue its request, then announce it so the UI can attach.
pub async fn spawn(req: &Request, app: &AppHandle) -> Value {
    let sessions = app.state::<Sessions>();
    let project = match auth(&sessions, req) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let agent = match req.agent.as_deref() {
        Some(a) => session::Agent::parse(a),
        None => session::Agent::parse(
            &app.state::<crate::settings::SettingsState>()
                .lock()
                .map(|s| s.default_agent.clone())
                .unwrap_or_else(|_| "claude".into()),
        ),
    };
    let agent = match agent {
        Ok(a) => a,
        Err(e) => return err(&e),
    };
    if let Err(e) = validate_spawn(agent, req) {
        return err(&e);
    }
    let name = req.name.clone().unwrap_or_default();
    let args = req.agent_args.clone().unwrap_or_default();
    let use_worktree = req.worktree.unwrap_or(true);
    let h = app.clone();
    let info = match tokio::task::spawn_blocking(move || {
        session::register(&h, project, agent, &name, args, use_worktree)
    })
    .await
    {
        Ok(Ok(i)) => i,
        Ok(Err(e)) => return err(&e),
        Err(e) => return err(&e.to_string()),
    };
    let new_id = Uuid::parse_str(&info.id).expect("registered session UUID");
    // The UI must not attach (and emit its first idle event) until the run exists.
    let created = (|| -> Result<Option<Run>, String> {
        let collab = app.state::<CollabState>();
        let map = sessions.lock().map_err(|e| e.to_string())?;
        let source = map
            .get(&req.source)
            .ok_or("source session closed during spawn")?;
        if source.token.is_empty() || !ct_eq(source.token.as_bytes(), req.token.as_bytes()) {
            return Err("unauthorized".into());
        }
        if let Some(text) = req.text.as_deref() {
            let mut c = collab.lock().map_err(|e| e.to_string())?;
            let r = collab::request(&mut c, req.source, new_id, text, collab::now(), &|id| {
                session::lookup(&map, id)
            })?;
            collab::save(&c);
            Ok(Some(r))
        } else {
            Ok(None)
        }
    })();
    let run = match created {
        Ok(run) => run,
        Err(e) => {
            let h = app.clone();
            return match tokio::task::spawn_blocking(move || session::close_one(&h, new_id)).await {
                Ok(Ok(session::CloseOutcome::Removed)) => err(&e),
                Ok(Ok(session::CloseOutcome::Kept { path })) => {
                    err(&format!("{e}; worktree kept at {path}"))
                }
                Ok(Err(cleanup)) => err(&format!("{e}; cleanup failed: {cleanup}")),
                Err(cleanup) => err(&format!("{e}; cleanup failed: {cleanup}")),
            };
        }
    };
    let _ = app.emit("session-spawned", &info);
    if let Some(r) = &run {
        collab::changed(app, &app.state::<CollabNotify>().0, &[r.project_id]);
    }
    json!({ "session": info, "run": run })
}

/// `session.close`: tear down a sibling session in the caller's project.
pub async fn close(req: &Request, app: &AppHandle) -> Value {
    let sessions = app.state::<Sessions>();
    if let Err(e) = auth(&sessions, req) {
        return e;
    }
    let Some(t) = req.target.as_deref() else {
        return err("missing target");
    };
    let id = {
        let map = match sessions.lock() {
            Ok(m) => m,
            Err(e) => return err(&e.to_string()),
        };
        match close_target(&map, req.source, t) {
            Ok(id) => id,
            Err(e) => return err(&e),
        }
    };
    let h = app.clone();
    match tokio::task::spawn_blocking(move || session::close_one(&h, id)).await {
        Ok(Ok(outcome)) => {
            let _ = app.emit("session-closed", json!({ "id": id }));
            json!({ "outcome": outcome })
        }
        Ok(Err(e)) => err(&e),
        Err(e) => err(&e.to_string()),
    }
}

type Reply = Pin<Box<dyn Future<Output = Value> + Send>>;

/// Started from `setup()`; runs for the life of the app.
pub async fn serve(app: AppHandle) {
    let socket = app.state::<Runtime>().socket.clone();
    let h = app.clone();
    let dispatch = move |req: Request| -> Reply {
        let h = h.clone();
        Box::pin(async move {
            let sessions = h.state::<Sessions>();
            let collab = h.state::<CollabState>();
            let notify = h.state::<CollabNotify>();
            if req.method == "run.wait" {
                return wait(&req, &sessions, &collab, &notify.0).await;
            }
            if req.method == "session.spawn" {
                return spawn(&req, &h).await;
            }
            if req.method == "session.close" {
                return close(&req, &h).await;
            }
            let (v, touched) = handle(&req, &sessions, &h.state::<Projects>(), &collab);
            if req.method == "scratchpad.write" || req.method == "scratchpad.append" {
                if let (Some(rev), Some((project, _))) = (
                    v["revision"].as_u64(),
                    sessions
                        .lock()
                        .ok()
                        .and_then(|m| session::lookup(&m, req.source)),
                ) {
                    scratchpad::emit_changed(&h, project, rev);
                }
            }
            if let Some(p) = touched {
                collab::changed(&h, &notify.0, &[p]);
                if req.method == "session.request" {
                    if let Some(id) = v["run"]["id"]
                        .as_str()
                        .and_then(|s| Uuid::parse_str(s).ok())
                    {
                        crate::deliver::on_request_created(&h, id);
                    }
                }
            } else if req.method == "run.wait.clear" {
                notify.0.notify_waiters();
            }
            v
        })
    };
    if let Err(e) = serve_on(&socket, dispatch).await {
        eprintln!("agentfleet: broker failed: {e}");
    }
}

/// Listener loop, independent of Tauri so tests can drive it.
pub async fn serve_on<F>(socket: &Path, dispatch: F) -> std::io::Result<()>
where
    F: Fn(Request) -> Reply + Clone + Send + Sync + 'static,
{
    let _ = std::fs::remove_file(socket);
    let listener = UnixListener::bind(socket)?;
    std::fs::set_permissions(socket, std::fs::Permissions::from_mode(0o600))?;
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("agentfleet: broker accept: {e}");
                continue;
            }
        };
        let d = dispatch.clone();
        tokio::spawn(async move {
            let _ = connection(stream, d).await;
        });
    }
}

async fn connection<F>(stream: UnixStream, dispatch: F) -> std::io::Result<()>
where
    F: Fn(Request) -> Reply,
{
    let (rd, mut wr) = stream.into_split();
    let mut reader = BufReader::new(rd.take(MAX_REQUEST + 1));
    let mut line = String::new();
    let resp = match tokio::time::timeout(DEADLINE, reader.read_line(&mut line)).await {
        Err(_) => err("timeout"),
        Ok(Err(e)) => err(&format!("bad request: {e}")),
        Ok(Ok(_)) if line.len() as u64 > MAX_REQUEST => err("request too large"),
        Ok(Ok(_)) => match serde_json::from_str::<Request>(line.trim_end()) {
            Ok(req) => dispatch(req).await,
            Err(e) => err(&format!("bad request: {e}")),
        },
    };
    let mut out = serde_json::to_string(&resp).unwrap_or_else(|_| "{}".into());
    out.push('\n');
    wr.write_all(out.as_bytes()).await?;
    wr.flush().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{Agent, Session};
    use crate::status::Status;
    use crate::worktree::Worktree;
    use std::path::PathBuf;
    use std::sync::Arc;

    const TOKEN: &str = "tok-abcdef";

    #[test]
    fn spawn_rejects_invalid_input_before_registration() {
        let mut req: Request = serde_json::from_value(json!({
            "method": "session.spawn", "source": Uuid::new_v4(), "token": TOKEN,
            "text": "x".repeat(collab::MAX_PROMPT + 1)
        }))
        .unwrap();
        assert!(validate_spawn(Agent::Claude, &req)
            .unwrap_err()
            .contains("prompt too large"));
        req.text = Some("task".into());
        assert!(validate_spawn(Agent::Claude, &req).is_ok());
        assert!(validate_spawn(Agent::Shell, &req).is_err());
        req.text = None;
        assert!(validate_spawn(Agent::Shell, &req).is_ok());
        req.agent_args = Some(vec!["--bad".into()]);
        assert!(validate_spawn(Agent::Shell, &req).is_err());
    }

    fn session(project: Uuid, agent: Agent) -> Session {
        let id = Uuid::new_v4();
        Session {
            id,
            id8: id.simple().to_string()[..8].to_string(),
            project_id: project,
            agent,
            repo: PathBuf::from("/r"),
            worktree: Worktree {
                repo: PathBuf::from("/r"),
                path: PathBuf::from("/w"),
                branch: "b".into(),
                base_commit: "c".into(),
            },
            pty: None,
            poller_stop: None,
            token: TOKEN.into(),
            status: Status::Unknown,
            draft_len: 0,
            name: String::new(),
            agent_args: Vec::new(),
            owns_worktree: true,
            model: String::new(),
        }
    }

    /// Two sessions in P, one in Q; returns (sessions, source id in P, ids in P).
    fn fixture() -> (Sessions, Uuid, Vec<Uuid>) {
        let (p, q) = (Uuid::new_v4(), Uuid::new_v4());
        let a = session(p, Agent::Claude);
        let b = session(p, Agent::Shell);
        let c = session(q, Agent::Claude);
        let ids = vec![a.id, b.id];
        let src = a.id;
        let sessions = Sessions::default();
        {
            let mut m = sessions.lock().unwrap();
            for s in [a, b, c] {
                m.insert(s.id, s);
            }
        }
        (sessions, src, ids)
    }

    fn req(source: Uuid, token: &str) -> Request {
        serde_json::from_value(json!({
            "method": "session.list", "source": source, "token": token, "extra": 1
        }))
        .unwrap()
    }

    #[test]
    fn ct_eq_works() {
        assert!(ct_eq(b"abc", b"abc"));
        assert!(!ct_eq(b"abc", b"abd"));
        assert!(!ct_eq(b"abc", b"abcd"));
    }

    #[test]
    fn list_filters_to_source_project() {
        let (sessions, src, mut ids) = fixture();
        let v = handle(
            &req(src, TOKEN),
            &sessions,
            &Projects::default(),
            &CollabState::default(),
        )
        .0;
        let list = v["sessions"].as_array().expect("sessions array");
        assert_eq!(list.len(), 2);
        let mut got: Vec<Uuid> = list
            .iter()
            .map(|s| {
                assert!(s["id8"].is_string() && s["agent"].is_string());
                assert_eq!(s["status"], "unknown");
                Uuid::parse_str(s["id"].as_str().unwrap()).unwrap()
            })
            .collect();
        got.sort();
        ids.sort();
        assert_eq!(got, ids);
    }

    #[test]
    fn close_target_rules() {
        let (sessions, src, ids) = fixture();
        let other = ids.iter().copied().find(|i| *i != src).unwrap();
        let m = sessions.lock().unwrap();
        let q = m
            .values()
            .find(|s| s.project_id != m[&src].project_id)
            .unwrap();
        assert!(close_target(&m, src, &q.id.to_string()).is_err());
        assert_eq!(
            close_target(&m, src, &src.to_string()),
            Err("cannot close yourself".into())
        );
        assert_eq!(close_target(&m, src, &other.to_string()), Ok(other));
        assert!(close_target(&m, src, "zzzzzzzz").is_err());
    }

    #[test]
    fn wrong_token_unauthorized() {
        let (sessions, src, _) = fixture();
        let v = handle(
            &req(src, "tok-abcdeX"),
            &sessions,
            &Projects::default(),
            &CollabState::default(),
        )
        .0;
        assert_eq!(v, json!({"error": "unauthorized"}));
    }

    #[test]
    fn unknown_source_unauthorized() {
        let (sessions, _, _) = fixture();
        let v = handle(
            &req(Uuid::new_v4(), TOKEN),
            &sessions,
            &Projects::default(),
            &CollabState::default(),
        )
        .0;
        assert_eq!(v, json!({"error": "unauthorized"}));
    }

    async fn roundtrip(path: &Path, line: String) -> Value {
        let mut s = UnixStream::connect(path).await.unwrap();
        s.write_all(line.as_bytes()).await.unwrap();
        let mut out = String::new();
        BufReader::new(s).read_line(&mut out).await.unwrap();
        serde_json::from_str(&out).unwrap()
    }

    #[tokio::test]
    async fn socket_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("broker.sock");
        let (sessions, src, _) = fixture();
        let sessions = Arc::new(sessions);
        let projects = Arc::new(Projects::default());
        let p2 = path.clone();
        tokio::spawn(async move {
            let collab = Arc::new(CollabState::default());
            serve_on(&p2, move |r: Request| -> Reply {
                let v = handle(&r, &sessions, &projects, &collab).0;
                Box::pin(async move { v })
            })
            .await
            .unwrap();
        });
        for _ in 0..100 {
            if path.exists() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);

        let ok = json!({"method": "session.list", "source": src, "token": TOKEN});
        let v = roundtrip(&path, format!("{ok}\n")).await;
        assert_eq!(v["sessions"].as_array().unwrap().len(), 2);

        let bad = json!({"method": "session.list", "source": src, "token": "nope"});
        let v = roundtrip(&path, format!("{bad}\n")).await;
        assert_eq!(v, json!({"error": "unauthorized"}));
    }
}
