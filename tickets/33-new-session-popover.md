---
id: 33
status: doing
model: sonnet
depends_on: [32]
max_attempts: 4
files:
  - src/lib/NewSessionPopover.svelte
  - src/lib/NewTabDialog.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/Sidebar.svelte
  - src/App.svelte
---
## Goal
"+" on a project row opens an anchored popover (agent segmented control, Enter creates, Esc closes); the sidebar header "+" opens the same popover with a project picker and "Add project…". The centered modal is gone.

## Context
`sessions.svelte.ts` `store` has `dialogOpen: boolean`, `preselectedProjectId: string | null`, `error: string`; `addPending(projectId, agent)` creates the tab; `addProject(path)` returns the project; `fail(key, message)` currently sets `store.error` and `store.dialogOpen = true` when `create_session` rejects. `Sidebar.svelte` header button `aria-label="Add project"` calls the folder picker directly; row button `aria-label="New session"` sets `preselectedProjectId` + `dialogOpen`. `NewTabDialog.svelte` holds the modal; `App.svelte` renders it under `{#if store.dialogOpen}`. Primitives: `src/lib/ui/Popover.svelte` (`anchor`, `bind:open`, `placement`), `SegmentedControl.svelte`, `Button.svelte`. Settings default agent: `settings.value.defaultAgent` (`loadSettings()` if `!settings.loaded`).

### Store changes
Replace `dialogOpen` + `preselectedProjectId` with
```ts
newSession: null as { anchor: HTMLElement; projectId: string | null } | null,
```
`fail(key, message)`: drop the tab as today, set `store.error = message`, do **not** reopen anything (the sidebar banner below shows it). Update every reference (`npm run check` finds them).

### Sidebar.svelte
- Header: keep the folder icon button but relabel `aria-label="Add project"` → stays; add a second header button `aria-label="New session"` (`Plus`) that opens the popover with `projectId: null`, anchor = the button element (`bind:this`). The folder-add button becomes `FolderPlus` icon.
- Row "+": `store.newSession = { anchor: e.currentTarget as HTMLElement, projectId: p.id }`.
- Error banner: when `store.error` is non-empty and `store.newSession === null`, render `<div class="banner" role="alert">{store.error} <button aria-label="Dismiss">×</button></div>` under the header (`background color-mix(in srgb, var(--danger) 15%, transparent); color var(--danger); font-size var(--text-sm); padding var(--space-1) var(--space-2)`). Dismiss clears `store.error`. (Colors here must be tokens; the rest of the sidebar restyle is ticket 35 — do not touch other styles.)
- Render `<NewSessionPopover />` once at the end of the `<aside>` (it reads `store.newSession`).

### NewSessionPopover.svelte (new)
```svelte
<Popover anchor={store.newSession?.anchor ?? null} open={store.newSession !== null} onclose={close}>
```
Content: if `projectId` is null → a row with a `<select>` of `store.projects` (default: project of `store.tabs.at(-1)` else first) and a ghost Button "Add project…" (folder picker → `addProject` → select it); if non-null → the project name as a muted caption. Below: `<SegmentedControl options=[{claude,"Claude Code"},{shell,"Shell"}] bind:value={agent} />` with `agent` initialised from `settings.value.defaultAgent` (call `loadSettings()` first if not loaded). Error line (`store.error`) in `var(--danger)` when set. Footer: `Button variant="primary"` "Create" (`autofocus`). `Enter` anywhere inside (except when the `<select>` is focused and open — just handle `keydown` on the panel root for `Enter` and call create) → create; `Escape` handled by Popover. `create()`: no project → `store.error = "choose or add a project"`; else `addPending(projectId, agent)`, `store.error = ""`, close. `close()`: `store.newSession = null`.

### Cleanup
Delete `NewTabDialog.svelte`; remove its import and `{#if store.dialogOpen}` line from `App.svelte`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`
4. `! grep -nE '#[0-9a-fA-F]{3,8}\b' src/lib/NewSessionPopover.svelte`
5. `! test -f src/lib/NewTabDialog.svelte`

## Done when
- `npm run tauri dev`: click "+" on a project row → popover appears under the button with an arrow, Create focused; press Enter → session tab appears, popover closed, focus back on "+". Esc closes without creating. Click outside closes.
- Header "+" → popover with project select + "Add project…"; choosing a folder adds it and selects it; Create works.
- Kill the agent binary path (e.g. temporarily set an invalid `worktreesRoot` with no write permission) → create fails → tab disappears and the red banner shows the message in the sidebar; dismiss clears it.
- Popover near the window bottom flips above the anchor.

## Not included
- Restyling the rest of the sidebar (35), codex agent option, remembering last agent per project.
