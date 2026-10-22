---
id: 25
status: done
model: haiku
depends_on: [20]
max_attempts: 3
files:
  - src-tauri/src/pty.rs
---
## Goal
Agents launched by agentfleet never inherit a parent Claude Code session's markers, so a nested `claude` persists transcripts and behaves as a top-level session.

## Context
Smoke finding: when the app is started from inside a Claude Code session (e.g. `npm run tauri dev` via the `!` prefix), every PTY inherits that session's environment and the nested `claude` prints "Transcript saving is off — inherited CLAUDE_CODE_CHILD_SESSION marker". In `pty::spawn`, after building the `CommandBuilder` and before applying `env`, remove every variable whose name starts with `CLAUDE_CODE_` plus `CLAUDECODE`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_SSE_PORT` (covered by the prefix) — iterate `std::env::vars_os()` and call `cmd.env_remove(name)` for matches. Do not touch `CLAUDE_CONFIG_DIR` (it is a user-facing config path, not a marker). Add a unit test that spawns `/bin/sh -c 'echo "[$CLAUDECODE][$CLAUDE_CODE_CHILD_SESSION][$KEEP_ME]"'` with those three set in the test process (`std::env::set_var`, then restore) and asserts the output is `[][][yes]`.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test pty`

## Done when
- Starting the app from inside a Claude Code session, a new Claude Code tab shows no "Transcript saving is off" notice.

## Not included
- Scrubbing other tools' markers, settings UI for env.
