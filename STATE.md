# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M5 tickets 30–32 done (1 attempt each; 56 Rust tests, svelte-check 0 errors, build ok; Codex hook probe passed: exit 0, status `waiting`, session_id captured). Uncommitted on worktree-spawn-codex. Awaiting approval 2 of 2 (diff summary).

## Next
manual probe in `npm run tauri dev`: spawn claude + codex from a Claude session, wait, close; restore a Codex session → then commit + merge to main (expect overlap with M4 session)

## Decisions
- branch `worktree-spawn-codex` (worktree under .claude/worktrees/spawn-codex), base main@0f121eb
- M3.2 artifacts archived to docs/forge-m3.2/ (same rename staged on main by the M4 session → identical renames at merge)
- M4 (native UI, production) runs in parallel on main; expect merge overlap in session.rs, cli.rs, agent.rs, NewTabDialog/Sidebar/SettingsPane
- execution: sequential in place (strict chain, shared session.rs)
- order: 30 register/label/args (foundation) → 31 spawn/close CLI (core ask) → 32 Codex (opus: unverified `-c hooks.*` contract, probe script gates it)
- spawn = broker `session.spawn` → `session::register` (no PTY) → emit `session-spawned` → frontend tab attaches with `resume:false` → prompt delivered on first `waiting` (orchestrator removed the immediate `on_request_created` call: no PTY yet → would flag needsAttention)
- roles resolved CLI-side (`~/.agentfleet/roles/<name>.md`), broker never sees roles
- Codex status via the same six hook events as Claude, passed as `-c hooks.<Event>=…` + `--dangerously-bypass-hook-trust`; session id captured from hook stdin payload → `codex resume <id>`
- state.json stays version 1; `name` / `agent_args` added with serde defaults

## Blockers
- none
