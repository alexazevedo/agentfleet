# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M4: tickets 30–33 done and committed on m4-ui (8d52813); 34, 35, 36 running in parallel worktrees (agentfleet-wt/t34 t35 t36, branches m4-t34/35/36)

## Next
merge m4-t34, m4-t35, m4-t36 into m4-ui as each lands (conflicts → micro-ticket); then ticket 37; then reviewer pass; approval 3/3; merge m4-ui to main

## Decisions
- execution: sequential 30–33 in place (branch m4-ui), parallel worktrees for 34/35/36, then 37
- worktrees need their own `npm ci` (symlinked node_modules breaks vite /@fs resolution)
- mode: production; routing cloud; hand-rolled primitives on native <dialog> + fixed-position popover (docs/adr/0001)
- theme: tokens.css with prefers-color-scheme default + data-theme override; `theme` field added to Settings (Rust + TS)
- new-session: store.newSession {anchor, projectId}; create failures show in a sidebar banner; NewTabDialog deleted
- settings: auto-save via updateSettings(patch) promise chain; last tab in localStorage agentfleet.settingsTab
- Messages/Scratchpad: color substitution only (hex gate), no restyle
- M3.2 artifacts archived to docs/forge-m3.2/

## Blockers
- none
