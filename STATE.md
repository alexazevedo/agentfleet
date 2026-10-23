# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M6 complete: tickets 35–39 all done, every attempt 1. Final checks: cargo 69 passed, svelte-check 0 errors, vitest 10 passed. Graph updated. PR #1 merged to main (CI green).

## Next
Live probe pending in `npm run tauri dev`: popover name+worktree→session in repo root; model appears in sidebar for Claude (statusLine) and Codex (payload `model` key, may stay blank).

## Decisions
- Dispatch: sequential in place (user choice at forge:run; matches parallel_default)
- No-worktree session = Worktree{path: repo root, branch: current HEAD}, owns_worktree=false; close returns Removed without touching files; old state.json rows default owns_worktree=true
- Claude model channel = per-session settings statusLine command → ~/.agentfleet/run/<id>.model; Codex = `model` key from captured hook payload, blank if absent
- M5 artifacts archived to docs/forge-m5/

## Blockers
(none)
