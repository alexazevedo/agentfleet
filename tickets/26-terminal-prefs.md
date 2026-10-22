---
id: 26
status: done
model: sonnet
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/settings.rs
  - src/lib/settingsStore.svelte.ts
  - src/lib/SettingsPane.svelte
  - src/lib/Terminal.svelte
---
## Goal
Font family, font size, line height and cursor blink are editable in Settings and apply immediately to every open terminal and to new ones.

## Context
`settings.rs`: nested `#[derive(Serialize, Deserialize, Clone, PartialEq)] #[serde(rename_all = "camelCase", default)] pub struct TerminalSettings { font_family: String /* "" = system monospace */, font_size: u8 /* 13 */, line_height: f32 /* 1.0 */, cursor_blink: bool /* true */ }` as `Settings.terminal` (with `#[serde(default)]` so old files load). Validation in `set_settings`: `font_size` 9..=24, `line_height` 1.0..=1.6, else `Err("font size must be 9–24")` / `"line height must be 1.0–1.6"`. Tests: defaults, out-of-range rejected, old JSON without `terminal` loads with defaults.

Frontend: `Settings.terminal: { fontFamily, fontSize, lineHeight, cursorBlink }` in `settingsStore.svelte.ts` (defaults mirror Rust). `SettingsPane.svelte`: a "Terminal" `<h3>` section with a text input (placeholder "system monospace"), two number inputs (`min/max/step`: 9–24 step 1; 1.0–1.6 step 0.05) and a checkbox; same Save button. `Terminal.svelte`: build the xterm options from `settings.value.terminal` (`fontFamily: family ? \`${family}, ui-monospace, Menlo, monospace\` : "ui-monospace, Menlo, monospace"`, `fontSize`, `lineHeight`, `cursorBlink`), and add an `$effect` that, when any of the four values change after mount, sets `term.options.fontFamily/fontSize/lineHeight/cursorBlink` and calls the existing `refit()` (only if the terminal is visible; hidden ones refit on activation already). Ensure `loadSettings()` has resolved before the first terminal mounts, or default to the store's defaults — both acceptable; terminals created before load pick the new values up through the effect.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test settings`
3. `npm run check && npm run build`

## Done when
- With two terminals open, change the font size to 16 and Save → both re-render at 16 px and the PTYs get resized (prompt reflows); the family "JetBrains Mono" (if installed) shows immediately; `settings.json` contains `terminal`.

## Not included
- Cursor style, scrollback, padding, smoothing, theme colours, font picker, Startup section (ticket 27).
