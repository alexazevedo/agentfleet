---
id: 30
status: done
model: sonnet
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/session.rs
  - src-tauri/src/state.rs
  - src-tauri/src/agent.rs
  - src/lib/sessions.svelte.ts
  - src/lib/Terminal.svelte
  - src/lib/Sidebar.svelte
  - src/lib/StatusBar.svelte
---
## Goal
A session can be registered (worktree + registry entry, no PTY) separately from being attached, carries a persisted label (`name`) and persisted extra agent CLI args (`agent_args`) that reach the launch command, and a fresh session can be attached without `--resume`. This is the foundation ticket 31 (`agentfleet spawn`) builds on; after it the app behaves exactly as before for the user, plus the sidebar shows `name` when set.

## Context
`src-tauri/src/session.rs` today: `create_session(project_id, agent, cols, rows, on_data, …)` parses the agent, creates the worktree (`worktree::create(&repo, agent.as_str(), &id8, &root)`), builds a `Session`, calls `spawn_for(&mut session, &app, cols, rows, on_data, false)` and only then inserts into the map and persists. `attach_session(id, cols, rows, on_data)` always calls `spawn_for(.., resume = true)`. `spawn_for` builds the launch command: Shell → `exec "$SHELL" -i`; Claude → `agent::claude_launch(id, &settings, resume, auto_permission)`. `persist` writes `PersistedSession` rows (`src-tauri/src/state.rs`); `list_restorable` rebuilds `Session`s from them.

### Backend changes
1. `Session` gains `pub name: String` and `pub agent_args: Vec<String>`. `SessionInfo` gains `pub name: String` (camelCase serialization is already on). Update `info_for`, both `Session {..}` literals in `session.rs`, the fixtures in `session.rs` and `broker.rs` tests (`name: String::new(), agent_args: Vec::new()`).
2. `PersistedSession` gains `#[serde(default)] pub name: String` and `#[serde(default)] pub agent_args: Vec<String>`. Schema `version` stays 1 (additive, pre-release per docs/roadmap.md). `persist` writes them; `list_restorable` reads them back. Add a `state.rs` test: a JSON row without the two fields loads with `name == ""` and empty args; a row with them round-trips.
3. Extract `pub fn register(app: &AppHandle, project_id: Uuid, agent: Agent, name: &str, agent_args: Vec<String>) -> Result<SessionInfo, String>`: looks up the repo (lock projects, release), creates the worktree, inserts a `Session` with `pty: None` into the map, persists, returns the info. Lock order unchanged (sessions, then projects). `create_session` becomes: parse → `register` → lock sessions → `spawn_for(.., resume = false)` on the map entry → on `Err`, remove the entry from the map, persist, and return the error (no orphan registry rows). Add two optional command args `name: Option<String>`, `agent_args: Option<Vec<String>>` (Tauri maps missing JS keys to `None`), defaulting to `""` / `[]`. Also validate: `agent_args` non-empty with `Agent::Shell` → `Err("agent args need a claude session")` (message updated to "claude or codex" in ticket 32).
4. `attach_session` gains `resume: bool` (frontend passes it; see below). `spawn_for(.., resume)` unchanged in shape.
5. `agent::claude_launch` gains a trailing `extra: &[String]` parameter; each element is appended shell-quoted after the session id (`claude … --session-id <id> 'arg1' 'arg2'`). Extend `launch_flags` test: args appear quoted and in order; empty slice leaves the string unchanged. `spawn_for` passes `&session.agent_args`.

### Frontend changes
- `src/lib/sessions.svelte.ts`: `SessionInfo.name: string`; `Tab.resume?: boolean` replaces `restored?: boolean` (`attachRestored` sets `resume: true`). Export `export function labelOf(agent: AgentKind): string` returning "Claude Code" / "Shell" and `export function tabTitle(info: SessionInfo): string` returning `info.name || labelOf(info.agent)`.
- `src/lib/Terminal.svelte`: branch on `tab.info` instead of `tab.restored`; call `attach_session` with `resume: tab.resume ?? true`. The `create_session` invoke is unchanged (no name / args from the dialog yet).
- `src/lib/Sidebar.svelte` line 91 and `src/lib/StatusBar.svelte` line 10: use `tabTitle(tab.info)` / `labelOf`. Nothing else in the UI changes.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- `cargo test` passes with the new `state.rs` round-trip test and the extended `launch_flags` test plus every existing test (50 today).
- `svelte-check` reports 0 errors; `npm run build` succeeds.
- `~/.agentfleet/state.json` written by the app contains `"name"` and `"agent_args"` on every session row; an old file without them still loads (covered by the test).
- Restored tabs still attach with `--resume`; a tab created from the dialog still starts fresh.

## Not included
- Any CLI or broker change (ticket 31); the Codex agent kind (ticket 32); a name field in the New-session dialog; renaming sessions from the UI.
