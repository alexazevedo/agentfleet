---
id: 35
status: done
model: sonnet
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/worktree.rs
  - src-tauri/src/session.rs
  - src-tauri/src/state.rs
  - src-tauri/src/broker.rs
---
## Goal
Sessions can be created without a git worktree: they run in the project root on the current branch, and close never touches the checkout.

## Context
Today `session::register` (src-tauri/src/session.rs) always calls `worktree::create`. Add `use_worktree: bool`:

- `worktree.rs`: new `pub fn current(repo: &Path) -> Result<Worktree, WorktreeError>` — validates repo (same checks as `create`), returns `Worktree { repo, path: repo.to_path_buf(), branch: <symbolic-ref --short HEAD, or short HEAD commit if detached>, base_commit: HEAD }`. No branch created, nothing checked out.
- `Session` gains `pub owns_worktree: bool`. `register` takes `use_worktree: bool`; false → `worktree::current`, true → `worktree::create` as today.
- `close_one`: when `!owns_worktree`, skip `is_clean`/`remove` entirely and return `CloseOutcome::Removed` (tab gone, checkout untouched).
- `state.rs`: `PersistedSession` gains `#[serde(default = "default_true")] pub owns_worktree: bool` (old rows = true). Persist/restore it in `session::persist` and `list_restorable` (restore skips the `worktree::exists` drop-check only in that it must still pass: for no-worktree rows `exists` is true because path is the repo root — keep the check as-is).
- `create_session` Tauri command gains `worktree: Option<bool>` (default true), forwarded to `register`.
- `broker.rs`: `Request` gains `pub worktree: Option<bool>` (camelCase via existing rename); `spawn` forwards `req.worktree.unwrap_or(true)` to `register`.

`SessionInfo` is unchanged (branch now reports the current branch for no-worktree sessions).

## Checks (run in order — cheapest first)
1. `cargo test --manifest-path src-tauri/Cargo.toml`

## Done when
- A Rust test creates a no-worktree session via `register(..., use_worktree=false)` against a temp repo: worktree path == repo root, no `agentfleet/*` branch created, close returns Removed with a dirty file present and the file survives.
- Old state.json rows without `owns_worktree` load with it true (test like `old_rows_default_name_and_args`).

## Not included
- CLI flag (ticket 36), UI (ticket 37), model field (ticket 38).
- Any limit on how many no-worktree sessions share the root.
