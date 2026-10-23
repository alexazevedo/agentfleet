---
id: 38
status: done
model: opus
depends_on: [35, 36]
max_attempts: 3
files:
  - src-tauri/src/agent.rs
  - src-tauri/src/cli.rs
  - src-tauri/src/status.rs
  - src-tauri/src/session.rs
  - src-tauri/src/state.rs
---
## Goal
The backend learns each agent session's current model live and emits it to the UI as a `session-model` event, persisted across restarts.

## Context
Channel per agent:
- **Claude**: the settings.json we write per session (`agent::write_hooks_settings`) gains a `statusLine` entry: `{"type":"command","command":"<shim> statusline <model-file>"}` where `<model-file>` is `~/.agentfleet/run/<id>.model` (pass both paths into `write_hooks_settings`; shim path is already a param). Claude invokes the command with JSON on stdin containing `model.display_name` and `model.id`, on every update including mid-session `/model` switches. New CLI subcommand `statusline <file>` in cli.rs, handled BEFORE the `env()` gate (like `guide`): read stdin JSON, extract `model.display_name` (fall back to `model.id`, else empty), write it to `<file>`, print it to stdout (it becomes the visible status line) — exit 0 even on parse failure.
- **Codex**: every hook already dumps its payload to `~/.agentfleet/run/<id>.codex-hook.json`. Add `agent::codex_model(payload_file) -> Option<String>` mirroring `codex_session_id`, reading a top-level `"model"` string key; None if absent (model then stays blank — acceptable per PRODUCT.md).

Plumbing:
- `status::spawn_poller` gains a `model_file: PathBuf` param (Claude: `<id>.model`; Codex: the payload json — pass which parse to use via a small enum or closure). Each tick, read + extract model; on change emit `session-model` `{id, model}` via a new `status::emit_model(app, id, model)` that also sets `session.model` and calls `session::persist` (lock sessions then projects, same order as everywhere).
- `Session` and `PersistedSession` gain `model: String` (`#[serde(default)]`); `SessionInfo` gains `model` (camelCase) so restored tabs show the last-known model; wire through `register`, `info_for`, `persist`, `list_restorable`, and the test fixtures in session.rs/state.rs/broker.rs that construct `Session`/`PersistedSession`.
- `close_one`: add `"model"` to the run-file extensions cleanup list.

Tests: statusline subcommand parse (model file written + printed, bad JSON tolerated), `codex_model` extraction, settings JSON contains the statusLine entry, state.json round-trip with model + old-row default.

## Checks (run in order — cheapest first)
1. `cargo test --manifest-path src-tauri/Cargo.toml`
2. `npm run check`

## Done when
- `write_hooks_settings` output contains a statusLine command pointing at the shim; `agentfleet statusline <f>` writes/echoes the display name; poller emits `session-model` when the file content changes; model survives state.json round-trip.

## Not included
- Sidebar rendering (ticket 39). No model selection/switching from agentfleet. No Codex OSC parsing for model.
