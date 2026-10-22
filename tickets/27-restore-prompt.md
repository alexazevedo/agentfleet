---
id: 27
status: done
model: sonnet
depends_on: [26]
max_attempts: 3
files:
  - src-tauri/src/settings.rs
  - src-tauri/src/session.rs
  - src-tauri/src/lib.rs
  - src/lib/settingsStore.svelte.ts
  - src/lib/SettingsPane.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/RestoreDialog.svelte
  - src/App.svelte
---
## Goal
On launch with saved sessions, the app asks "Restore N sessions?" (Restore / Start fresh, "Don't ask again"), honouring a `restoreSessions` setting of ask | always | never; Start fresh forgets the sessions but leaves worktrees on disk.

## Context
`settings.rs`: `Settings.restore_sessions: String` with values `"ask" | "always" | "never"`, default `"ask"`, validated in `set_settings`. Test: invalid value rejected.

`session.rs`: today `list_restorable` loads `state.json` once, inserts every surviving session into the map with `pty: None`, and returns their infos; keep that. Add `#[tauri::command] forget_restorable(sessions, projects) -> Result<usize, String>`: remove from the map every session whose `pty.is_none()`, `persist` the remaining map (projects untouched), return the count. Worktrees and branches are not touched. Register in `lib.rs`. Unit test the pure helper `fn drain_unattached(map: &mut HashMap<Uuid, Session>) -> usize`.

Frontend: split `restoreTabs()` in `sessions.svelte.ts` into `listRestorable(): Promise<SessionInfo[]>` (calls `list_restorable` once, loads projects, stores the infos in `store.pendingRestore`) and `attachRestored()` (pushes the tabs exactly as today from `store.pendingRestore`, then clears it) plus `forgetRestorable()` (invokes `forget_restorable`, clears `store.pendingRestore`). `App.svelte` `onMount`: `await loadSettings()` (ignore errors), `const infos = await listRestorable()`; if `infos.length === 0` nothing; else if `restoreSessions === 'always'` → `attachRestored()`; `'never'` → `forgetRestorable()`; `'ask'` → `store.restoreDialogOpen = true`. `RestoreDialog.svelte`: modal "Restore N sessions?" listing `<project> · <agent> <id8>` per session (project name from `store.projects`), buttons Restore / Start fresh, checkbox "Don't ask again"; on a choice with the checkbox on, `saveSettings({ ...settings.value, restoreSessions: choice === 'restore' ? 'always' : 'never' })` before acting. `SettingsPane.svelte`: a "Startup" section with a `<select>` for restoreSessions (Ask / Always restore / Never restore). Keep the update-check scheduler start after this flow so two modals don't stack on first launch (start it after the restore decision).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- Relaunch with three saved sessions → dialog lists three rows; Start fresh + Don't ask again → empty session list, worktrees still under the worktrees root, `settings.json` has `"restoreSessions": "never"`, next launch shows no dialog; switching the setting back to Ask restores the prompt.

## Not included
- Per-session selection, deleting worktrees, remembering the active tab.
