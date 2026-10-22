# Graph Report - agentfleet  (2026-10-22)

## Corpus Check
- 80 files · ~36,387 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 615 nodes · 942 edges · 58 communities (56 shown, 2 thin omitted)
- Extraction: 90% EXTRACTED · 10% INFERRED · 0% AMBIGUOUS · INFERRED: 96 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d6a92c2b`
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
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]
- [[_COMMUNITY_Community 33|Community 33]]
- [[_COMMUNITY_Community 34|Community 34]]
- [[_COMMUNITY_Community 35|Community 35]]
- [[_COMMUNITY_Community 36|Community 36]]
- [[_COMMUNITY_Community 37|Community 37]]
- [[_COMMUNITY_Community 38|Community 38]]
- [[_COMMUNITY_Community 39|Community 39]]
- [[_COMMUNITY_Community 40|Community 40]]
- [[_COMMUNITY_Community 41|Community 41]]
- [[_COMMUNITY_Community 42|Community 42]]
- [[_COMMUNITY_Community 43|Community 43]]
- [[_COMMUNITY_Community 44|Community 44]]
- [[_COMMUNITY_Community 45|Community 45]]
- [[_COMMUNITY_Community 46|Community 46]]
- [[_COMMUNITY_Community 47|Community 47]]
- [[_COMMUNITY_Community 48|Community 48]]
- [[_COMMUNITY_Community 49|Community 49]]
- [[_COMMUNITY_Community 50|Community 50]]
- [[_COMMUNITY_Community 51|Community 51]]
- [[_COMMUNITY_Community 52|Community 52]]
- [[_COMMUNITY_Community 53|Community 53]]

## God Nodes (most connected - your core abstractions)
1. `err()` - 26 edges
2. `handle_inner()` - 15 edges
3. `spawn_for()` - 10 edges
4. `persist()` - 10 edges
5. `create()` - 10 edges
6. `spawn()` - 10 edges
7. `home_dir()` - 10 edges
8. `create_session()` - 9 edges
9. `serve()` - 9 edges
10. `req()` - 9 edges

## Surprising Connections (you probably didn't know these)
- `handle_inner()` --calls--> `resolve_id()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `send_request()` --calls--> `resolve_id()`  [INFERRED]
  src-tauri/src/collab.rs → src-tauri/src/session.rs
