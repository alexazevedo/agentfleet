---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M2 — inter-session messaging)

## What / why
M1 (docs/forge-m1/) gives a solo developer parallel agent sessions in isolated worktrees with live status. They still relay work between agents by hand: copy a result from one terminal, paste it into another. M2 adds a message broker so sessions send each other requests through an `agentfleet` CLI, the app delivers a request into the recipient's terminal when it is idle, and the reply flows back to the sender. A per-project scratchpad gives agents a shared note. Shape rules in docs/roadmap.md ("Shape fixed in M1") apply: no new entities outside projectId/sessionId, Messages is a tab inside the main pane, status comes from the M1 `session-status` event.

## Primary user story
As a solo developer, I tell one agent "ask the reviewer session to check this branch", and the request lands in the reviewer's terminal by itself, with the reply shown back to the asking agent and in the app, so that I orchestrate a fleet without copy-pasting between terminals.

## Scope (M2)
- Broker: Unix socket (0600) in a per-launch 0700 temp dir, one JSON object per line, per-session token. Every PTY gets `AGENTFLEET_SOCKET`, `AGENTFLEET_SESSION`, `AGENTFLEET_TOKEN` and a PATH shim.
- `agentfleet` CLI = same binary, hidden subcommand: `list`, `send <session> <text>`, `inbox`, `read <run>`, `wait <run>`, `reply <run> <text>`, `cancel <run>`, `scratchpad read|write|append [--if-revision N]`, `guide`.
- Run `{ id, projectId, fromSessionId, toSessionId, prompt, status, response, createdAt }` in `~/.agentfleet/collaboration.json`; states queued, running, completed, cancelled, interrupted, needsAttention. Same project only. Shell sessions send but cannot receive. Caps: 256 KB request, 128 KB reply; prune at 500 runs / 30 days. Closing a session interrupts its running runs.
- Delivery: auto-paste the prompt into the recipient PTY when its status is `waiting` and no draft is typed; Execute button for queued runs; Cancel. Cycle detection on `wait`.
- UI: unread/queued badge on the session row; "Terminal | Messages" tabs in the main pane; Messages shows the inbox history with status chips, Execute/Cancel, and a compose box (target session in same project, Cmd+Enter).
- Scratchpad: one Markdown note per project with a revision counter, 256 KB cap; "Scratchpad" row under each project opens a textarea (read/edit/Save) in the main pane.
- Skill stub: `skills/agentfleet/SKILL.md` teaching agents the CLI; `agentfleet guide` prints it. Content from other sessions is labelled as data.

## Out of scope
- Settings UI (auto-delivery stays on, hardcoded), attention chime, Changes pane
- Cross-project requests; agents other than Claude Code and Shell
- Skill installer via npx; profiles; GitHub; updater/signing/CI (M3)
- Drag-reorder, scratchpad history/diff, run search

## Success demo
Done when I can open two Claude Code sessions in one project, type into A "run `agentfleet send <B> 'list the files in src and reply'`", watch the prompt appear in B's terminal by itself while B is idle, see B's `agentfleet reply` show up in A's `wait` output and in the Messages tab with status completed, write a line to the project scratchpad from the CLI and see it in the sidebar viewer, and relaunch the app with the run history intact.
