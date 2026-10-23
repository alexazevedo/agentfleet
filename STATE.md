# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M4 (native-feeling UI) planned: PRODUCT.md approved (1/3), tickets 30–37 written, awaiting plan approval (2/3)

## Next
tickets 30→31→32→33 sequential in place on branch m4-ui; then 34 ‖ 35 ‖ 36 in git worktrees; merge; then 37; reviewer pass; approval 3/3

## Decisions
- execution: sequential 30–33 in place (branch m4-ui), parallel worktrees for 34/35/36, then 37
- mode: production; routing cloud; hand-rolled primitives on native <dialog> + fixed-position popover (docs/adr/0001)
- theme: tokens.css with prefers-color-scheme default + data-theme override; `theme` field added to Settings (Rust + TS)
- new-session: store.dialogOpen/preselectedProjectId → store.newSession {anchor, projectId}; create failures show in a sidebar banner
- settings: auto-save via updateSettings(patch) promise chain; last tab in localStorage agentfleet.settingsTab
- Messages/Scratchpad: color substitution only (hex gate), no restyle
- M3.2 artifacts archived to docs/forge-m3.2/

## Blockers
- none
