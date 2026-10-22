# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
26-terminal-prefs — dispatched (sonnet)

## Next
27-restore-prompt

## Decisions
- execution: sequential
- M3 archived under docs/forge-m3/ with ticket 24 (first releases) pending; resume with "cut v0.1.0"
- settings.json gains `terminal {fontFamily,fontSize,lineHeight,cursorBlink}` and `restoreSessions: ask|always|never` (default ask); old files load via serde defaults
- terminal options applied live via $effect on the settings store; refit after change
- Start fresh = forget sessions with no PTY from state.json; worktrees untouched

## Blockers
- none
