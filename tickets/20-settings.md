---
id: 20
status: done
model: sonnet
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/settings.rs
  - src-tauri/src/lib.rs
  - src-tauri/src/session.rs
  - src-tauri/src/agent.rs
  - src-tauri/src/deliver.rs
  - src/lib/settingsStore.svelte.ts
  - src/lib/SettingsPane.svelte
  - src/lib/Sidebar.svelte
  - src/lib/NewTabDialog.svelte
  - src/lib/sessions.svelte.ts
  - src/App.svelte
  - src-tauri/capabilities/default.json
  - package.json
---
## Goal
A Settings pane (sidebar Settings row → main pane) edits worktrees root, default agent, Claude auto-permission, auto-delivery, confirm-quit and update-checks, persisted in `~/.agentfleet/settings.json`, and the M1/M2 code paths honour those values instead of hardcoded ones.

## Context
`settings.rs`: `#[derive(Serialize, Deserialize, Clone)] #[serde(rename_all = "camelCase", default)] pub struct Settings { worktrees_root: PathBuf /* default ~/.agentfleet/worktrees */, default_agent: String /* "claude" */, auto_permission: bool /* true */, auto_delivery: bool /* true */, confirm_quit: bool /* true */, update_checks: bool /* true */ }`, `impl Default`, `load(path)` (missing/unreadable → defaults, logged), `save(&Settings, path)` (tmp+rename like state.rs), `pub type SettingsState = Mutex<Settings>` managed from `lib.rs` setup. Commands: `get_settings() -> Settings`, `set_settings(settings: Settings) -> Result<Settings, String>` (validate: `worktrees_root` absolute, `default_agent` ∈ claude|shell; save; return), `app_info() -> { version: env!("CARGO_PKG_VERSION"), commit: option_env!("AGENTFLEET_COMMIT").unwrap_or("unknown") }` (use `option_env!` so the build works before ticket 19 lands). Tests: defaults, round trip, invalid agent rejected.

Consumers: `session.rs` `create_session` uses `settings.worktrees_root` (lock order: settings may be read first and released before sessions). `agent.rs` `claude_launch(session_id, settings_path, resume, auto_permission: bool)` adds `--permission-mode auto` only when true (update its tests). `deliver.rs` `on_status` and `on_request_created` return early without delivering when `auto_delivery` is false (Execute still forces). `confirm_quit` and `update_checks` are only stored here (tickets 22, 21 read them).

Frontend: `settingsStore.svelte.ts` (`settings`, `loadSettings`, `saveSettings`, `appInfo`); `SettingsPane.svelte`: a form with a folder picker (`@tauri-apps/plugin-dialog` `open({directory:true})`) for the worktrees root, a select for default agent, four checkboxes, Save with inline validation error, and an About block (version, commit). `store.view` union gains `'settings'`; the sidebar Settings row sets `store.activeKey = 'settings'` and `store.view = 'settings'`; `App.svelte` renders the pane in `.pane` with a "Settings" strip label. `NewTabDialog` preselects `settings.defaultAgent`.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- Change the worktrees root in Settings, create a session → its worktree lands under the new root; set default agent to Shell → the dialog preselects Shell; turn auto-delivery off → `agentfleet send` stays queued until Execute; `settings.json` reflects every change after relaunch.

## Not included
- Update checks behaviour and "Check now" (ticket 21), quit confirmation (22), themes/fonts/shortcuts.
