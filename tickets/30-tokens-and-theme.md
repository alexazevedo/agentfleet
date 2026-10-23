---
id: 30
status: done
model: sonnet
depends_on: []
max_attempts: 4
files:
  - src/lib/ui/tokens.css
  - src/app.css
  - src/main.ts
  - src/lib/theme.svelte.ts
  - src/lib/settingsStore.svelte.ts
  - src/lib/Terminal.svelte
  - src/App.svelte
  - src-tauri/src/settings.rs
---
## Goal
One token layer drives every color in the app; the OS appearance picks light or dark; a persisted `theme` setting (`system | light | dark`) overrides it; xterm follows. Verified by editing `~/.agentfleet/settings.json` (the Settings UI for it comes in ticket 34).

## Context
`src/app.css` is 11 lines (hardcoded `#1e1e1e` / `#ddd`). `src/main.ts` imports it first. `settingsStore.svelte.ts` holds `settings.value: Settings` loaded via `invoke("get_settings")`. `Terminal.svelte` builds an xterm `Terminal` in `onMount` and already has a `$effect` that pushes font settings into `term.options`. `App.svelte` has an `onMount` that calls `loadSettings()`.

### `src/lib/ui/tokens.css` (new)
Three blocks. Light on `:root`; dark under `@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {…} }` and again under `:root[data-theme="dark"] {…}`. Set `color-scheme: light` / `dark` in each so native form controls follow. Exact names (values are the spec; keep them):

```
--font-ui: system-ui, -apple-system, "Segoe UI", sans-serif;
--font-mono: ui-monospace, Menlo, Consolas, monospace;
--text-xs: 11px; --text-sm: 12px; --text-md: 13px; --text-lg: 15px;
--space-1: 4px; --space-2: 8px; --space-3: 12px; --space-4: 16px; --space-5: 24px;
--radius-sm: 4px; --radius-md: 6px; --radius-lg: 10px;
--shadow-popover: 0 8px 24px rgba(0,0,0,.18);
--focus-ring: 0 0 0 3px color-mix(in srgb, var(--accent) 40%, transparent);
```
Light: `--bg:#f5f5f7 --bg-sidebar:#ececef --bg-surface:#ffffff --bg-strip:#e8e8ec --bg-hover:rgba(0,0,0,.05) --bg-active:rgba(0,0,0,.09) --bg-control:#ffffff --border:rgba(0,0,0,.12) --border-strong:rgba(0,0,0,.22) --text:#1d1d1f --text-muted:#6e6e73 --text-inverse:#ffffff --accent:#0a84ff --accent-hover:#0071e3 --danger:#ff3b30 --danger-hover:#d70015 --success:#34c759 --warning:#ff9f0a --neutral:#8e8e93 --overlay:rgba(0,0,0,.35)`
Dark: `--bg:#1e1e1e --bg-sidebar:#252526 --bg-surface:#2d2d30 --bg-strip:#252526 --bg-hover:#2a2d2e --bg-active:#37373d --bg-control:#3c3c3c --border:#111111 --border-strong:#444444 --text:#dddddd --text-muted:#aaaaaa --text-inverse:#ffffff --accent:#3794ff --accent-hover:#1f7ae0 --danger:#f85149 --danger-hover:#da3633 --success:#3fb950 --warning:#e3a008 --neutral:#888888 --overlay:rgba(0,0,0,.5)`
Brand (same in both): `--brand-claude:#d97757 --brand-codex-bg:#f0f0f0 --brand-codex-fg:#222222 --brand-shell-bg:#1e1e1e --brand-shell-fg:#aaaaaa --brand-shell-border:#555555`
Terminal, light: `--term-bg:#ffffff --term-fg:#1d1d1f --term-cursor:#1d1d1f --term-selection:rgba(10,132,255,.25)` + ANSI `--term-black:#000000 --term-red:#c91b00 --term-green:#00c200 --term-yellow:#c7c400 --term-blue:#0225c7 --term-magenta:#ca30c7 --term-cyan:#00c5c7 --term-white:#c7c7c7 --term-bright-black:#686868 --term-bright-red:#ff6e67 --term-bright-green:#5ffa68 --term-bright-yellow:#fffc67 --term-bright-blue:#6871ff --term-bright-magenta:#ff77ff --term-bright-cyan:#60fdff --term-bright-white:#ffffff`
Terminal, dark: `--term-bg:#1e1e1e --term-fg:#dddddd --term-cursor:#dddddd --term-selection:rgba(55,148,255,.35)` + ANSI `--term-black:#000000 --term-red:#cd3131 --term-green:#0dbc79 --term-yellow:#e5e510 --term-blue:#2472c8 --term-magenta:#bc3fbc --term-cyan:#11a8cd --term-white:#e5e5e5 --term-bright-black:#666666 --term-bright-red:#f14c4c --term-bright-green:#23d18b --term-bright-yellow:#f5f543 --term-bright-blue:#3b8eea --term-bright-magenta:#d670d6 --term-bright-cyan:#29b8db --term-bright-white:#ffffff`

