---
id: 39
status: done
model: haiku
depends_on: [37, 38]
max_attempts: 3
files:
  - src/lib/sessions.svelte.ts
  - src/lib/Sidebar.svelte
---
## Goal
Each session row in the sidebar project tree shows the agent's current model.

## Context
Ticket 38 added `model` to `SessionInfo` (camelCase over Tauri) and emits `session-model` `{ id, model }`. In `sessions.svelte.ts`: add `model: string` to the `SessionInfo` interface; in `initStatusListener()` (or a sibling `listen` call beside it) subscribe to `session-model` and set `t.info.model` on the matching tab (ignore unknown ids — no `early` buffering needed; a later event will repeat). In `Sidebar.svelte`: inside the session row, after the `id8` span, render `{#if tab.info?.model}<span class="model">{tab.info.model}</span>{/if}` styled like `.id8` (`var(--text-muted)`, `var(--text-xs)`, ellipsis via `overflow: hidden; text-overflow: ellipsis; max-width: 9em; white-space: nowrap`). Blank model renders nothing.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run test`

## Done when
- svelte-check 0 errors; a `session-model` event updates the visible row; sessions without a model look exactly as today.

## Not included
- Model in StatusBar or Messages view; tooltips; any backend change.
