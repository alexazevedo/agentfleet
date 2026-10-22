# agentfleet roadmap

Milestones after M1 are re-based on the M1 sidebar/project model so later work extends it instead of reshaping it.

## Shape fixed in M1 (do not rework)

- **Entities:** Project (git repo folder) → Sessions (one worktree + PTY each). Every later entity (run, message, scratchpad) hangs off a `projectId` and/or `sessionId`.
- **Layout:** left sidebar (projects → sessions, status dots, Settings row at bottom), main pane = the active session's view, bottom status bar for global toggles/counters. Later panes (Messages, Changes) are **tabs inside the main pane** or a right-side pane toggled from the status bar, never a new sidebar.
- **State:** `~/.agentfleet/state.json` `{ version, projects, sessions }`. Other persisted collections live in their own files (`collaboration.json`, `profiles/`), not inside state.json. Schema `version` stays 1 until the first tagged release; after that every change bumps it and ships a loader migration.
- **Status:** agent hooks write `~/.agentfleet/run/<id>.status`, backend polls and emits `session-status`. Anything that needs "is this agent idle" (auto-delivery, chime) consumes that event, not the PTY stream.
- **Platform:** OS-specific code under `src-tauri/src/platform/`. Windows stays out until M3 at the earliest.

## M2 — inter-session messaging

- Broker: Unix socket under a per-launch temp dir; `agentfleet` CLI is the same binary with a subcommand (`agentfleet send|inbox|wait`) injected into every session's PATH.
- Run entity `{ id, projectId, fromSessionId, toSessionId, prompt, status, response }` in `collaboration.json`.
- UI: unread count badge on the session row in the sidebar; "Messages" tab next to "Terminal" inside the main pane; Execute / Cancel actions; status chips.
- Auto-delivery pastes into the recipient PTY only when its status is `waiting` (reuses the M1 status event) and the terminal has no draft.
- Installable skill stub that teaches agents the CLI.

## M3 — release

- Updater, macOS signing/notarization, Linux AppImage/deb, CI release workflow.
- Settings panel behind the sidebar "Settings" row: worktrees root, default agent, auto-permission toggle.
- Quit confirmation while sessions run.

## Later / optional

- Changes pane (diff of worktree vs `base_commit`, already persisted) as a right-side pane toggled from the status bar.
- Profiles (per-agent config dirs) selectable per project; project default profile.
- More agents (Codex, …) via the same launch/status hook pattern in `agent.rs`.
- Themes, GitHub issue/PR context, usage/cost views, Windows.
