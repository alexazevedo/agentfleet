---
id: 14
status: done
model: sonnet
depends_on: [13]
max_attempts: 3
files:
  - src-tauri/src/scratchpad.rs
  - src-tauri/src/broker.rs
  - src-tauri/src/cli.rs
  - src-tauri/src/lib.rs
  - src/lib/scratchpadStore.svelte.ts  (renamed: case-insensitive clash with Scratchpad.svelte)
  - src/lib/Scratchpad.svelte
  - src/lib/Sidebar.svelte
  - src/lib/sessions.svelte.ts
  - src/App.svelte
---
## Goal
One Markdown scratchpad per project with a revision counter, readable and writable from the CLI with optimistic locking and from a sidebar "Scratchpad" row that opens an editor in the main pane.

## Context
`scratchpad.rs`: file `~/.agentfleet/scratchpads/<projectId>.json` `{ revision: u64, text: String }`, 256 KB cap, tmp+rename 0600. Pure functions with tests: `read(project) -> (revision, text)` (missing → `(0, "")`), `write(project, text, if_revision: Option<u64>) -> Result<u64>` (mismatch → `Err("revision mismatch: current N")`), `append(project, text, if_revision)` (adds `\n` + text). Emit `scratchpad-changed { projectId, revision }` after writes. Broker methods `scratchpad.read` → `{ revision, text }`, `scratchpad.write {text, revision?}`, `scratchpad.append {text, revision?}`, scoped to the source session's project. CLI verbs `scratchpad read` (prints text, revision on stderr), `scratchpad write [--if-revision N] <text…|-|` (`-` reads stdin), `scratchpad append [--if-revision N] <text…|->`; mismatch exits 4. Tauri commands `read_scratchpad(projectId)`, `write_scratchpad(projectId, text, ifRevision)`.

Frontend: sidebar row "📝 Scratchpad" under each project; selecting it sets `store.activeKey = 'scratchpad:' + projectId` (tabs' terminals hide as for any inactive key). `Scratchpad.svelte` props `{ projectId }`: loads on mount and on `scratchpad-changed` (if not dirty), textarea, revision shown, Save calls `write_scratchpad` with the loaded revision and shows the mismatch error inline with a Reload button. `App.svelte` renders it when the active key starts with `scratchpad:`; status bar may show "Scratchpad · rev N" (optional).

Integration notes from tickets 10–13 (read `broker.rs`, `cli.rs`, `collab.rs` for the method/verb/command patterns, `state.rs` for tmp+rename 0600, `App.svelte`, `Sidebar.svelte`, `sessions.svelte.ts`, `collab.svelte.ts` first): the main pane is a 28 px `.strip` over a `.pane`; terminals stay mounted with `active = tab.key === store.activeKey && store.view === 'terminal'`; `store.view: 'terminal' | 'messages'` — extend the union with `'scratchpad'` and set `store.activeKey = 'scratchpad:<projectId>'` + `store.view = 'scratchpad'` from the sidebar row (no tab key matches, so no terminal is active; clicking a session row must reset `view` to `'terminal'`). Broker `handle(req, sessions, projects, collab) -> (Value, Option<Uuid>)` is sync; add the three scratchpad methods there (they need the source session's project id via `session::lookup`). `cli::call(method, extra)` and `parse`/`Cmd` are in `cli.rs`. Emit `scratchpad-changed` with `app.emit` from the Tauri command and from the broker's async dispatch (mirror how `collab-changed` is emitted there).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- `agentfleet scratchpad append "note from A"` in a session → the sidebar viewer shows it within a second; editing in the viewer and saving bumps the revision; a CLI `write --if-revision <stale>` exits 4 with the mismatch message.

## Not included
- History/diff, per-session scratchpads, conflict merge UI.
