---
mode: production                # prototype | mvp | production
routing_profile: cloud          # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential    # sequential | parallel
owner: human                    # human | agent (set by forge:auto)
---

# agentfleet (M4 — native-feeling UI)

## What / why
The app works but looks like a scaffold: every component carries its own hardcoded hex colors, `<select>`/radio/checkbox are browser defaults, "New tab" is a centered modal with a backdrop, Settings is one long scroll with a Save button, and the whole thing is dark-only regardless of the OS. This run gives agentfleet the feel of a polished desktop app (VS Code / Linear class, platform-neutral chrome, no macOS vibrancy tricks): one token layer, a small set of shared primitives, light/dark that follows the OS with an in-app override, an anchored new-session popover, and a tabbed, auto-saving Settings pane. Shape rules in docs/roadmap.md apply: layout stays sidebar + main pane + status bar; no new sidebar.

## Primary user story
As a solo developer living in this app all day, I want it to look and behave like the native tools around it (system appearance, instant-apply preferences, lightweight popovers) so that nothing about its chrome distracts me from the agents.

## Scope
- **Tokens:** CSS custom properties on `:root` for color (bg/surface/border/text/accent/status), spacing, radius, type scale. Light and dark sets. `prefers-color-scheme` drives the default; `data-theme` attribute overrides. No hardcoded hex remains in `src/lib`.
- **Theme setting:** `theme: "system" | "light" | "dark"` added to `Settings` (frontend store + Rust settings struct, default `system`), shown in Settings > General. xterm theme switches with it.
- **Primitives (src/lib/ui/):** Button (default/primary/danger), Switch, SegmentedControl, Tabs, Popover (anchored to a trigger, arrow, Esc/outside-click closes, focus returns to trigger), Dialog (modal, Esc/Enter, focus trap). Hand-rolled vs library is a forge:plan decision.
- **New session popover:** replaces `NewTabDialog`. From a project row "+": project implicit, agent as SegmentedControl (Claude Code / Shell), Enter creates, Esc closes. From the global "+": adds a project picker + "Add project…" row. Same `addPending` / `addProject` calls; error inline inside the popover.
- **Settings pane:** Tabs General (worktrees root, default agent, theme, auto-permission, auto-delivery, confirm quit, restore sessions) · Terminal (font family, size, line height, cursor blink) · Updates (check toggle, Check now, last result) · About (version, commit). Last tab remembered in the frontend. Auto-save: toggles/selects write on change, text/number on blur or Enter, debounced; backend validation error shows inline beside the field and the control reverts. Terminal font changes apply live to open terminals. No Save button.
- **Restyle on tokens + primitives:** Sidebar (project/session rows, status dots, hover actions, Settings row), StatusBar, RemoveProjectDialog, RestoreDialog, UpdateDialog. Behavior unchanged.
- **Tests:** Rust unit test for the theme field default + round-trip; Vitest/svelte component tests for Popover open/close/focus, Tabs persistence, and auto-save revert-on-error; `svelte-check` 0 errors; production reviewer pass.

## Out of scope
- macOS-specific chrome: hidden title bar, inset traffic lights, vibrancy, accent-color sync
- Messages pane, Scratchpad editor, Terminal content styling beyond the xterm theme
- Custom color themes beyond light/dark, font settings outside the terminal
- Animations beyond a single popover/dialog fade; icon set changes; Windows

## Success demo
Done when I can switch macOS to light mode and watch the app follow, override to Dark in Settings > General, use the four Settings tabs with every change persisting without a Save button (font size applies live to an open terminal), click "+" on a project row and get an anchored popover that creates a session on Enter, and see the sidebar, status bar and all three dialogs on the same tokens with `grep -rE '#[0-9a-fA-F]{3,6}\b' src/lib` returning nothing outside the token file.
