---
id: 13
status: done
model: sonnet
depends_on: [12]
max_attempts: 3
files:
  - src-tauri/src/collab.rs
  - src-tauri/src/lib.rs
  - src/App.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/collab.svelte.ts
  - src/lib/Messages.svelte
  - src/lib/Sidebar.svelte
  - src/lib/Terminal.svelte
---
## Goal
The main pane shows "Terminal | Messages" for the active session; Messages lists its runs with status chips, Execute/Cancel, and a compose box to request work from another session; the sidebar shows a queued-count badge.

## Context
Backend commands (add to `collab.rs`, register in `lib.rs`): `list_runs(sessionId) -> Vec<Run>` (to or from, newest first), `send_request(from, to, text) -> Run` (human-authored, same rules as `session.request`, then `deliver::on_request_created`), plus the existing `execute_run`, `cancel_run`. Event `collab-changed { projectId }` already fires on every mutation.

Frontend: `collab.svelte.ts` store: `runsBySession: Record<string, Run[]>`, `load(sessionId)`, `initCollabListener()` (on `collab-changed` reload runs for every tab in that project), `queuedCount(sessionId)`. `Messages.svelte` props `{ tab }`: list of runs (chip colours: queued grey, running blue, completed green, cancelled/interrupted amber, needsAttention red), each row shows direction (`→ <id8>` / `← <id8>`), first line of prompt, expandable response/detail, Execute (queued, incoming) and Cancel (queued/running) buttons; compose at the bottom: `<select>` of other Claude sessions in the same project, textarea, Send on Cmd/Ctrl+Enter. `App.svelte`: a small tab strip above the main area ("Terminal", "Messages (n queued)") per active session, `store.view: 'terminal' | 'messages'`; terminals stay mounted and hidden as today; `Terminal.svelte` must still refit when switching back. `Sidebar.svelte`: a small count badge next to the status dot when `queuedCount > 0`.

Integration notes from tickets 11–12 (read `collab.rs`, `deliver.rs`, `broker.rs`, `lib.rs`, `sessions.svelte.ts`, `Sidebar.svelte`, `App.svelte`, `Terminal.svelte` first): existing commands `execute_run({id}) -> bool` and `cancel_run({id})`. Run JSON (camelCase): `id, projectId, fromSessionId, toSessionId, prompt, status, response, detail, createdAt, updatedAt`; statuses `queued|running|completed|cancelled|interrupted|needsAttention`. Event `collab-changed { projectId }` (needs `core:event:default`, already granted). For `send_request`, reuse `collab::request(...)` with the same lookup closure the broker uses, then `collab::save`, `collab::changed`, then `deliver::on_request_created(&app, run_id)`. `session::lookup(sessions, id) -> Option<(project, Agent)>` exists. Lock order: sessions → projects → collab.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`
3. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
4. `cd src-tauri && cargo test`

## Done when
- Compose from A's Messages tab to B → run appears in both tabs, B's sidebar badge shows 1, B's terminal receives it when idle (or on Execute), B's `agentfleet reply` flips the chip to completed with the response visible.
- Switching Terminal ↔ Messages keeps terminal content and size.

## Not included
- Run search/filter, markdown rendering, notifications/chime, scratchpad row (14).
