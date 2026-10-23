---
id: 36
status: done
model: haiku
depends_on: [35]
max_attempts: 3
files:
  - src-tauri/src/cli.rs
  - skills/agentfleet/SKILL.md
---
## Goal
`agentfleet spawn --no-worktree` creates the sibling session in the project root (no worktree).

## Context
`parse_spawn` in src-tauri/src/cli.rs parses value-taking `--flag val` pairs; `--no-worktree` is boolean, so handle it before the `.ok_or(format!("{flag} needs a value"))?` lookup (match the flag, set `worktree = false`, `i += 1`, continue). Add `worktree: bool` (default true) to `Cmd::Spawn`; when false, `run()` inserts `"worktree": false` into the `session.spawn` call map (broker field added in ticket 35). Update the `USAGE` string spawn line to `spawn [--agent claude|codex|shell] [--no-worktree] [--name <label>] [--role <name>] [--agent-arg <arg>]... <prompt...|->`, and the same line plus one explanatory sentence ("--no-worktree runs the session in the project root on the current branch") in skills/agentfleet/SKILL.md's Spawn section. Extend the `parses_verbs` test: `--no-worktree` parses, combines with other flags, and plain spawn defaults to worktree=true.

## Checks (run in order — cheapest first)
1. `cargo test --manifest-path src-tauri/Cargo.toml`

## Done when
- `agentfleet spawn --no-worktree "x"` parses to `Cmd::Spawn { worktree: false, .. }`; without the flag `worktree: true`.
- SKILL.md documents the flag.

## Not included
- UI (ticket 37). No `--worktree` positive flag. No change to `close` output.