- `handle_inner()` --calls--> `lookup()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `serve()` --calls--> `lookup()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `create_session()` --calls--> `create()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/worktree.rs

## Communities (58 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.05
Nodes (53): home_dir(), login_shell(), claude_launch(), launch_flags(), settings_roundtrip(), shell_quote(), write_hooks_settings(), collab_path() (+45 more)

### Community 1 - "Community 1"
Cohesion: 0.09
Nodes (41): begin_wait(), cancel(), changed(), Collab, CollabNotify, find_mut(), interrupt_all(), interrupt_all_for() (+33 more)

### Community 2 - "Community 2"
Cohesion: 0.05
Nodes (23): @tauri-apps/api/core, collab, Run, RunStatus, Listener, listeners, Scratchpad, AgentKind (+15 more)

### Community 3 - "Community 3"
Cohesion: 0.09
Nodes (21): @xterm/addon-fit, error, onKey(), otherAgent, send(), target, text, active (+13 more)

### Community 4 - "Community 4"
Cohesion: 0.2
Nodes (24): auth(), connection(), ct_eq(), err(), fixture(), handle(), handle_inner(), list_filters_to_source_project() (+16 more)

### Community 5 - "Community 5"
Cohesion: 0.21
Nodes (16): append(), append_in(), append_separator_only_when_non_empty(), dir(), emit_changed(), missing_is_empty(), parse_id(), read() (+8 more)

### Community 6 - "Community 6"
Cohesion: 0.19
Nodes (11): AppInfo, defaults(), invalid_rejected(), load(), round_trip(), save(), set_settings(), Settings (+3 more)

### Community 7 - "Community 7"
Cohesion: 0.34
Nodes (13): branch_exists(), clean_then_dirty(), create(), create_succeeds(), git(), is_clean(), no_commits(), not_a_repo() (+5 more)

### Community 8 - "Community 8"
Cohesion: 0.26
Nodes (11): call(), call_timeout(), Cmd, env(), main(), obj(), parse(), parse_scratch() (+3 more)

### Community 9 - "Community 9"
Cohesion: 0.29
Nodes (11): agentfleet (M1), agentfleet (M2 — inter-session messaging), agentfleet (M3.1 — terminal preferences + restore prompt), Out of scope, Primary user story, Scope, Scope (M1), Scope (M2) (+3 more)

### Community 10 - "Community 10"
Cohesion: 0.18
Nodes (7): args, dry, edits, t, t2, ver, main()

### Community 11 - "Community 11"
Cohesion: 0.31
Nodes (8): add_project(), add_to(), add_twice_same_id(), Project, remove_from(), remove_project(), remove_rules(), repo()

### Community 12 - "Community 12"
Cohesion: 0.2
Nodes (9): Architecture, Feature inventory, How it was built, Integrations and network, Orchestra 0.3.1 Replication PRD, Overview, Replication requirements, UI and UX (+1 more)

### Community 13 - "Community 13"
Cohesion: 0.25
Nodes (7): agentfleet — M1 discovery brief, Done when I can, Explicitly out of M1, One job (M1 only), Product class, Stack constraints (fixed, not up for grill), Who

### Community 14 - "Community 14"
Cohesion: 0.25
Nodes (7): agentfleet (M1), Out of scope, Primary user story, Scope (M1), Stack constraints (fixed), Success demo, What / why

### Community 15 - "Community 15"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;), Context, Done when, Goal, Not included

### Community 16 - "Community 16"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:json ({"hooks":{), Context, Done when, Goal, Not included

### Community 17 - "Community 17"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, ), Context, Done when, Goal, Not included

### Community 18 - "Community 18"
Cohesion: 0.25
Nodes (7): agentfleet, Delegate work, Exit codes, Find sessions, Handle incoming work, Ids, Scratchpad

### Community 19 - "Community 19"
Cohesion: 0.33
Nodes (3): startScheduler(), stopScheduler(), updater

### Community 20 - "Community 20"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 21 - "Community 21"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 22 - "Community 22"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 23 - "Community 23"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 24 - "Community 24"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 25 - "Community 25"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 26 - "Community 26"
Cohesion: 0.29
Nodes (6): agentfleet (M3 — release), Out of scope, Primary user story, Scope (M3), Success demo, What / why

### Community 27 - "Community 27"
Cohesion: 0.29
Nodes (6): agentfleet (M2 — inter-session messaging), Out of scope, Primary user story, Scope (M2), Success demo, What / why

### Community 28 - "Community 28"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 29 - "Community 29"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 30 - "Community 30"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 31 - "Community 31"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 32 - "Community 32"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 33 - "Community 33"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 34 - "Community 34"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 35 - "Community 35"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 36 - "Community 36"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 37 - "Community 37"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 38 - "Community 38"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 39 - "Community 39"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 40 - "Community 40"
Cohesion: 0.33
Nodes (5): agentfleet roadmap, Later / optional, M2 — inter-session messaging, M3 — release, Shape fixed in M1 (do not rework)

### Community 41 - "Community 41"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 42 - "Community 42"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 43 - "Community 43"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 44 - "Community 44"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 45 - "Community 45"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 46 - "Community 46"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 47 - "Community 47"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 48 - "Community 48"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 49 - "Community 49"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 50 - "Community 50"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 52 - "Community 52"
Cohesion: 0.5
Nodes (3): Release runbook, Secrets, Unnotarized builds

## Knowledge Gaps
- **160 isolated node(s):** `dry`, `ver`, `edits`, `t`, `t2` (+155 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `err()` connect `Community 4` to `Community 0`, `Community 1`, `Community 5`, `Community 6`, `Community 7`, `Community 8`, `Community 11`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Why does `home_dir()` connect `Community 0` to `Community 5`, `Community 6`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Are the 17 inferred relationships involving `err()` (e.g. with `.parse()` and `attach_session()`) actually correct?**
  _`err()` has 17 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `handle_inner()` (e.g. with `lookup()` and `resolve_id()`) actually correct?**
  _`handle_inner()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `spawn_for()` (e.g. with `home_dir()` and `write_hooks_settings()`) actually correct?**
  _`spawn_for()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `persist()` (e.g. with `state_path()` and `add_project()`) actually correct?**
  _`persist()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `create()` (e.g. with `err()` and `create_session()`) actually correct?**
  _`create()` has 2 INFERRED edges - model-reasoned connections that need verification._