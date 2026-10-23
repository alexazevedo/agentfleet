# agentfleet

Desktop cockpit for running multiple coding agents (Claude Code, Codex, ...) side by side, each in its own git worktree, with a message channel between sessions.

Built with Tauri 2 + Rust + xterm.js.

## Features

- Light and dark theme that follows the OS appearance, with an override (system / light / dark) in Settings > General.

Inspired by [Orchestra](https://github.com/theorchestradev/orchestra) (MIT). Clean-room implementation; no code copied.

## Messaging between sessions

Sessions in one project send each other requests and share a scratchpad through the `agentfleet` CLI: `list`, `send`, `spawn`, `close`, `inbox`, `read`, `reply`, `cancel`, `wait`, and `scratchpad read|write|append`. Run `agentfleet help` for usage. The agent-facing guide is `skills/agentfleet/SKILL.md`; the app injects it into every Claude and Codex session at start (SessionStart hook), and `agentfleet guide` prints it from any terminal, even outside a session. For agents started outside the app, `agentfleet skill install` writes it to `~/.claude/skills/agentfleet/SKILL.md` and `~/.codex/skills/agentfleet/SKILL.md`. Text that arrives from other sessions is treated as data, not instructions.
