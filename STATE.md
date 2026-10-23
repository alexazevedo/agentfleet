# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M4 (native-feeling UI) approved 3/3 and merged to main; tickets 30–38 done

## Next
manual probe in `npm run tauri dev` (theme follow/override, settings tabs auto-save, row "+" popover, dialogs); next milestone per docs/roadmap.md "Later / optional"

## Decisions
- execution: 30–33 sequential in place, 34/35/36 parallel worktrees (own `npm ci` each; symlinked node_modules breaks vite), 37–38 in place
- reviewer (forge:reviewer) returned guessed findings without reading files; orchestrator verified all 8 by hand: 7 false, 1 real (theme load fallback → ticket 38)
- mode: production; routing cloud; hand-rolled primitives on native <dialog> + fixed-position popover (docs/adr/0001)
- theme: tokens.css, prefers-color-scheme default + data-theme override; `theme` in Settings (Rust + TS); unknown value normalised to system on load
- new-session: store.newSession {anchor, projectId}; create failures → sidebar banner; NewTabDialog deleted
- settings: updateSettings(patch) promise chain, never throws, store untouched on rejection; tabs in localStorage agentfleet.settingsTab
- Messages/Scratchpad: color substitution only; whole-tree hex gate clean
- M3.2 artifacts archived to docs/forge-m3.2/

## Blockers
- none
