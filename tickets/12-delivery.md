---
id: 12
status: done
model: sonnet
depends_on: [11]
max_attempts: 3
files:
  - src-tauri/src/collab.rs
  - src-tauri/src/deliver.rs
  - src-tauri/src/session.rs
  - src-tauri/src/status.rs
  - src-tauri/src/lib.rs
---
## Goal
A queued request is pasted into the recipient's terminal by itself when that session is idle with nothing typed; Execute forces it; closing a session interrupts its runs; dead targets flag needsAttention.

## Context
Draft tracking (backend shortcut, documented): `Session.draft_len: usize` updated in `write_session`: `\r` or `\n` → 0; `0x7f`/`0x08` → saturating −1; other bytes +1 each. Status tracking: `Session.status` updated whenever `status::emit` runs (make `emit` take the `Sessions` state via `AppHandle` and set it, then call `deliver::on_status(app, id, status)`).

`deliver.rs`: `pub fn try_deliver(app, run_id, force: bool) -> Result<bool, String>`: locks sessions then collab (lock order: sessions, projects, collab); the run must be queued; the target must have a PTY, `agent == Claude`, and (unless `force`) `status == Waiting && draft_len == 0`; writes `ESC[200~` + prompt (CR/LF normalised to `\n`) + `ESC[201~` + `\r` to the PTY, sets the run `running`, saves, emits `collab-changed`. Target without PTY or not Claude → `needs_attention` with `detail`. `pub fn on_status(app, session, status)`: on `Waiting` deliver the oldest queued run targeting it; on `Ended` mark its queued/running runs `needs_attention` ("session ended"). `pub fn on_request_created(app, run)` → `try_deliver(.., false)`; call it from the broker after `session.request`. `close_session` calls `collab::interrupt_all_for(id)` then persists.

Tauri commands (frontend wiring is ticket 13): `execute_run(id)` → `try_deliver(.., true)`, `cancel_run(id)` (by the human; allowed from either side). Unit tests: the pure delivery decision `fn can_deliver(status, draft_len, agent, has_pty, force) -> Decision { Deliver, Wait, NeedsAttention(&str) }` and the paste encoding.

Integration notes from tickets 10–11 (read `collab.rs`, `broker.rs`, `session.rs`, `status.rs`, `lib.rs` first): `broker::handle(req, sessions, projects, collab) -> (Value, Option<Uuid /*project*/>)` saves itself; caller emits via `collab::changed(&AppHandle, &Notify, &[project_ids])`. State: `app.state::<CollabState>()` (Mutex<Collab>) and `app.state::<CollabNotify>().0` (Arc<Notify>). Helpers: `collab::interrupt_all_for(&mut Collab, session, now) -> Vec<Uuid>` (project ids), `collab::save(&Collab)`, `collab::now()`. `Session.status` exists but is never written; `status::emit(app, id, status)` is the single place to hook. The broker handles `session.request` in `handle` — call `deliver::on_request_created` from the async dispatch in `serve` after a successful request (the run id is in the returned Value). Lock order: sessions → projects → collab; never hold the sessions lock while writing to a PTY that may block — `PtyHandle::write` is a short pipe write, acceptable under the lock for ≤256 KB.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- With B idle at its Claude prompt, `agentfleet send <B> "say hi"` from A makes the text appear and submit in B's terminal within a second; B's run shows running; after B runs `agentfleet reply`, A's `wait` returns.
- Typing a partial line in B (no Enter) holds the run queued until B's next idle after it is submitted.
- Closing B while a run is running flips it to interrupted.

## Not included
- Attention chime, auto-delivery toggle (M3), re-delivery on failure, UI.
