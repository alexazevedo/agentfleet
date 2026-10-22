---
id: 28
status: done
model: sonnet
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/session.rs
  - src-tauri/src/lib.rs
---
## Goal
A Tauri command `remove_project_cascade(id)` closes every session of a project through the existing close path, then removes the project, and is unit-tested for ordering and mid-way failure.

## Context
`session.rs` today: `close_session(id, sessions, projects, app) -> Result<CloseOutcome, String>` (line ~450) removes the session from the map under lock, persists, interrupts collab runs, stops the poller, kills the PTY, deletes `~/.agentfleet/run/{uuid}.status|settings.json`, then removes the worktree when `worktree::is_clean` (dirty → `CloseOutcome::Kept { path }`). Lock order everywhere: sessions, then projects.

Refactor: move the body of `close_session` into `pub fn close_one(app: &AppHandle, uuid: Uuid) -> Result<CloseOutcome, String>` (take `Sessions`/`Projects` via `app.state::<_>()`); `close_session` becomes parse + `close_one`. Behaviour unchanged.

Add a pure helper and test it with closures:

```rust
#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClosedSession { pub id: String, pub outcome: CloseOutcome }

/// Closes `ids` in order; stops at the first error (already-closed ones stay closed).
/// Only when every close succeeded does `finish` run (the project removal).
pub fn cascade(
    ids: Vec<Uuid>,
    mut close: impl FnMut(Uuid) -> Result<CloseOutcome, String>,
    finish: impl FnOnce() -> Result<(), String>,
) -> Result<Vec<ClosedSession>, String>
```

Then `#[tauri::command] pub fn remove_project_cascade(id: String, app: AppHandle) -> Result<Vec<ClosedSession>, String>`: parse uuid; collect (lock sessions, release) the ids of sessions with `project_id == uuid`, sorted by `id8` for determinism; call `cascade(ids, |sid| close_one(&app, sid), || { lock sessions then projects; let has = any session still has project_id == uuid; project::remove_from(&mut list, uuid, has)?; persist(&map, &list); Ok(()) })`. Unknown project id → `Err("no such project")` before closing anything. Register the command in `lib.rs` next to `project::remove_project` (keep that one; it is unchanged).

Tests in `session.rs` `mod tests` (pure, no PTY): (1) three ids all `Ok(Removed)` → finish called once, result has three entries in input order; (2) second close returns `Err` → result is that error, close called exactly twice, finish never called; (3) a `Kept { path }` outcome is not an error: finish still runs and the entry carries the path.

`CloseOutcome` already derives `Serialize, Debug, PartialEq`; add `Clone` if the tests need it.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`

## Done when
- `cargo test` passes with the three new cascade tests plus every existing test (46 today).
- `close_session` still returns the same `CloseOutcome` JSON shape (`{"kind":"removed"}` / `{"kind":"kept","path":...}`); no frontend change needed for it.

## Not included
- Any frontend change; deleting dirty worktrees, branches, the repo folder, or scratchpad files; changes to `project::remove_project`.
