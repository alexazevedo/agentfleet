---
id: 37
status: todo
model: haiku
depends_on: [34, 35, 36]
max_attempts: 4
files:
  - docs/roadmap.md
  - docs/ui.md
  - README.md
---
## Goal
Docs reflect the new UI layer, and the whole-tree hex gate from PRODUCT.md passes.

## Context
`docs/roadmap.md` "Later / optional" lists "Themes"; "M3 — release" lists the Settings panel. `docs/adr/0001-hand-rolled-ui-primitives.md` records the primitives decision. README has a features list (check with `grep -n "Settings\|theme" README.md`).

- `docs/ui.md` (new, ≤ 60 lines): token names grouped (surface, text, accent/status, brand, terminal, type/space/radius), how theming works (`prefers-color-scheme` + `data-theme`, `theme.svelte.ts`), the six primitives with their props, the rule "no hex outside `src/lib/ui/tokens.css`", and the grep command.
- `docs/roadmap.md`: add "## M4 — native-feeling UI (done)" with a 4-bullet summary (tokens + light/dark + override, primitives, new-session popover, tabbed auto-saving Settings); move "Themes" out of Later into "custom color themes" wording; note Settings panel now tabbed.
- README: one line under features: light/dark theme following the OS with override.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`
4. `cd src-tauri && cargo test`
5. `! grep -rnE '#[0-9a-fA-F]{3,8}\b' src --include=*.svelte --include=*.ts --include=*.css | grep -v 'src/lib/ui/tokens.css'`

## Done when
- All five checks green on the merged tree; docs mention nothing that the code does not do.

## Not included
- Screenshots, changelog, release bump (release flow is docs/release.md).
