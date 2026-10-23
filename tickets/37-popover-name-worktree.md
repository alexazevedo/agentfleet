---
id: 37
status: done
model: sonnet
depends_on: [35]
max_attempts: 3
files:
  - src/lib/NewSessionPopover.svelte
  - src/lib/sessions.svelte.ts
  - src/lib/Terminal.svelte
---
## Goal
The New-session popover takes an optional session name and a "Create worktree" checkbox (default checked), and both reach the backend.

## Context
Flow: NewSessionPopover `create()` → `addPending(projectId, agent)` pushes a pending `Tab` → Terminal.svelte's `onMount` calls `invoke("create_session", { projectId, agent, cols, rows, onData })`. Ticket 35 added `name: Option<String>` (already existed) and `worktree: Option<bool>` params to `create_session`.

- `sessions.svelte.ts`: `Tab` gains `name?: string; worktree?: boolean;`. `addPending(projectId, agent, name = "", worktree = true)` stores them.
- `Terminal.svelte`: pass `name: tab.name ?? ""` and `worktree: tab.worktree ?? true` in the `create_session` invoke.
- `NewSessionPopover.svelte`: add `let name = $state("")` and `let worktree = $state(true)`; reset both in the existing open-effect; render a text `<input placeholder="Session name (optional)" aria-label="Session name" bind:value={name}>` above the SegmentedControl and a `<label><input type="checkbox" bind:checked={worktree}> Create worktree</label>` below it; `create()` passes `name.trim()` and `worktree` to `addPending`. Keep the existing Enter-key handling (Enter in the text input should trigger `create()` — the current `onkeydown` already does this since INPUT is neither BUTTON nor SELECT). Style with existing tokens (`var(--space-2)`, `var(--text-sm)`), match the current panel look.

Empty name = today's default title (`tabTitle` falls back to agent label — no change needed).

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm run test`
3. `cargo test --manifest-path src-tauri/Cargo.toml`

## Done when
- svelte-check passes with 0 errors; popover shows name field + checked worktree checkbox; values flow into `create_session`.

## Not included
- Remembering last choice, settings default, renaming existing sessions, model display (tickets 38/39).
