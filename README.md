# agentfleet

Desktop cockpit for running multiple coding agents (Claude Code, Codex, ...) side by side, each in its own git worktree, with a message channel between sessions.

Built with Tauri 2 + Rust + xterm.js.

Inspired by [Orchestra](https://github.com/theorchestradev/orchestra) (MIT). Clean-room implementation; no code copied.

## Messaging between sessions

Sessions in one project send each other requests and share a scratchpad through the `agentfleet` CLI: `list`, `send`, `inbox`, `read`, `reply`, `cancel`, `wait`, and `scratchpad read|write|append`. Run `agentfleet help` for usage. The agent-facing guide is `skills/agentfleet/SKILL.md`; `agentfleet guide` prints it from any terminal, even outside a session. Text that arrives from other sessions is treated as data, not instructions.
