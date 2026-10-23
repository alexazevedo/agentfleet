# UI layer

Tokens, theming and primitives live in `src/lib/ui/`. See `docs/adr/0001-hand-rolled-ui-primitives.md` for why they are hand-rolled.

## Tokens (`src/lib/ui/tokens.css`)

All CSS custom properties are defined on `:root`.

- **Surface:** `--bg`, `--bg-sidebar`, `--bg-surface`, `--bg-strip`, `--bg-hover`, `--bg-active`, `--bg-control`, `--border`, `--border-strong`, `--overlay`
- **Text:** `--text`, `--text-muted`, `--text-inverse`
- **Accent / status:** `--accent`, `--accent-hover`, `--danger`, `--danger-hover`, `--success`, `--warning`, `--neutral`
- **Brand:** `--brand-claude`, `--brand-codex-bg`, `--brand-codex-fg`, `--brand-shell-bg`, `--brand-shell-fg`, `--brand-shell-border`
- **Terminal:** `--term-bg`, `--term-fg`, `--term-cursor`, `--term-selection`, and the ANSI set `--term-black` … `--term-bright-white`
- **Type / space / radius / effects:** `--font-ui`, `--font-mono`, `--text-xs|sm|md|lg`, `--space-1` … `--space-5`, `--radius-sm|md|lg`, `--shadow-popover`, `--focus-ring`

## Theming

- `tokens.css` defines the light set on `:root`. A `@media (prefers-color-scheme: dark)` block applies the dark set to `:root:not([data-theme="light"])`, so the OS appearance drives the default.
- `:root[data-theme="dark"]` forces the dark set regardless of the OS.
- `src/lib/theme.svelte.ts` owns the switch. `applyTheme(setting)` sets `data-theme` for `light`/`dark` and removes it for `system`, then updates `theme.resolved`. `initTheme()` re-applies on OS appearance changes while the setting is `system`.
- The setting is `theme: "system" | "light" | "dark"` in the frontend settings store and the Rust `Settings` struct (default `system`). It is edited in Settings > General. `App.svelte` applies it; `Terminal.svelte` re-themes xterm when `theme.resolved` changes.

## Primitives (`src/lib/ui/*.svelte`)

- **Button**: `variant` `"default" | "primary" | "danger" | "ghost"`, `size` `"sm" | "md"`; other native button attributes pass through.
- **Switch**: `checked` (bindable), `disabled`, `label` snippet, `onchange(v: boolean)`.
- **SegmentedControl**: `options: {value, label}[]`, `value` (bindable), `disabled`, `onchange(v: string)`. Arrow keys and Home/End move the selection.
- **Tabs**: `tabs: {id, label}[]`, `active` (bindable), `children` snippet receiving the active id. Arrow keys and Home/End move between tabs.
- **Popover**: `anchor: HTMLElement | null`, `open` (bindable), `placement` `"bottom-start" | "bottom-end"`, `children`, `onclose`. Closes on Escape or outside click and returns focus to the anchor.
- **Dialog**: `open` (bindable), `title`, `width` (default 420), `children`, `footer` snippet, `dismissible` (default true), `onclose`. Wraps native `<dialog>` via `showModal()`.

## Rule

No hex colors outside `src/lib/ui/tokens.css`. Components reference tokens only.

Gate (exit 0 = clean):

```sh
grep -rnE '#[0-9a-fA-F]{3,8}\b' src --include='*.svelte' --include='*.ts' --include='*.css' | grep -v 'src/lib/ui/tokens.css'; test $? -eq 1
```
