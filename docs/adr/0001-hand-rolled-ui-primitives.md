# ADR 0001 — Hand-rolled UI primitives on native `<dialog>` and a positioned `<div>`

Status: accepted (M4 plan, 2026-10-22)

## Context
M4 needs six primitives (Button, Switch, SegmentedControl, Tabs, Popover, Dialog). Options: a headless Svelte 5 library (bits-ui, melt) or hand-rolled components in `src/lib/ui/`.

## Decision
Hand-rolled, zero new runtime dependencies.
- Dialog wraps the native `<dialog>` element (`showModal()`): top layer, focus trap, Escape, `aria-modal` for free. Supported by WKWebView (Safari ≥ 15.4) and WebKitGTK ≥ 2.36, both below the app's floor.
- Popover is a `position: fixed` div positioned from the trigger's `getBoundingClientRect()` (below-start, flips above when it would overflow the viewport). Outside-click and Escape close it; focus returns to the trigger. The native Popover API and CSS anchor positioning are not used: WebKitGTK support is version-dependent.
- Switch is a `<button role="switch" aria-checked>`; SegmentedControl is `role="radiogroup"` with arrow-key movement; Tabs is `role="tablist"` / `role="tab"` / `role="tabpanel"` with arrow keys.

## Consequences
- ~300 lines of our own code to maintain, covered by Vitest component tests (Popover, Tabs, Switch).
- No bundle growth, no library upgrade churn, styling fully on our tokens.
- If a later milestone needs menus, comboboxes or tooltips, revisit bits-ui rather than growing this set.
