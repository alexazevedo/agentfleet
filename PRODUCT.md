---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M1)

## What / why
A solo developer running several AI coding agents on one repo loses track of which terminal is doing what and which branch each one is on. agentfleet is a desktop cockpit (Tauri 2 + Rust + xterm.js) where every tab is one agent session in its own git worktree, with a live status badge. M1 is the core loop only: tab -> worktree -> PTY -> terminal -> clean close -> restore on relaunch. Later milestones add inter-session messaging (M2) and release tooling (M3). Reference requirements come from `docs/orchestra-prd.md`; clean-room, no code reuse.

## Primary user story
As a solo developer, I open a tab per task and get Claude Code running in an isolated worktree branch so that parallel agents never collide and I can see at a glance which one is working, waiting, or done.

## Scope (M1)
- Layout: collapsible left sidebar listing Projects (a git repo folder) with their sessions and status dots; main pane shows the active session's terminal; bottom status bar. "+" on a project opens the new-session dialog: pick agent (Claude Code | Shell). "+" next to "Projects" adds a repo folder.
- New tab: `git worktree add -b agentfleet/<agent>-<id>` under `~/.agentfleet/worktrees/<repo>/<id>`, then spawn a login-shell PTY in that worktree. Claude Code tab runs `claude --permission-mode auto --session-id <id> --settings <hooks.json>`; Shell tab runs `$SHELL` only.
- Refuse tab creation with a visible error when: folder is not a git repo, repo has no commits, branch already exists.
- Terminal: xterm.js, output streamed over a Tauri `Channel<Vec<u8>>`, input, resize, copy/paste. Tabs are independent.
- Status badge per tab (working / waiting / ended / unknown) driven by Claude hooks writing a status file, polled by the backend. Shell tabs show no badge.
- Close tab: worktree and branch removed when `git status --porcelain` is clean; otherwise tab closes, worktree kept, badge says "kept".
- State in `~/.agentfleet/state.json` (atomic write): projects and their sessions. On relaunch, sessions restore: worktrees reattached, PTY relaunched; Claude Code tabs resume with `claude --resume <session-id>`.
- Platform-specific code lives behind a `platform/` module; macOS + Linux only.

## Out of scope
- Inter-session messaging, inbox, CLI, broker (M2)
- Updater, code signing, CI release (M3)
- Agents other than Claude Code and plain shell
- Themes, profiles, GitHub integration, usage/cost views, changes/diff pane
- Settings UI beyond what M1 needs
- Windows support
- Permission-mode toggle (auto mode is hardcoded for M1, documented shortcut)
- Project description, emoji, rename, reorder; removing a project that still has sessions
- Settings panel contents (sidebar entry is a placeholder), Changes pane, Messages pane

## Stack constraints (fixed)
Tauri 2; Rust backend with `portable-pty`, tokio, serde; frontend Svelte or Solid + `@xterm/xterm` (not Vue). PTY output over a Tauri `Channel`, never `emit` per chunk.

## Success demo
Done when I can run `npm run tauri dev`, click "+", pick a repo and "Claude Code", see the Claude prompt in xterm inside a fresh `agentfleet/claude-<id>` worktree within 2 s, open a second tab on the same repo with both terminals independent and resizable, watch the badge flip working/waiting, close a clean tab and see its worktree and branch gone (dirty one kept with a "kept" badge), then relaunch the app and get every tab back with Claude resuming its previous conversation.
