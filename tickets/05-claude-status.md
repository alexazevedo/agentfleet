---
id: 05
status: done
model: sonnet
depends_on: [04]
max_attempts: 3
files:
  - src-tauri/src/agent.rs
  - src-tauri/src/status.rs
  - src-tauri/src/session.rs
  - src-tauri/src/lib.rs
  - src/lib/sessions.svelte.ts
  - src/lib/TabBar.svelte
---
## Goal
Claude Code tab launches with auto permissions, a stable session id and status hooks; each tab shows a badge (working / waiting / ended / unknown) driven by the hook status file.

## Context
`agent.rs`: `pub fn claude_launch(session_id: Uuid, settings_path: &Path, resume: bool) -> String` returning `claude --permission-mode auto --settings <settings_path> --session-id <uuid>` (or `--resume <uuid>` instead of `--session-id` when `resume`). Verify the flag names with `claude --help` once; if `auto` is rejected, use the closest fully-automatic mode and note it in the ticket report. `pub fn write_hooks_settings(status_file: &Path, dest: &Path)` writes:

```json
{"hooks":{
 "SessionStart":[{"hooks":[{"type":"command","command":"printf waiting > STATUS"}]}],
 "UserPromptSubmit":[{"hooks":[{"type":"command","command":"printf working > STATUS"}]}],
 "PreToolUse":[{"matcher":"","hooks":[{"type":"command","command":"printf working > STATUS"}]}],
 "PermissionRequest":[{"hooks":[{"type":"command","command":"printf waiting > STATUS"}]}],
 "Stop":[{"hooks":[{"type":"command","command":"printf waiting > STATUS"}]}]
}}
```
with STATUS replaced by the shell-quoted absolute status path. Files live in `~/.agentfleet/run/<id>.status` and `<id>.settings.json`. Pass `CLAUDE_CONFIG_DIR` through untouched.

`status.rs`: `enum Status { Unknown, Working, Waiting, Ended }`, `parse(&str) -> Status`, and a tokio task per session polling the file every 200 ms, emitting Tauri event `session-status` `{ id, status }` only on change. PTY `on_exit` sets `Ended`. Unit tests: `parse` and `claude_launch` string shape. Frontend: store status per session; `TabBar.svelte` shows a coloured dot + text (shell tabs: no badge).

Integration notes from ticket 04 (read `session.rs`, `lib.rs`, `sessions.svelte.ts`, `TabBar.svelte` first): commands are `create_session({repo, agent: "claude"|"shell", cols, rows, onData})` → `SessionInfo {id, id8, agent, branch, worktreePath, repoName}` (camelCase), `write_session({id, data: number[]})`, `resize_session({id, cols, rows})`. In `create_session` the Claude placeholder is `command = Some("claude")` and the PTY `on_exit` closure only logs; replace both. Frontend store `store.tabs` = `{key, repo, agent, info?}` with `activeKey`; add a `status` field per tab. `lib.rs` carries `#[allow(dead_code)]` on `mod pty/session/worktree`; narrow or drop them as items get used. Emit status with `app.emit("session-status", payload)` from the poller task (hold an `AppHandle`), listen with `listen` from `@tauri-apps/api/event` (needs `core:event:default` in capabilities, add it).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- Claude Code tab: prompt visible within 2 s, badge "waiting"; submit a prompt → "working"; reply ends → "waiting"; `/exit` → "ended".

## Not included
- Permission-mode toggle, model/effort picker, attention chime, auto-delivery.