### `src/app.css`
Replace hex with `var(--bg)` / `var(--text)` / `var(--font-ui)`; add `font-size: var(--text-md)`, `-webkit-font-smoothing: antialiased`, `user-select: none` on body (inputs, textareas and `.term` keep `user-select: text`), and a global `:focus-visible { outline: none; box-shadow: var(--focus-ring); }`. `src/main.ts`: `import "./lib/ui/tokens.css";` before `./app.css`.

### Settings field
Rust `Settings`: add `pub theme: String` (default `"system"`), `validate`: `"system" | "light" | "dark"` else `Err("theme must be system, light or dark")`. Extend the existing `defaults`, `round_trip`, `invalid_rejected` tests (default is `system`; `{"theme":"dark"}` partial parses; bad value message exact). Frontend `Settings` interface: `theme: "system" | "light" | "dark"`, default `"system"` in the initial `settings.value`.

### `src/lib/theme.svelte.ts` (new)
```ts
export type ThemeSetting = "system" | "light" | "dark";
export type ResolvedTheme = "light" | "dark";
export function resolveTheme(setting: ThemeSetting, prefersDark: boolean): ResolvedTheme  // pure
export const theme = $state({ resolved: "dark" as ResolvedTheme });
export function initTheme(): () => void   // matchMedia("(prefers-color-scheme: dark)") listener; returns disposer
export function applyTheme(setting: ThemeSetting): void  // sets/removes document.documentElement.dataset.theme ("" for system → remove attribute) and updates theme.resolved
```
`App.svelte`: `onMount` calls `initTheme()` (return its disposer) and a `$effect` calls `applyTheme(settings.value.theme)`.

### `Terminal.svelte`
Add `function xtermTheme()` that reads the `--term-*` vars via `getComputedStyle(document.documentElement).getPropertyValue(...)` and returns an xterm `ITheme` (`background, foreground, cursor, selectionBackground, black…brightWhite`). Pass it at construction and in a new `$effect` that reads `theme.resolved` (so it re-runs on change) and sets `term.options.theme = xtermTheme()`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`
3. `cd src-tauri && cargo test settings`
4. `! grep -nE '#[0-9a-fA-F]{3,8}\b' src/app.css src/lib/Terminal.svelte src/lib/theme.svelte.ts` (App.svelte keeps its hex until ticket 35)

## Done when
- `npm run tauri dev` with macOS in light mode: window background is `#f5f5f7`, text dark, terminal white with dark text. Switch macOS to dark: app and terminal flip without reload.
- `~/.agentfleet/settings.json` with `"theme": "dark"` + relaunch: dark regardless of OS. `"theme": "purple"` + relaunch: file ignored with the stderr message already printed by `load` (defaults used). `invoke set_settings` with bad theme rejects with the exact message.
- `cargo test` all green (49 + new).

## Not included
- Theme selector UI (ticket 34), restyling any component (31, 35, 36), removing hex from `App.svelte` (35).
