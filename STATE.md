# STATE

## Now
M6 session/model changes and Pi/OpenCode support integrated. All 12 core review findings fixed; regression checks: 86 Rust tests and 14 frontend tests pass, svelte-check clean, production build passes.

## Next
Commit verified fixes and merge to main.

## Decisions
- No-worktree sessions run in the project checkout; close never removes it.
- Claude model comes from statusLine; Codex model comes from hook payloads.
- Pi and OpenCode launch/status/resume support retained from local main.
- Keep unmerged worktrees; bound PTY I/O and shutdown; reserve composers before delivery and block delivery during permission prompts.
- Recover interrupted requests on restart; reject capacity overflow; validate and roll back failed spawns; preserve scratchpad edits across asynchronous operations.
- Prior milestone details remain in docs/forge-m4/ and docs/forge-m5/.

## Blockers
(none)
