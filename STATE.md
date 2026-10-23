# STATE

## Now
M6 session/model changes and Pi/OpenCode support integrated. Core correctness fixes in progress from the 12-finding review.

## Next
Regression checks, commit fixes, merge to main.

## Decisions
- No-worktree sessions run in the project checkout; close never removes it.
- Claude model comes from statusLine; Codex model comes from hook payloads.
- Pi and OpenCode launch/status/resume support retained from local main.
- Prior milestone details remain in docs/forge-m4/ and docs/forge-m5/.

## Blockers
(none)
