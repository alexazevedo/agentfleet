# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M5 merged to main; follow-up ticket 33 (skill discovery) done inline by the orchestrator after the first manual probe: agents inside the app reached for the user's `orchestra` skill and did not know `agentfleet spawn`

## Next
re-probe in a fresh `npm run tauri dev` (rebuild needed): new Claude session answers "how do you message another session?" with `agentfleet send/spawn`; spawn claude + codex (`--role reviewer`), `wait`, `close`; restore a Codex tab; M4 probe (theme, settings tabs, popover) still pending

## Decisions
- M5 built on branch worktree-spawn-codex (base 0f121eb) while M4 ran on main; merge resolved here: PRODUCT/STATE/tickets → M5 at root, M4 artifacts archived to docs/forge-m4/; NewTabDialog gone (Codex option added to NewSessionPopover + settings GeneralTab); StatusBar/Sidebar use tabTitle
- both milestones numbered tickets 30+; M4's live in docs/forge-m4/tickets, M5's in tickets/
- spawn = broker `session.spawn` → `session::register` (no PTY) → emit `session-spawned` → frontend attaches with `resume:false` → prompt delivered on first `waiting` (no immediate delivery attempt: no PTY yet)
- roles resolved CLI-side (`~/.agentfleet/roles/<name>.md`); broker never sees roles
- Codex status via the same six hook events as Claude, passed as `-c hooks.<Event>=…` + `--dangerously-bypass-hook-trust`; session id from hook stdin payload → `codex resume <id>`; `scripts/probe-codex-hooks.sh` is the runtime check (one real model call)
- state.json stays version 1; `name` / `agent_args` additive with serde defaults
- skill discovery (33): guide injected at SessionStart via `agentfleet hook-context` (`hookSpecificOutput.additionalContext`, second hook on SessionStart for Claude settings and Codex `-c` override); `agentfleet skill install` writes ~/.claude/skills + ~/.codex/skills; SKILL.md says "not Orchestra"
- M4 decisions: see docs/forge-m4/STATE.md

## Blockers
- none
