---
id: 29
status: done
model: sonnet
depends_on: [28]
max_attempts: 3
files:
  - src/lib/Sidebar.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/RemoveProjectDialog.svelte
  - src/App.svelte
---
## Goal
A Remove control on each sidebar project row opens a confirmation modal; confirming closes the project's sessions and removes the project, and the sidebar, active view and state.json reflect it.

## Context
Backend (ticket 28): `invoke<ClosedSession[]>("remove_project_cascade", { id })` where `ClosedSession = { id: string; outcome: { kind: "removed" } | { kind: "kept"; path: string } }`. It closes sessions in order and stops at the first error; the project is removed only if every close succeeded. The error is the rejection string.

`sessions.svelte.ts`: `store` holds `projects`, `tabs` (`tab.projectId`, `tab.info?.id`), `activeKey` (a tab key, `"scratchpad:<projectId>"`, or `"settings"`), `view`, `dialogOpen`, `error`. `remove(key)` drops a tab and falls back to the last tab. Add `store.removeProjectId: string | null` (dialog open when non-null) and:

```ts
export async function removeProject(id: string): Promise<void>
```
It invokes the command, then on success: removes every tab with `tab.projectId === id` (splice, no `close_session` call — the backend already closed them), removes the project from `store.projects`, and if `store.activeKey` was one of those tab keys or `"scratchpad:" + id`, sets `activeKey = store.tabs.at(-1)?.key ?? null` and `view = "terminal"`. On rejection: leave store untouched and rethrow (the dialog shows the error; the user retries, and the backend skips sessions it already closed because they are gone from its map).

`Sidebar.svelte`: in `.project` row add a second button after "New session": `aria-label="Remove project"`, lucide `Trash2` size 14, class `remove`, hidden until `.project:hover` (same pattern as `.session:hover .close`). Click → `store.removeProjectId = p.id`.

`RemoveProjectDialog.svelte`: copy the `.backdrop`/`.dialog` markup and styles from `NewTabDialog.svelte`. Content: `<h3>Remove {name}?</h3>`; with `n = store.tabs.filter(t => t.projectId === id).length`: if `n > 0` the paragraph "N open session(s) will be closed. Clean worktrees are deleted, dirty ones stay on disk. The repo folder is not touched."; if `n === 0` "The repo folder is not touched." Buttons: Cancel (`store.removeProjectId = null`) and Remove (primary, red `#f85149` background). Remove: `busy = true`, `await removeProject(id)`, close dialog; on catch show `String(e)` in a `<p class="error">` inside the dialog and keep it open. Escape key = Cancel.

`App.svelte`: `{#if store.removeProjectId}<RemoveProjectDialog />{/if}` next to `NewTabDialog`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`
3. `cd src-tauri && cargo test`

## Done when
- `npm run tauri dev`: add a repo, open two sessions (make one dirty with `touch x` inside its worktree), hover the project row, click the trash icon, confirm → project and both tabs vanish, active view falls back to another tab or the empty state, clean worktree gone from the worktrees root, dirty one still there; `~/.agentfleet/state.json` has no entry for the project after relaunch.
- Cancel and Escape close the dialog without changes.

## Not included
- Undo, multi-select, keyboard shortcut, project rename/reorder, deleting scratchpad files, StatusBar toast (none exists today).
