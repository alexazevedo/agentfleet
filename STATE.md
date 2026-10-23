# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
35–38 done, all checks green (cargo 69, svelte-check 0 errors). Ticket 39 (model in sidebar) dispatched to haiku.

## Next
39 finishes → `graphify update .` → mvp final gate = diff summary review. Live-agent probe (statusLine + codex payload model) still untested against real sessions.

## Decisions
- Dispatch: sequential in place (user choice at forge:run; matches parallel_default)
- No-worktree session = Worktree{path: repo root, branch: current HEAD}, owns_worktree=false; close returns Removed without touching files; old state.json rows default owns_worktree=true
- Claude model channel = per-session settings statusLine command → ~/.agentfleet/run/<id>.model; Codex = `model` key from captured hook payload, blank if absent
- M5 artifacts archived to docs/forge-m5/

## Blockers
(none)
