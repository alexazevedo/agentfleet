# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
idle — M3.1 approved and committed

## Next
(none) — after approval: commit + push; then "cut v0.1.0" resumes M3 ticket 24 (docs/forge-m3/)

## Decisions
- execution: sequential
- settings.json: `terminal {fontFamily,fontSize,lineHeight,cursorBlink}` + `restoreSessions: ask|always|never`; serde defaults keep old files loading
- terminal options applied live via $effect; SettingsPane form copy = structuredClone($state.snapshot())
- Start fresh = `forget_restorable` drains sessions with no PTY from state.json; worktrees untouched
- updater scheduler waits for `store.restoreDecided` so modals never stack on launch

## Blockers
- none
