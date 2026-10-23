---
id: 34
status: doing
model: sonnet
depends_on: [32]
max_attempts: 4
files:
  - src/lib/SettingsPane.svelte
  - src/lib/settingsStore.svelte.ts
  - src/lib/settingsStore.test.ts
  - src/lib/settings/GeneralTab.svelte
  - src/lib/settings/TerminalTab.svelte
  - src/lib/settings/UpdatesTab.svelte
  - src/lib/settings/AboutTab.svelte
  - src/lib/settings/Field.svelte
---
## Goal
Settings is four tabs (General · Terminal · Updates · About), every control saves on change with inline error + revert, the last tab is remembered, terminal font changes apply live, and there is no Save button.

## Context
`SettingsPane.svelte` today: local `form` clone, one `save()` → `saveSettings(snapshot)`, which `invoke("set_settings")`s and assigns the returned value into `settings.value` only on success (so on rejection `settings.value` is still the last good state). Backend validation messages (exact): `"worktrees root must be an absolute path"`, `"unknown agent: …"`, `"restore sessions must be ask, always or never"`, `"font size must be 9–24"`, `"line height must be 1.0–1.6"`, `"theme must be system, light or dark"` (ticket 30). `updater.svelte.ts` exports `checkForUpdate(manual)`, `updater.checking/error/lastResult`. `appInfo()` returns `{ version, commit }`. `Terminal.svelte` already reacts to `settings.value.terminal` (font) and `theme.resolved` (ticket 30), so live apply needs nothing there. Primitives: `Tabs`, `Switch`, `SegmentedControl`, `Button` from `src/lib/ui/`. Theme setting from ticket 30: `settings.value.theme`.

### settingsStore.svelte.ts
Add
```ts
/** Deep-merge `patch` into a snapshot of the current settings and save. Resolves "" on success, the backend message on failure; settings.value is untouched on failure. Never throws. */
export async function updateSettings(patch: DeepPartial<Settings>): Promise<string>
```
(`DeepPartial` local type; merge one level deep for `terminal`.) Serialise concurrent calls with a promise chain so two quick edits can't race.

### `settingsStore.test.ts`
`vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }))`. Cases: success → resolves `""` and `settings.value.terminal.fontSize` updated from the mocked return; rejection (`invoke` rejects `"font size must be 9–24"`) → resolves that string and `settings.value` unchanged; two overlapping calls → `invoke` called twice in order with the second payload including the first's change.

### `Field.svelte` (layout helper)
Props: `label`, `hint?`, `error?`, `children`. Row layout: label column 160px (`color var(--text)`), control column grows; `hint` in `var(--text-muted) var(--text-xs)` under the control; `error` in `var(--danger) var(--text-xs)` with `role="alert"`.

### Tabs (each a component receiving nothing; they read `settings.value` and call `updateSettings`)
- **General**: Worktrees root (text input + ghost Button "Browse…" using `@tauri-apps/plugin-dialog` `open`), commit on blur/Enter; Default agent (SegmentedControl claude/shell); Theme (SegmentedControl system/light/dark); Claude auto-permission (Switch); Auto-deliver queued requests (Switch); Confirm before quitting (Switch); Restore sessions on launch (SegmentedControl ask/always/never, labels "Ask", "Always", "Never").
- **Terminal**: Font family (text, placeholder "system monospace", commit on blur/Enter); Font size (number 9–24); Line height (number 1.0–1.6 step .05); Cursor blink (Switch). Hint under font size: "Applies to open terminals immediately."
- **Updates**: Check for updates automatically (Switch); Button "Check now" (disabled while `updater.checking`), result text `updater.lastResult` or `updater.error` beside it.
- **About**: version + commit in `var(--font-mono)`, muted; app name "agentfleet".

Commit rule for text/number fields: local `draft` initialised from `settings.value`; on blur or Enter call `updateSettings`; if the returned message is non-empty → show it in the Field `error` and reset `draft` to `settings.value.*`; else clear error. Switch/SegmentedControl call `updateSettings` directly in `onchange`; on error show it under the control and the control re-renders from `settings.value` (already reverted since the store never changed). Number inputs are parsed with `Number()`; NaN → error "enter a number", no save.

### SettingsPane.svelte
Wraps `Tabs` with `tabs = [general, terminal, updates, about]`; `active` read from `localStorage["agentfleet.settingsTab"]` on mount (try/catch), written on change. Panel padding `var(--space-4)`, max content width 560px, `background var(--bg)`. Calls `loadSettings()` on mount if not loaded; shows load error in `var(--danger)`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`
4. `! grep -nE '#[0-9a-fA-F]{3,8}\b' src/lib/SettingsPane.svelte src/lib/settings/*.svelte`
5. `! grep -n "Save" src/lib/SettingsPane.svelte src/lib/settings/*.svelte`

## Done when
- `npm run tauri dev`: open Settings → four tabs; pick Terminal, change font size to 16 with a terminal open in another tab → switch back, terminal is bigger; relaunch → Settings opens on Terminal tab; `~/.agentfleet/settings.json` has `fontSize: 16`.
- Enter 30 in font size → inline "font size must be 9–24", field reverts to 16.
- General → Theme → Dark while macOS is light: app goes dark immediately; System restores.
- No Save button anywhere; `Check now` on Updates works as before.

## Not included
- Reset to defaults, import/export, search, keyboard shortcut to open Settings, restyling the sidebar Settings row (35).
