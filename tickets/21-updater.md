---
id: 21
status: todo
model: sonnet
depends_on: [19, 20]
max_attempts: 3
files:
  - src-tauri/Cargo.toml
  - src-tauri/tauri.conf.json
  - src-tauri/capabilities/default.json
  - src-tauri/src/lib.rs
  - package.json
  - src/lib/updater.svelte.ts
  - src/lib/UpdateDialog.svelte
  - src/lib/SettingsPane.svelte
  - src/App.svelte
---
## Goal
The app checks GitHub releases for a newer version (10 s after start and every 24 h when enabled, or on "Check now" in Settings), shows an Update dialog with Install / Later, and installs then relaunches.

## Context
Add `tauri-plugin-updater = "2"` and `tauri-plugin-process = "2"` (Rust) and `@tauri-apps/plugin-updater`, `@tauri-apps/plugin-process` (npm); register both plugins in `lib.rs`. `tauri.conf.json` `plugins.updater`: `pubkey` = contents of `src-tauri/updater.pubkey` (the orchestrator places this file before dispatch; read it, paste the single-line key, do not generate keys), `endpoints: ["https://github.com/alexazevedo/agentfleet/releases/latest/download/latest.json"]`. Capabilities: add `updater:default`, `process:default`, `dialog:allow-ask`/`dialog:allow-message` as needed.

`updater.svelte.ts`: `checkForUpdate(manual: boolean)` using `check()` from the plugin; on an update set `updateState = { available, version, body, update }`; on none and `manual` show a "You're up to date" message; on error and `manual` show it inline in Settings, otherwise log. `installUpdate()` → `update.downloadAndInstall()` then `relaunch()`. Scheduler in `App.svelte` `onMount`: if `settings.updateChecks`, `setTimeout(10 s)` then `setInterval(24 h)`; re-evaluate when the setting changes. `UpdateDialog.svelte`: modal with version, release notes (plain text), Install / Later; shows a progress line while downloading. SettingsPane: "Check now" button next to the update-checks toggle, with the result text. Checks cannot exercise a real release; the done-when is verified by ticket 24.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`
4. `grep -q '"pubkey": "dW50cnVzdGVkIGNvbW1lbnQ' src-tauri/tauri.conf.json`

## Done when
- With no release published, "Check now" reports an error/no update without crashing; the scheduler does not run when update checks are off.
- (Proven in ticket 24) a newer release triggers the dialog and installs.

## Not included
- Beta channel, skip-version, delta updates, Windows.
