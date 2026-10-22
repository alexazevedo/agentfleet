---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M3.1 — terminal preferences + restore prompt)

## What / why
Two gaps found while using the M3 build. The terminal is fixed at 13 px in the default font, which is wrong for most screens and eyes. And every launch silently re-attaches every saved session, relaunching Claude Code for each one even when the user wanted a clean start. This run adds terminal preferences to the existing Settings pane and a restore prompt on launch, both persisted in `~/.agentfleet/settings.json` (M3). Shape rules in docs/roadmap.md apply; M3's pending ticket 24 (first releases) stays in docs/forge-m3/ and is unaffected.

## Primary user story
As a solo developer, I set my terminal font and size once and see every open terminal change immediately, and on launch I choose whether yesterday's sessions come back, so that the app fits my screen and does not spin up agents I did not ask for.

## Scope
- Settings pane "Terminal" section: font family (free text, monospace fallback appended), font size 9–24, line height 1.0–1.6, cursor blink on/off. Saved values apply live to all mounted xterm instances (options update + refit) and to new ones.
- Settings pane "Startup" section: restore sessions = ask | always | never (default ask).
- Launch flow: projects always load. When saved sessions exist and the setting is `ask`, a modal "Restore N sessions?" with Restore / Start fresh and a "Don't ask again" checkbox (persists the chosen answer as `always`/`never`). Start fresh forgets the sessions in state.json; worktrees and branches stay on disk. `always`/`never` skip the dialog.
- `settings.json` gains `terminal { fontFamily, fontSize, lineHeight, cursorBlink }` and `restoreSessions`; missing fields fall back to defaults (current behaviour: 13 px, system monospace, blink on, restore always-on → new default `ask`).

## Out of scope
- Cursor style, scrollback size, padding, font smoothing, themes, light mode
- Per-session restore selection; removing worktrees on Start fresh
- Keyboard shortcut for Settings; font picker UI (free text only)

## Success demo
Done when I can change the font family and size in Settings and watch two open terminals re-render immediately, relaunch with three saved sessions and get the "Restore 3 sessions?" dialog, pick Start fresh with "Don't ask again" and get an empty session list with the three worktrees still on disk, then see `restoreSessions: "never"` in settings.json and no dialog on the next launch.
