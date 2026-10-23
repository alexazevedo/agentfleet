---
id: 32
status: todo
model: sonnet
depends_on: [31]
max_attempts: 4
files:
  - src/lib/ui/Button.svelte
  - src/lib/ui/Switch.svelte
  - src/lib/ui/SegmentedControl.svelte
  - src/lib/ui/Tabs.svelte
  - src/lib/ui/Popover.svelte
  - src/lib/ui/Dialog.svelte
  - src/lib/ui/Popover.test.ts
  - src/lib/ui/Tabs.test.ts
  - src/lib/ui/Switch.test.ts
  - src/lib/ui/__fixtures__/PopoverHost.svelte
---
## Goal
Six reusable primitives in `src/lib/ui/`, styled only with tokens, keyboard-accessible, covered by component tests. Nothing consumes them yet (33–36 do).

## Context
Decision record: `docs/adr/0001-hand-rolled-ui-primitives.md` (native `<dialog>`, fixed-position popover, no library). Tokens: `src/lib/ui/tokens.css` (ticket 30). Svelte 5 runes, `$props()`, snippets for children. Icons via `lucide-svelte` where needed (`Check`).

### Button.svelte
Props: `variant: "default" | "primary" | "danger" | "ghost" = "default"`, `size: "sm" | "md" = "md"`, `disabled`, `type = "button"`, `onclick`, `children` snippet, rest spread onto `<button>` (`aria-label`, `title`). Styles: `height 28px` (sm 22px), padding `0 var(--space-3)` (sm `0 var(--space-2)`), `border-radius var(--radius-md)`, `font-size var(--text-md)` (sm xs), `font-family inherit`. default: `background var(--bg-control); border 1px solid var(--border-strong); color var(--text)`. primary: `background var(--accent); color var(--accent-text, var(--text-inverse)); border-color transparent`, hover `--accent-hover`. danger: same with `--danger` / `--danger-hover`. ghost: transparent, `color var(--text-muted)`, hover `background var(--bg-hover); color var(--text)`. `disabled`: `opacity .5; cursor default`.

### Switch.svelte
`<button type="button" role="switch" aria-checked={checked}>` with `bind:checked`, `disabled`, `label` snippet rendered as adjacent text in a `<label>` wrapper (click on text toggles). Track 32×18, knob 14, `background var(--neutral)` off / `var(--accent)` on, transition 120ms. Space/Enter toggle (native button behavior). `onchange?: (v: boolean) => void` called after toggle.

### SegmentedControl.svelte
Props: `options: { value: string; label: string }[]`, `bind:value`, `disabled`, `onchange`. `role="radiogroup"`, each option `role="radio" aria-checked` as a `<button>`; ArrowLeft/Right/Home/End move selection and focus; `tabindex` roving (selected = 0). Style: outer `background var(--bg-hover); border-radius var(--radius-md); padding 2px`, selected segment `background var(--bg-surface); box-shadow 0 1px 2px rgba(0,0,0,.2); color var(--text)`, unselected `color var(--text-muted)`.

### Tabs.svelte
Props: `tabs: { id: string; label: string }[]`, `bind:active: string`, `children` snippet receives `active` (render the panel in the consumer: `{@render children(active)}`). Markup: `<div role="tablist">` of `<button role="tab" id="tab-{id}" aria-selected aria-controls="panel-{id}">`, then `<div role="tabpanel" id="panel-{id}" aria-labelledby="tab-{id}">` around the rendered children. Arrow keys move + activate (automatic activation), Home/End. Style: tabs centered in a row, 28px tall, selected tab `color var(--text)` with a 2px `var(--accent)` underline, others `var(--text-muted)`; list has `border-bottom 1px solid var(--border)`.

### Popover.svelte
Props: `anchor: HTMLElement | null`, `open: boolean` (`bind:open`), `placement: "bottom-start" | "bottom-end" = "bottom-start"`, `children` snippet, `onclose?`. Renders nothing when `!open || !anchor`. When open: a `position: fixed; z-index: 50` div with `role="dialog"`, `background var(--bg-surface); border 1px solid var(--border-strong); border-radius var(--radius-lg); box-shadow var(--shadow-popover); padding var(--space-3); min-width 240px`, plus an 8px rotated-square arrow (`::before`) that points at the anchor. Position: `anchor.getBoundingClientRect()`; top = `rect.bottom + 8`; left = `rect.left` (start) or `rect.right - width` (end); if top + height > `innerHeight - 8` flip above (`rect.top - 8 - height`, arrow at bottom); clamp left into `[8, innerWidth - width - 8]`. Measure with `requestAnimationFrame` after mount; recompute on `resize` and `scroll` (capture). Behaviors: on open, focus the first focusable descendant (`[autofocus], button, input, select, [tabindex]`), else the panel (`tabindex="-1"`); `Escape` → close; `mousedown` outside panel and outside anchor → close; on close, `anchor.focus()` and call `onclose`. 120ms opacity/translateY(4px→0) transition via a class added after mount.

### Dialog.svelte
Props: `open: boolean` (`bind:open`), `title: string`, `width = 420`, `children` snippet, `footer?` snippet, `dismissible = true`, `onclose?`. Uses native `<dialog>`: `$effect` calls `showModal()` when `open` and `close()` when not. `cancel` event (Escape): if `dismissible` set `open = false` else `preventDefault`. Click on the backdrop (event target === dialog element) → close when dismissible. `close` event → `open = false; onclose?.()`. Style: `::backdrop { background: var(--overlay) }`, panel `background var(--bg-surface); color var(--text); border 1px solid var(--border-strong); border-radius var(--radius-lg); padding 0; box-shadow var(--shadow-popover)`; header `h2` `font-size var(--text-lg); font-weight 600; padding var(--space-4) var(--space-4) 0; margin 0`; body `padding var(--space-3) var(--space-4)`; footer `display flex; gap var(--space-2); justify-content flex-end; padding 0 var(--space-4) var(--space-4)`. jsdom lacks `showModal`; in `Dialog.svelte` guard `if (typeof el.showModal === "function")`, and tests for Dialog are not required.

### Tests (`@testing-library/svelte` + `user-event`)
- `Popover.test.ts`: mount a wrapper (inline test component via `svelte` `mount` or a `.svelte` fixture under `src/lib/ui/__fixtures__/PopoverHost.svelte` — allowed as an extra file) with a trigger button and a popover containing an input. Assert: click trigger → panel in document and input has focus; `Escape` → panel gone and trigger has focus; reopen, `mousedown` on `document.body` → closed.
- `Tabs.test.ts`: three tabs; click second → `aria-selected` moves, panel shows its content; `ArrowRight` → third.
- `Switch.test.ts`: click → `aria-checked` flips and `onchange(true)` called; Space flips back.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`
4. `! grep -nE '#[0-9a-fA-F]{3,8}\b' src/lib/ui/*.svelte`

## Done when
- All checks green; `npm test` ≥ 4 files passing. No consumer changes.

## Not included
- Tooltip, Menu, Select primitives; animations beyond the single fade; storybook/demo page.
