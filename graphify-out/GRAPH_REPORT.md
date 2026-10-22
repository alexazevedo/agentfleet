# Graph Report - agentfleet  (2026-10-22)

## Corpus Check
- 36 files · ~13,984 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 230 nodes · 295 edges · 28 communities (27 shown, 1 thin omitted)
- Extraction: 90% EXTRACTED · 10% INFERRED · 0% AMBIGUOUS · INFERRED: 30 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4df3a0e9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 16|Community 16]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 24|Community 24]]

## God Nodes (most connected - your core abstractions)
1. `spawn_for()` - 10 edges
2. `create_session()` - 9 edges
3. `persist()` - 9 edges
4. `create()` - 9 edges
5. `Orchestra 0.3.1 Replication PRD` - 9 edges
6. `list_restorable()` - 7 edges
7. `agentfleet (M1)` - 7 edges
8. `agentfleet — M1 discovery brief` - 7 edges
9. `close_session()` - 6 edges
10. `repo_with_commit()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `create_session()` --calls--> `create()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/worktree.rs
- `spawn_for()` --calls--> `home_dir()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/platform/unix.rs
- `spawn_for()` --calls--> `login_shell()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/platform/unix.rs
- `persist()` --calls--> `save()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/state.rs
- `add_project()` --calls--> `persist()`  [INFERRED]
  src-tauri/src/project.rs → src-tauri/src/session.rs

## Communities (28 total, 1 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.12
Nodes (21): home_dir(), Agent, attach_session(), close_decision(), close_session(), CloseOutcome, create_session(), info_for() (+13 more)

### Community 1 - "Community 1"
Cohesion: 0.14
Nodes (15): login_shell(), claude_launch(), launch_flags(), settings_roundtrip(), shell_quote(), write_hooks_settings(), runs_command_and_streams_output(), spawn() (+7 more)

### Community 2 - "Community 2"
Cohesion: 0.12
Nodes (12): AgentKind, CloseOutcome, closeTab(), early, loadProjects(), Project, remove(), restoreTabs() (+4 more)

### Community 3 - "Community 3"
Cohesion: 0.18
Nodes (10): @xterm/addon-fit, @tauri-apps/api/core, active, left, running, active, @tauri-apps/plugin-dialog, ./sessions.svelte (+2 more)

### Community 4 - "Community 4"
Cohesion: 0.34
Nodes (13): branch_exists(), clean_then_dirty(), create(), create_succeeds(), git(), is_clean(), no_commits(), not_a_repo() (+5 more)

### Community 5 - "Community 5"
Cohesion: 0.23
Nodes (7): PtyHandle, bad_json_errors(), PersistedSession, round_trip(), sample(), save(), wrong_version_errors()

### Community 6 - "Community 6"
Cohesion: 0.31
Nodes (8): add_project(), add_to(), add_twice_same_id(), Project, remove_from(), remove_project(), remove_rules(), repo()

### Community 7 - "Community 7"
Cohesion: 0.2
Nodes (9): Architecture, Feature inventory, How it was built, Integrations and network, Orchestra 0.3.1 Replication PRD, Overview, Replication requirements, UI and UX (+1 more)

### Community 8 - "Community 8"
Cohesion: 0.25
Nodes (7): agentfleet (M1), Out of scope, Primary user story, Scope (M1), Stack constraints (fixed), Success demo, What / why

### Community 9 - "Community 9"
Cohesion: 0.25
Nodes (7): agentfleet — M1 discovery brief, Done when I can, Explicitly out of M1, One job (M1 only), Product class, Stack constraints (fixed, not up for grill), Who

### Community 10 - "Community 10"
Cohesion: 0.29
Nodes (6): Checks (run in order — cheapest first), code:rust (pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;), Context, Done when, Goal, Not included

### Community 11 - "Community 11"
Cohesion: 0.29
Nodes (6): Checks (run in order — cheapest first), code:json ({"hooks":{), Context, Done when, Goal, Not included

### Community 12 - "Community 12"
Cohesion: 0.29
Nodes (6): Checks (run in order — cheapest first), code:rust (pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, ), Context, Done when, Goal, Not included

### Community 13 - "Community 13"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 14 - "Community 14"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 15 - "Community 15"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 16 - "Community 16"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 17 - "Community 17"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 18 - "Community 18"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 19 - "Community 19"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 20 - "Community 20"
Cohesion: 0.33
Nodes (5): agentfleet roadmap, Later / optional, M2 — inter-session messaging, M3 — release, Shape fixed in M1 (do not rework)

## Knowledge Gaps
- **95 isolated node(s):** `Session`, `SessionInfo`, `CloseOutcome`, `Status`, `StatusEvent` (+90 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `spawn_for()` connect `Community 1` to `Community 0`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `persist()` connect `Community 0` to `Community 5`, `Community 6`?**
  _High betweenness centrality (0.039) - this node is a cross-community bridge._
- **Why does `create_session()` connect `Community 0` to `Community 1`, `Community 4`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `spawn_for()` (e.g. with `home_dir()` and `write_hooks_settings()`) actually correct?**
  _`spawn_for()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `create_session()` (e.g. with `home_dir()` and `create()`) actually correct?**
  _`create_session()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `persist()` (e.g. with `save()` and `state_path()`) actually correct?**
  _`persist()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Session`, `SessionInfo`, `CloseOutcome` to the rest of the system?**
  _95 weakly-connected nodes found - possible documentation gaps or missing edges._