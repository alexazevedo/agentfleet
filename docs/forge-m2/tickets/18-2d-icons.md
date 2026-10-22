---
id: 18
status: done
model: haiku
depends_on: [17]
max_attempts: 3
files:
  - package.json
  - package-lock.json
  - src/lib/Sidebar.svelte
  - src/App.svelte
  - src/lib/Scratchpad.svelte
  - src/lib/Messages.svelte
---
## Goal
Flat 2D line icons (lucide) replace every emoji/text glyph in the chrome: project folder, scratchpad, settings, add, close, sidebar toggle.

## Context
Smoke feedback: the reference app uses flat line icons; ours use emoji. Add `lucide-svelte` (ISC, Svelte 5 compatible) via `npm install lucide-svelte`. Import named icons: `Folder` (project header row), `NotebookPen` (scratchpad row and the Scratchpad pane title), `Settings` (sidebar bottom row), `Plus` (header "+" and per-project "+"), `X` (session close), `PanelLeft` (sidebar toggle in App.svelte, replacing ☰), `Send` (Messages compose button, optional). Size 14 in rows, 16 for the toggle, `stroke-width={1.75}`, colour inherits (`currentColor`), `aria-label` on icon-only buttons. Keep `AgentIcon.svelte` for the agent glyphs. No layout changes; align icons with `display:inline-flex; align-items:center; gap:6px` on the rows that need it.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`

## Done when
- No emoji remain in `src/` (`grep -rnP "[\x{1F300}-\x{1FAFF}\x{2600}-\x{27BF}]" src/` returns nothing); icons render at the same positions as before.

## Not included
- Icon theming, project emoji picker, a settings panel.
