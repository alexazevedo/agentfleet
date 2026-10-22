---
id: 07
status: done
model: sonnet
depends_on: [06]
max_attempts: 3
files:
  - src-tauri/src/state.rs
  - src-tauri/src/session.rs
  - src-tauri/src/agent.rs
  - src-tauri/src/lib.rs
  - src/App.svelte
  - src/lib/sessions.svelte.ts
---
## Goal
Sessions persist to `~/.agentfleet/state.json`; on relaunch every tab comes back with its worktree reattached and its PTY relaunched, Claude Code tabs resuming their conversation.

## Context
`state.rs`: `struct PersistedState { version: 1, sessions: Vec<PersistedSession> }`, `PersistedSession { id, id8, agent, repo, worktree_path, branch, base_commit }`. `save(&PersistedState, path)` writes to `path.tmp` then renames (0600). `load(path) -> Result<PersistedState>`; missing file → empty state; unparsable or wrong version → return error and start empty (log it). Save after every create and close. Unit tests: round trip; missing file; bad JSON.

Restore: on app start the backend loads state and exposes `list_restorable() -> Vec<SessionInfo>`; the frontend mounts a Terminal per entry and calls `attach_session(id, cols, rows, on_data)` which, for each entry, checks `worktree::exists` (missing → drop entry, save) and spawns the PTY: Shell → `None`; Claude → `agent::claude_launch(id, settings, resume = true)`, i.e. `claude --resume <uuid> …`, after rewriting the hooks settings file. If Claude exits immediately (no such session) the tab shows status "ended" and the user closes it. Keep the status poller behaviour from ticket 05.

Integration notes from tickets 04–06 (read `session.rs`, `agent.rs`, `status.rs`, `lib.rs`, `sessions.svelte.ts`, `Terminal.svelte`, `App.svelte` first): `Session { id, id8, agent, repo, worktree: worktree::Worktree, pty: Option<PtyHandle>, poller_stop: Option<watch::Sender<bool>> }`. Save state after the `insert` in `create_session` and after the map removal in `close_session`. Claude launch = `agent::claude_launch(id, settings_path, resume)`; hooks file via `agent::write_hooks_settings(status_file, dest)`; poller via `status::spawn_poller(app, id, status_file, rx)`. Factor the "spawn PTY + poller for an existing Session" part of `create_session` into a shared helper so `attach_session` reuses it. Frontend tabs are `{key, repo, agent, info?, status?, closing?, keptPath?}`; `Terminal.svelte` currently calls `create_session` on mount for pending tabs — add a `restore` path that calls `attach_session` instead when the tab already has `info`. `worktree` and `session` modules still carry a commented `#[allow(dead_code)]` in `lib.rs`; `exists` and `base_commit` get used here, so drop them if clippy allows.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- Open two tabs (Claude + Shell), quit, `npm run tauri dev` again → both tabs reappear, Shell prompt is live in the same worktree, Claude shows the resumed conversation.
- `~/.agentfleet/state.json` is valid JSON with `"version": 1`.

## Not included
- Migration from other formats, read-only mode on corrupt state, Resume / Start New choice dialog.
