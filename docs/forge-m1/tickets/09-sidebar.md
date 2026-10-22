---
id: 09
status: done
model: sonnet
depends_on: [08]
max_attempts: 3
files:
  - src/App.svelte
  - src/lib/Sidebar.svelte
  - src/lib/StatusBar.svelte
  - src/lib/NewTabDialog.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/TabBar.svelte
  - src/app.css
---
## Goal
Replace the tab bar with a collapsible left sidebar that groups sessions under projects (with status dots), an empty-state main pane, and a bottom status bar.

## Context
Pure frontend; backend commands from ticket 08 are final. Layout (dark theme, system font): `grid-template-columns: var(--sidebar-w) 1fr`, rows `1fr 24px`. Sidebar 260 px, collapsible to 0 via a toggle button at its top (remember in `localStorage`, wrapped in try/catch); window default 1200×800.

Sidebar content, top to bottom: header row "Projects" + "+" (folder picker → `addProject`); for each project a row with a folder glyph, name, and a "+" that opens `NewTabDialog` with that project preselected (dialog then shows only the agent radio); under it one row per session: agent label ("Claude Code" / "Shell"), the `id8`, close "×" on hover, and a status dot at the right (grey unknown, green working, amber waiting, red ended; "kept" text while closing). Active session row highlighted; click selects. Bottom of sidebar: a disabled "Settings" row (placeholder for M3). Main pane: terminals as today (hidden when inactive) or an empty state "Add a project, then press + to start a session" when there are no sessions. Status bar: `<project> · <agent> <id8> · <status>` on the left, "N running sessions" (sessions with a PTY attached, status ≠ ended) on the right. Move `:global` styles into `src/app.css` imported by `main.ts`. Delete `TabBar.svelte`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`
3. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`

## Done when
- App opens with sidebar listing persisted projects/sessions, clicking a session shows its terminal, "+" on a project creates a session under it, status dots update live, sidebar collapses and the main pane refits (terminal `fit()` runs on toggle).

## Not included
- Resizable sidebar drag, project rename/emoji/remove UI, Settings contents, Changes/Messages panes, light theme.
