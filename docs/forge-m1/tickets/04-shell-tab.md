---
id: 04
status: done
model: sonnet
depends_on: [02, 03]
max_attempts: 3
files:
  - src-tauri/src/session.rs
  - src-tauri/src/lib.rs
  - src-tauri/Cargo.toml
  - src-tauri/capabilities/default.json
  - package.json
  - src/App.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/TabBar.svelte
  - src/lib/NewTabDialog.svelte
  - src/lib/Terminal.svelte
---
## Goal
End-to-end Shell tab: "+" → pick repo folder → pick agent (Claude Code | Shell) → worktree created → login shell PTY in it → live xterm in the tab; multiple independent tabs; resize works; refusals surface as an error.

## Context
Backend `session.rs`: `struct Session { id: Uuid, id8: String, agent: Agent /* Claude | Shell */, repo: PathBuf, worktree: worktree::Worktree, pty: Option<pty::PtyHandle> }` in `tauri::State<Mutex<HashMap<Uuid, Session>>>`. Worktree root = `platform::home_dir()/.agentfleet/worktrees`. Tauri commands:

- `create_session(repo: String, agent: String, cols: u16, rows: u16, on_data: Channel<InvokeResponseBody>) -> Result<SessionInfo, String>`: create worktree (map `WorktreeError` to a readable string), spawn PTY (`platform::login_shell()`, cwd = worktree path, `command = None` for Shell; for Claude run `claude` for now — ticket 05 replaces this). Output chunks go `on_data.send(InvokeResponseBody::Raw(chunk))`. Returns `{ id, id8, agent, branch, worktreePath, repoName }`.
- `write_session(id, data: Vec<u8>)`, `resize_session(id, cols, rows)`.
Add `tauri-plugin-dialog` (Rust + `@tauri-apps/plugin-dialog`) and `@xterm/xterm`, `@xterm/addon-fit`. Capability file allows `dialog:allow-open` and `core:default`.

Frontend: `sessions.svelte.ts` = runes store of `SessionInfo[]` + active id. `NewTabDialog.svelte`: folder via `open({ directory: true })`, radio Claude Code / Shell, Create. `Terminal.svelte`: one xterm per session, `new Channel<ArrayBuffer>()` → `term.write(new Uint8Array(buf))`, `onData` → `write_session`, `FitAddon` + `ResizeObserver` → `resize_session`. Hidden tabs stay mounted (`display:none`), so switching does not lose output. Errors from `create_session` show inline in the dialog.

Integration notes from tickets 02/03 (already implemented, read the two modules): `worktree::WorktreeError` has no `Display`, map each variant to a message in `session.rs` (`NotARepo` → "not a git repository", `NoCommits` → "repository has no commits", `BranchExists(b)` → "branch <b> already exists", `Git(m)`/`Io(e)` → the message). `pty::PtyHandle` is `Send` but not `Sync`; `write(&self)`, `resize(&self)`, `kill(&mut self)`. `pty::spawn` takes `on_exit: Box<dyn Fn(i32) + Send>`; for this ticket just log the exit code (ticket 05 wires it to status). Both modules carry `#![allow(dead_code)]`; remove those attributes now that they are consumed, and remove `#[allow(dead_code)]`/`#[allow(unused_imports)]` in `lib.rs`/`platform/mod.rs` if clippy allows.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- `npm run tauri dev`: "+" → repo → Shell → tab appears with a live prompt in `~/.agentfleet/worktrees/<repo>/<id8>` on branch `agentfleet/shell-<id8>`; `git branch` in that terminal confirms.
- Second tab on same repo gets its own worktree; typing in one does not affect the other; window resize refits both.
- Picking a non-git folder shows "not a git repository" in the dialog.

## Not included
- Status badge, hooks, Claude flags (ticket 05). Close/cleanup (06). Persistence (07).
