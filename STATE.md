# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M5 (spawn/close CLI + Codex agent) done, approved 2/2, merged with M4 (native UI) into main; tickets 30–32 done

## Next
manual probe in `npm run tauri dev`: spawn claude + codex from a Claude session (`--role reviewer`), `wait`, `close`; quit + restore a Codex tab (resume); M4 probe (theme, settings tabs, popover) still pending too

## Decisions
- M5 built on branch worktree-spawn-codex (base 0f121eb) while M4 ran on main; merge resolved here: PRODUCT/STATE/tickets → M5 at root, M4 artifacts archived to docs/forge-m4/; NewTabDialog gone (Codex option added to NewSessionPopover + settings GeneralTab); StatusBar/Sidebar use tabTitle
- both milestones numbered tickets 30+; M4's live in docs/forge-m4/tickets, M5's in tickets/
- spawn = broker `session.spawn` → `session::register` (no PTY) → emit `session-spawned` → frontend attaches with `resume:false` → prompt delivered on first `waiting` (no immediate delivery attempt: no PTY yet)
- roles resolved CLI-side (`~/.agentfleet/roles/<name>.md`); broker never sees roles
- Codex status via the same six hook events as Claude, passed as `-c hooks.<Event>=…` + `--dangerously-bypass-hook-trust`; session id from hook stdin payload → `codex resume <id>`; `scripts/probe-codex-hooks.sh` is the runtime check (one real model call)
- state.json stays version 1; `name` / `agent_args` additive with serde defaults
- M4 decisions: see docs/forge-m4/STATE.md

## Blockers
- none
