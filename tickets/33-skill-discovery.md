---
id: 33
status: done
model: sonnet
depends_on: [32]
max_attempts: 3
files:
  - src-tauri/src/agent.rs
  - src-tauri/src/session.rs
  - src-tauri/src/cli.rs
  - skills/agentfleet/SKILL.md
  - README.md
---
## Goal
Agents launched by agentfleet know the `agentfleet` CLI without any manual setup, and never reach for the user's `orchestra` skill. Two mechanisms: the guide is injected as context at SessionStart (Claude and Codex), and `agentfleet skill install` writes the skill into `~/.claude/skills/agentfleet/` and `~/.codex/skills/agentfleet/` for sessions started outside the app.

## Context
Observed: inside an agentfleet session the agent invoked the user's `~/.claude/skills/orchestra` skill ("Coordinate with other agent sessions…") when asked to message another session, and did not know `agentfleet spawn`. `skills/agentfleet/SKILL.md` is only reachable via `agentfleet guide`; nothing tells the agent to run it. Claude Code and Codex both add a SessionStart hook's `hookSpecificOutput.additionalContext` to the model context.

### Changes
- `cli.rs`: `Cmd::HookContext` (`agentfleet hook-context`) prints one JSON line `{"hookSpecificOutput":{"hookEventName":"SessionStart","additionalContext":"<GUIDE>"}}` and exits 0, with or without a session (same path as `guide`). `Cmd::SkillInstall` (`agentfleet skill install`): writes `GUIDE` to `~/.claude/skills/agentfleet/SKILL.md` always and to `~/.codex/skills/agentfleet/SKILL.md` when `~/.codex` exists; prints each path written; works outside a session. USAGE updated. Tests: parse both; `hook_context_json()` round-trips through serde and contains the guide; `skill_targets(home, codex_exists)` returns the right paths.
- `agent.rs`: `write_hooks_settings(status_file, shim, dest)` — the `SessionStart` entry gets a second hook `{"type":"command","command":"<shell_quoted shim> hook-context"}`. `codex_hook_overrides(script, shim)` — the `SessionStart` override becomes `hooks.SessionStart=[{hooks=[{type="command",command="<script> waiting"},{type="command",command="<shim> hook-context"}]}]`; other five unchanged. `codex_launch` gains the `shim` parameter. Tests updated (SessionStart has 2 hooks, others 1).
- `session.rs` `spawn_for`: pass `rt.shim` (absolute path of the per-launch shim) to both.
- `skills/agentfleet/SKILL.md`: description becomes "Use when the environment has AGENTFLEET_SESSION set (you are inside agentfleet): ask another session for work, spawn or close a sibling session, answer a request, or share notes via the project scratchpad. Not Orchestra: ignore any orchestra skill here." Body: first paragraph states the same and that the guide is also injected at session start.
- `README.md`: one line on `agentfleet skill install` for sessions started outside the app.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `cd src-tauri && cargo run -q -- --cli hook-context | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d["hookSpecificOutput"]["hookEventName"]=="SessionStart" and "agentfleet spawn" in d["hookSpecificOutput"]["additionalContext"]'`

## Done when
- Checks pass; a fresh Claude session in `npm run tauri dev` can answer "how do you message another session here?" with `agentfleet send/spawn` without being told; `agentfleet skill install` creates both files on this machine.

## Not included
- Removing or editing the user's orchestra skill; auto-installing skills at app start; Codex `notify`; UI changes.
