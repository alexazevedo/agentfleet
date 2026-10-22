---
id: 11
status: done
model: sonnet
depends_on: [10]
max_attempts: 3
files:
  - src-tauri/src/collab.rs
  - src-tauri/src/broker.rs
  - src-tauri/src/cli.rs
  - src-tauri/src/lib.rs
  - src-tauri/src/session.rs
---
## Goal
Agents exchange requests: `agentfleet send`, `inbox`, `read`, `reply`, `cancel`, `wait` work end to end over the broker, persisted in `~/.agentfleet/collaboration.json`, without any automatic delivery yet.

## Context
`collab.rs`: `#[derive(Serialize, Deserialize, Clone)] #[serde(rename_all = "camelCase")] pub struct Run { id: Uuid, project_id: Uuid, from_session_id: Uuid, to_session_id: Uuid, prompt: String, status: RunStatus, response: Option<String>, detail: Option<String>, created_at: u64 /* unix seconds */, updated_at: u64 }`, `RunStatus { Queued, Running, Completed, Cancelled, Interrupted, NeedsAttention }` (lowercase/camelCase serde). `pub struct Collab { runs: Vec<Run>, waiting_on: HashMap<Uuid /*session*/, Uuid /*run*/>, notify: tokio::sync::Notify }` behind `pub type CollabState = Mutex<Collab>` (managed). Persistence: `~/.agentfleet/collaboration.json` `{ version: 1, runs }`, same tmp+rename 0600 pattern as `state.rs`; load at startup (unreadable → empty, logged); `prune()` keeps ≤ 500 runs and drops terminal runs older than 30 days; save after every mutation. Pure functions, each unit-tested: `request(from, to, prompt, lookup: &dyn Fn(Uuid) -> Option<(project, agent)>) -> Result<Run>` (same project, target is `claude`, prompt ≤ 256 KB, from ≠ to), `reply(run, from, text ≤ 128 KB)` (only the target, only while queued/running → completed), `cancel(run, by)` (sender or target, only queued/running → cancelled), `interrupt_all_for(session)` (queued/running runs to or from it → interrupted; ticket 12's close path calls it), `begin_wait(session, run) -> Result<()>` with cycle detection (walk `waiting_on` from the run's target; if it reaches `session` → `Err("wait cycle")`), `clear_wait(session)`, `prune`.

`broker.rs` methods: `session.request {target,text}` → `{ run }`; `session.inbox` → `{ runs }` (to or from source, newest first, max 100); `run.read {requestId}` → `{ run }` (participants only); `run.reply {requestId,text}`; `run.cancel {requestId}`; `run.wait {requestId}` → registers `begin_wait`, then awaits `notify` until the run is terminal or 25 s pass, returns `{ run, timedOut: bool }`, then `clear_wait`; `run.wait.clear`. Every mutation calls `notify.notify_waiters()` and emits Tauri event `collab-changed` `{ projectId }`.

`cli.rs` verbs: `send <session> <text…>` (session = id8 or full id, resolved via `session.list` first; prints the run id), `inbox` (table: id8-of-run  status  from→to  first 60 chars of prompt), `read <run>` (prints prompt, status, response), `reply <run> <text…>`, `cancel <run>`, `wait <run>` (loops `run.wait` until terminal, prints the response or the terminal status; exit 1 on cancelled/interrupted/needsAttention, 3 on wait cycle). Run ids accept an 8-char prefix.

`session.rs`: add `pub fn lookup(sessions, id) -> Option<(Uuid /*project*/, Agent)>` and accept id8 prefixes in a `resolve_id(sessions, s) -> Option<Uuid>` helper used by the broker.

Integration notes from ticket 10 (read `broker.rs`, `cli.rs`, `runtime.rs`, `session.rs` first): `broker::Request { method, source: Uuid, token, target, text, request_id, before: Option<String>, revision: Option<u64> }` (camelCase JSON); `broker::handle(req, sessions, projects) -> Value` is sync and AppHandle-free — keep it that way for pure methods and add a second entry point (e.g. `handle_with_app(app, req)`) for methods that must emit events or await (`run.wait`); `serve_on(socket, dispatch: F)` takes a `Fn(&Request) -> Value` closure, so an async `run.wait` needs the dispatch to become async (change `F` to return a boxed future, or spawn the wait inside and block the connection task on it). `cli::call(method, extra: serde_json::Map) -> Result<Value, String>` already maps `{error}` to `Err`. `Session` has `token` and `status` fields; `Session.status` is never updated yet (ticket 12). Lock order: sessions → projects → collab.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- From session A: `agentfleet send <B> "ping"` prints a run id; in B `agentfleet inbox` lists it queued; `agentfleet reply <run> "pong"` in B; `agentfleet wait <run>` in A prints `pong` and exits 0.
- `agentfleet send` to a session in another project or to a Shell session is refused with a clear error.
- Relaunching the app keeps the run history (collaboration.json present, `version: 1`).

## Not included
- Auto-delivery, Execute, draft tracking, status-driven transitions (ticket 12); UI (13); scratchpad (14).
