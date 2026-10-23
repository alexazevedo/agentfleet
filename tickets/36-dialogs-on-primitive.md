---
id: 36
status: todo
model: haiku
depends_on: [32]
max_attempts: 4
files:
  - src/lib/RemoveProjectDialog.svelte
  - src/lib/RestoreDialog.svelte
  - src/lib/UpdateDialog.svelte
---
## Goal
The three modals render through `src/lib/ui/Dialog.svelte` + `Button.svelte` with identical behavior.

## Context
Each file today has its own `.backdrop`/`.overlay` + `.dialog` markup and hex styles. `Dialog` props: `open` (`bind:open`), `title`, `width`, `dismissible`, `onclose`, `children`, `footer` snippet. Buttons: `variant` default/primary/danger, `disabled`.

- **RemoveProjectDialog**: `open = store.removeProjectId !== null`; `onclose` → `store.removeProjectId = null` unless `busy` (`dismissible={!busy}`); title `Remove {name}?`; body paragraphs unchanged; error paragraph `color var(--danger)` `role="alert"`; footer: Cancel (default) + Remove (danger, `autofocus`). Remove the `<svelte:window onkeydown>` (Dialog handles Escape).
- **RestoreDialog**: `open = store.restoreDialogOpen`, `dismissible={false}` (a choice is required); title unchanged; list + "Don't ask again" (use `Switch` with label snippet); footer: Start fresh (default) + Restore (primary, `autofocus`).
- **UpdateDialog**: `open = !!updater.available`, `dismissible={!busy}`, `onclose` → `dismissUpdate()`; title unchanged; `<pre class="notes">` keeps its max-height/scroll, `font-family var(--font-ui)`; progress + error lines; footer: Later (default) + Install (primary).
Remove all local backdrop/overlay/button styles. `App.svelte` mounting lines stay as they are (components still mount conditionally; `open` is true on mount).

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`
3. `! grep -nE '#[0-9a-fA-F]{3,8}\b' src/lib/RemoveProjectDialog.svelte src/lib/RestoreDialog.svelte src/lib/UpdateDialog.svelte`
4. `! grep -nE 'backdrop|overlay' src/lib/RemoveProjectDialog.svelte src/lib/RestoreDialog.svelte src/lib/UpdateDialog.svelte`

## Done when
- `npm run tauri dev`: Remove flow (ticket 29 demo) unchanged, Escape/backdrop click cancel unless busy; Restore dialog on launch cannot be dismissed by Escape; update dialog (trigger with `checkForUpdate(true)` on a build with an older version, or temporarily set `updater.available` in devtools) shows notes and installs/dismisses.

## Not included
- New dialogs, animations beyond Dialog's own, changing any dialog copy.
