---
id: 35
status: done
model: sonnet
depends_on: [33]
max_attempts: 4
files:
  - src/lib/Sidebar.svelte
  - src/lib/StatusBar.svelte
  - src/App.svelte
  - src/lib/AgentIcon.svelte
  - src/lib/Messages.svelte
  - src/lib/Scratchpad.svelte
---
## Goal
Sidebar, status bar and main-pane chrome use tokens and primitives and read as one polished desktop app in both themes; Messages and Scratchpad lose their hardcoded hex via a color-only substitution. Behavior unchanged.

## Context
Tokens: `src/lib/ui/tokens.css` (ticket 30). Primitives: `Button` (`ghost`, `sm`) in `src/lib/ui/`. Sidebar structure and handlers from ticket 33 stay; only markup classes/styles change. Keep every `aria-label`, `role`, `tabindex`, `title` and the hover-reveal behavior of `.close` / `.remove`.

### Sidebar.svelte
- `background var(--bg-sidebar); border-right 1px solid var(--border); font-family var(--font-ui); font-size var(--text-md)`.
- Header: "Projects" as `var(--text-xs)` uppercase, letter-spacing .04em, `var(--text-muted)`, 600; the two header buttons become `<Button variant="ghost" size="sm">` with icons.
- Project row: 28px tall, `border-radius var(--radius-sm)` with 4px horizontal margin, hover `var(--bg-hover)`; name 500 weight; action buttons `Button ghost sm`, remove button icon colored `var(--danger)` on hover.
- Session rows (incl. Scratchpad row): 26px tall, same radius/margin, indent 24px, hover `var(--bg-hover)`, active `var(--bg-active)` + `color var(--text)`; `.id8` `var(--font-mono) var(--text-xs) var(--text-muted)`; badge `background var(--accent); color var(--text-inverse)`; `.kept` `var(--warning)`; dots: unknown `var(--neutral)`, working `var(--success)`, waiting `var(--warning)`, ended `var(--danger)`; add a 1px lighter ring on the dot (`box-shadow 0 0 0 1px var(--bg-sidebar)`).
- Settings row: `border-top 1px solid var(--border)`, same hover/active treatment, gear icon.
- Banner (33) unchanged.

### StatusBar.svelte
`background var(--bg-strip); color var(--text-muted); border-top 1px solid var(--border); font-size var(--text-xs)`; left text `var(--text)` for project name only (wrap it in a `<strong>` with 500 weight). The accent-blue bar is gone. Height stays 24px (grid in App).

### App.svelte
Strip: `background var(--bg-strip); border-bottom 1px solid var(--border)`; view buttons become 26px pills (`border-radius var(--radius-sm)`), `color var(--text-muted)`, `.on` → `color var(--text); background var(--bg-active)` (no underline); labels same. Toggle button `color var(--text-muted)`, hover `var(--text)`. `.empty` `color var(--text-muted)`; add a 24px lucide `Plus` icon above the text inside a dashed `var(--border-strong)` rounded box. Keep the `--sidebar-w` grid.

### AgentIcon.svelte
Replace the five hex literals with `var(--brand-claude)`, `var(--brand-codex-bg)`, `var(--brand-codex-fg)`, `var(--brand-shell-bg)`, `var(--brand-shell-fg)`, `var(--brand-shell-border)` (SVG `fill`/`stroke` accept `var()`).

### Messages.svelte, Scratchpad.svelte (substitution only)
Map each hex to the closest token: `#1e1e1e`→`var(--bg)`, `#252526`→`var(--bg-surface)`, `#2d2d2d`/`#111`→`var(--border)`, `#444`→`var(--border-strong)`, `#333`→`var(--bg-control)`, `#aaa`→`var(--text-muted)`, `#888`→`var(--text-muted)`, `#fff`→`var(--text-inverse)`, `#666`→`var(--neutral)`, `#0e639c`→`var(--accent)`, `#2ea043`→`var(--success)`, `#b7791f`→`var(--warning)`, `#da3633`/`#f85149`→`var(--danger)`. No layout or markup changes in these two files.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`
4. `! grep -rnE '#[0-9a-fA-F]{3,8}\b' src/lib src/App.svelte --include=*.svelte --include=*.ts`
5. `git diff --stat -- src/lib/Messages.svelte src/lib/Scratchpad.svelte` shows only `<style>` lines changed (manual look by the runner; the diff must contain no `<` or `{` markup lines)

## Done when
- `npm run tauri dev` in light and dark: sidebar/strip/status bar share surfaces and borders; hover and active rows visible in both; status dots readable; Messages and Scratchpad still work and no longer look pitch black in light mode.
- All existing interactions (new session, remove, close, scratchpad, settings, messages tab) unchanged.

## Not included
- Messages/Scratchpad layout changes, icon set swap, sidebar resizing, drag reorder.
