---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M3.2 — remove project)

## What / why
A project added to the sidebar can never be taken out again. The backend command `remove_project` exists (src-tauri/src/project.rs) and refuses while the project still has sessions, but nothing in the frontend calls it and the sidebar has no control for it. Stale repos pile up in `~/.agentfleet/state.json` and the sidebar. This run wires removal end to end: a sidebar action, a confirmation, and a cascade that closes the project's sessions first so removal always succeeds. Shape rules in docs/roadmap.md apply; M3 ticket 24 (first release) stays paused in docs/forge-m3/.

## Primary user story
As a solo developer, I remove a project from the sidebar with one confirmed click, even while it has open sessions, so that the app shows only the repos I am still working on.

## Scope
- Sidebar project row: a Remove control (hover icon or row menu) next to "New session".
- Confirmation modal, always shown: "Remove <name>? N open sessions will be closed. Clean worktrees are deleted, dirty ones stay on disk. The repo folder is not touched." Remove / Cancel. N = 0 shows a shorter message.
- Cascade: Remove closes each of the project's sessions through the existing `close_session` path (PTY killed, collab runs interrupted, clean worktree removed, dirty worktree kept and its path reported), then calls `remove_project`. Order: sessions first, project last, so a failure mid-way leaves a consistent state.json.
- Frontend store: project and its tabs disappear from the sidebar; if the active view belonged to that project (session tab or scratchpad), the active view moves to a neighbour or the empty state.
- Failure surfacing: if a session close or the final remove errors, the project stays, the error shows in the existing status/toast channel, and already-closed sessions are gone.
- Tests: Rust unit tests for the cascade ordering and the kept-worktree case; existing `remove_from` tests stay green.

## Out of scope
- Deleting the repo folder, dirty worktrees, or branches
- Deleting the project's scratchpad file (`~/.agentfleet/scratchpads`); it stays on disk
- Undo, multi-select, keyboard shortcut for removal
- Reordering or renaming projects

## Success demo
Done when I can add a repo, open two sessions in it (one with uncommitted changes), click Remove, confirm, and see the project and both tabs vanish from the sidebar, the clean worktree gone from disk, the dirty worktree still on disk, and the project absent from `~/.agentfleet/state.json` after relaunch.
