---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M5 — spawn sessions from a session + Codex agent)

## What / why
Sessions already talk to each other (`agentfleet send/wait/inbox/reply`), but only a human can create a session, and the only agent kind is Claude Code. An agent that wants a second opinion ("spin up a reviewer for this branch") cannot get one. This run adds (1) `agentfleet spawn`: a session creates a sibling session in the same project, hands it a first prompt as a normal run, waits on it and closes it; (2) Codex CLI as a second agent kind with the same launch / status-hook / resume contract as Claude. Roles are Markdown files prepended to the prompt. Shape rules in docs/roadmap.md apply. M3 ticket 24 (release) stays paused in docs/forge-m3/. M4 (native UI) runs in parallel on main; overlap is `session.rs`, `cli.rs`, `agent.rs` and the new-session control — settle at merge.

## Primary user story
As an agent inside a Claude session, I run `agentfleet spawn --agent codex --role reviewer "Review branch X against main"`, a Codex tab appears in the sidebar with its own worktree, `agentfleet wait <run>` returns the review text, and `agentfleet close <session>` removes the reviewer when I am done.

## Scope
- **CLI `spawn`:** `agentfleet spawn [--agent claude|codex|shell] [--name <label>] [--role <name>] [--agent-arg <arg>]... <prompt...|->`. Creates a session in the caller's project through the existing create path (worktree + PTY + sidebar tab + state.json), then creates a run caller → new session with the prompt. Prints `<session id8> <run id>`. Default agent = settings `default_agent`. Delivery reuses auto-delivery once the new session reports `waiting`. `--agent shell` takes no prompt and creates no run.
- **Roles:** `~/.agentfleet/roles/<name>.md`; `--role` prepends the file body and a blank line to the prompt. Missing file = exit 1 with the path. No UI, no built-in library; skill doc shows an example `reviewer.md`.
- **Agent args passthrough:** each `--agent-arg` is appended verbatim to the agent launch command (e.g. `--agents <json>` for Claude, `-c key=value` for Codex). Persisted on the session so restart / resume keeps them.
- **CLI `close`:** `agentfleet close <session>`, same project only, through the existing `close_session` path (clean worktree removed, dirty kept). Closing yourself is refused.
- **Session label:** `--name` sets a persisted label shown as the tab / sidebar name; empty = today's default name.
- **Codex agent:** `Agent::Codex`. Launch `codex` with its auto-approval flag when auto-permission is on, a `notify` hook that writes `waiting` to the session status file on turn completion, and `working` set when a prompt is delivered; resume on app restart via Codex's resume command (id capture: forge:plan decides). Sidebar "New session" offers Claude / Codex / Shell; `default_agent` accepts `codex` and Settings validates it.
- **Skill doc:** skills/agentfleet/SKILL.md gains Spawn / Close / Roles sections; `agentfleet guide` prints them.
- **Tests (Rust):** CLI parsing for `spawn` / `close` / `--role` / `--agent-arg`; role prepend incl. missing file; Codex launch string and notify settings; spawn order (session, then run) and rejections; close refuses self. `svelte-check` 0 errors.

## Out of scope
- Cross-project spawn or send
- Role presets UI, role editing, shipped role library
- Ephemeral / auto-closing sessions
- Codex profiles, usage or rate-limit views
- Model / effort flags beyond `--agent-arg` passthrough
- Windows

## Success demo
Done when, from a Claude session in the app, `agentfleet spawn --agent codex --role reviewer "review this branch vs main"` makes a Codex tab appear whose status dot goes waiting → working → waiting, `agentfleet wait <run>` prints the review, `agentfleet close <id8>` removes the tab, and the same sequence works with `--agent claude`.
