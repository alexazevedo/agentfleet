---
id: 17
status: done
model: sonnet
depends_on: [16]
max_attempts: 3
files:
  - src/lib/Sidebar.svelte
  - src/lib/AgentIcon.svelte
  - src/lib/StatusBar.svelte
  - src/lib/Messages.svelte
---
## Goal
The Scratchpad row sits directly under its project header, above the sessions; session rows and the status bar show an agent icon (Claude, Codex, Shell) instead of plain text labels.

## Context
Smoke-test feedback: the scratchpad belongs to the project, so it is the first child row under the project header (as the reference app places its notes row), followed by the session rows. Keep the row's existing click behaviour and active highlight.

`AgentIcon.svelte` props `{ agent: 'claude' | 'codex' | 'shell'; size?: number }` rendering an inline SVG, original simple glyphs only (no copied brand assets): claude = an orange eight-ray asterisk/sunburst (`#d97757`), codex = a white rounded square with a dark `>_` style mark (prepared for a future agent; not selectable yet), shell = a grey terminal prompt `>` in a rounded rect. `aria-label` = agent name. Sidebar session row: `<AgentIcon agent={tab.info.agent} size={14} />` + label "Claude Code"/"Shell" + id8, as now. StatusBar left segment and Messages direction labels (`→`/`←`) reuse the icon at size 12 where an agent is known. No layout changes beyond that.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run build`

## Done when
- Each project shows: header → Scratchpad row → session rows. Session rows and the status bar show the icon; `agentfleet` app still builds with zero svelte-check errors.

## Not included
- Codex as a launchable agent, emoji/project icons, icon theming.
