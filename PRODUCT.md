---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M6 — session name, worktree opt-out, model in tree)

## What / why
Every new session gets a dedicated git worktree, whether you want one or not, and only CLI `spawn` can set a label (`--name`); the sidebar popover offers nothing but project + agent. For quick sessions that should work directly on the project's current branch — or when the user simply wants no isolation — the forced worktree is friction. The sidebar also says nothing about which model a session runs, so a fleet of Claude/Codex tabs all look alike. This run makes worktree creation optional, exposes a session name on both creation surfaces, and shows each session's live model in the project tree.

## Primary user story
As a user opening the New-session popover, I type a name, uncheck "create worktree", and get a session titled with my name running in the project root on the current branch — and an agent gets the same via `agentfleet spawn --no-worktree --name <label>`. Glancing at the sidebar, I see each session's current model next to it.

## Scope
- **UI (NewSessionPopover):** optional name text field (empty = today's default name) and a "create worktree" checkbox, default checked. Both passed through the existing create path.
- **CLI:** `agentfleet spawn --no-worktree` flag; `--name` already exists and keeps its behavior.
- **No-worktree semantics:** session runs in the project root checkout on whatever branch is currently checked out — no branch created, no checkout switched. Multiple no-worktree sessions may share the tree; that is the user's responsibility. Close of a no-worktree session never touches the working tree (nothing to remove or keep).
- **Name semantics:** display label only — tab/sidebar title, persisted on the session (same field CLI `--name` sets today). Worktree directory and branch names stay id-based.
- **Defaults:** worktree ON everywhere; no new settings entry; popover does not remember the last choice.
- **Persistence:** session records whether it owns a worktree so restart/resume and close behave correctly; `state.json` stays additive (serde defaults).
- **Model in tree:** each session row in the sidebar project tree shows the model the agent is actually running, read live from the agent (hook/status payloads, same channel as session status), updating if the user switches model mid-session. Blank until the first report; persisted so a restored tab shows the last-known model immediately.

## Out of scope
- Settings knob or per-project memory for the worktree default
- Name used for branch/worktree naming, or renaming existing sessions
- Limiting how many no-worktree sessions share the project root
- Choosing or switching the model from agentfleet (display only)
- Changing worktree layout, branch naming, or close/keep rules for worktree sessions
- Windows

## Success demo
Done when I can create a session from the popover with a custom name and worktree unchecked, see the named tab whose terminal is in the project root on the current branch with its model shown in the tree, close it without the checkout being touched — and get the same session via `agentfleet spawn --no-worktree --name review`.
