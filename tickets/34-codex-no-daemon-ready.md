---
id: 34
status: done
model: opus
depends_on: [33]
max_attempts: 3
files:
  - src-tauri/src/agent.rs
  - src-tauri/src/session.rs
  - src-tauri/src/cli.rs
  - scripts/probe-codex-hooks.sh
---
## Goal
A spawned Codex session actually receives its prompt: Codex launches in embedded mode so the `-c` hook overrides apply, reports `waiting` as soon as its TUI is idle (before any turn), and gets the agentfleet guide as developer instructions.

## Context (verified 2026-10-23 against codex-cli 0.163.0 by driving the TUI in a pty)
- Codex TUI attaches to a shared background daemon by default and then **ignores every `-c` override** (startup warning: "command-line configuration overrides (-c, --enable, --disable, or --search) requires embedded mode"). `codex exec` is embedded, which is why `scripts/probe-codex-hooks.sh` passed while the app's sessions never wrote a status file. `--no-daemon` fixes it.
- With `--no-daemon` + `--dangerously-bypass-hook-trust`, the `-c hooks.*` hooks run: `SessionStart` fires at the **first turn** (not at launch), `UserPromptSubmit` → working, `Stop` → waiting; payload has `session_id`. So nothing writes `waiting` before the first prompt — and auto-delivery waits for `waiting`: deadlock for spawned sessions.
- At startup the TUI emits `ESC ] 7501 ; state=idle:app=codex ESC \`, then `state=working` ×n while loading, then `state=idle` when ready (≈2–3 s). It is not emitted per turn, so it is only a readiness signal.
- `-c developer_instructions="…"` (TOML basic string) is honoured by the model. Bracketed paste followed immediately by `\r` submits a turn.

### Changes
- `agent.rs`: `codex_launch` adds `--no-daemon`; the Codex SessionStart override goes back to the single status hook (drop the `hook-context` hook and the `shim` parameter from `codex_hook_overrides`/`codex_launch`); add `-c developer_instructions=<toml_basic_string(GUIDE)>` where `pub fn toml_basic_string(s: &str) -> String` escapes `\`, `"`, newline, tab, CR and control chars; `GUIDE` becomes `pub` in `cli.rs` and is reused. Add `pub fn codex_ready_scan(seen_working: &mut bool, chunk: &[u8]) -> bool`: scans a PTY chunk for `7501;state=working` (sets the flag) and `7501;state=idle`; returns true the first time an idle follows a working. Tests: escaping, launch string (`--no-daemon`, 6 `-c hooks`, 1 `-c developer_instructions`), the observed idle/working/working/idle sequence.
- `session.rs` `spawn_for`, Codex only: wrap the PTY `on_data` callback so each chunk goes through `codex_ready_scan`; on the first ready signal, and only while the session status is `Unknown`, call `status::emit(&app, id, Status::Waiting)` (this triggers auto-delivery of the queued prompt). Keep a 6 s fallback (`tauri::async_runtime::spawn` + sleep): if status is still `Unknown`, emit `Waiting`. The Claude branch is unchanged (`hook-context` stays there).
- `scripts/probe-codex-hooks.sh`: run `codex exec … --no-daemon` too, so the probe exercises the same mode as the app.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run build`

## Done when
- Checks pass; in `npm run tauri dev`, `agentfleet spawn --agent codex --role reviewer "…"` from a Claude session makes the Codex tab's dot turn `waiting` within a few seconds, the prompt appears and runs in the Codex TUI, the dot goes working → waiting, and `agentfleet wait <run>` returns the reply once Codex runs `agentfleet reply`.

## Not included
- Codex `notify` (hooks suffice once embedded); per-turn OSC 7501 parsing; trusting hooks persistently via `/hooks`; changes to Claude launch.
