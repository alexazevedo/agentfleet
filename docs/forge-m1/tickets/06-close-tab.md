---
id: 06
status: done
model: sonnet
depends_on: [05]
max_attempts: 3
files:
  - src-tauri/src/session.rs
  - src-tauri/src/lib.rs
  - src/lib/sessions.svelte.ts
  - src/lib/TabBar.svelte
---
## Goal
Closing a tab kills its PTY and removes the worktree and branch when clean; a dirty worktree is kept and the tab shows "kept" briefly before disappearing.

## Context
Tauri command `close_session(id) -> Result<CloseOutcome, String>` where `CloseOutcome = Removed | Kept { path }`. Order: kill PTY, stop status poller, delete `~/.agentfleet/run/<id>.*`, then `worktree::is_clean` → `remove` (Removed) or leave untouched (Kept). If `worktree::remove` fails for any git reason, also return Kept (path plus the git error message) rather than surfacing a failure. Tab bar: each tab has an "×"; on Kept, show badge "kept" for 3 s with the worktree path as tooltip, then drop the tab from the store. Add a unit test in `session.rs` for the decision helper `fn close_decision(clean: bool, remove_result) -> CloseOutcome`.

Integration notes from ticket 05 (read `session.rs`, `status.rs`, `sessions.svelte.ts`, `TabBar.svelte` first): `Session.poller_stop: Option<watch::Sender<bool>>` — stop the poller with `.send(true)`. Run files: `~/.agentfleet/run/<id>.status` and `<id>.settings.json`. PTY `on_exit` already sends the stop flag then emits `session-status` `ended`; `close_session` must kill the PTY via `pty::PtyHandle::kill(&mut self)` (take the handle out of the map entry first so the mutex is not held across `kill`, which joins the reader thread). Frontend tabs are `{key, repo, agent, info?, status?}`; add a transient `closing: 'kept' | null` style field and a `remove(key)` helper.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- Close a tab with no changes → `git worktree list` and `git branch` in the repo no longer show it.
- Touch a file in the worktree, close → tab shows "kept", worktree and branch remain.

## Not included
- "Remove anyway" option, confirmation dialog, quit-while-running guard.
