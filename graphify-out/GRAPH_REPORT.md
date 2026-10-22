# Graph Report - agentfleet  (2026-10-22)

## Corpus Check
- 59 files · ~27,234 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 464 nodes · 729 edges · 44 communities (42 shown, 2 thin omitted)
- Extraction: 88% EXTRACTED · 12% INFERRED · 0% AMBIGUOUS · INFERRED: 85 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f5fc2a45`
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

## God Nodes (most connected - your core abstractions)
1. `err()` - 25 edges
2. `handle_inner()` - 15 edges
3. `spawn_for()` - 10 edges
4. `create()` - 10 edges
5. `create_session()` - 9 edges
6. `persist()` - 9 edges
7. `serve()` - 9 edges
8. `req()` - 9 edges
9. `Orchestra 0.3.1 Replication PRD` - 9 edges
10. `close_session()` - 8 edges

## Surprising Connections (you probably didn't know these)
- `serve()` --calls--> `lookup()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `create_session()` --calls--> `create()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/worktree.rs
- `spawn_for()` --calls--> `home_dir()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/platform/unix.rs
- `spawn_for()` --calls--> `spawn()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/pty.rs
- `spawn_for()` --calls--> `login_shell()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/platform/unix.rs

## Communities (44 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.07
Nodes (35): home_dir(), add_project(), add_to(), add_twice_same_id(), Project, remove_from(), remove_project(), remove_rules() (+27 more)

### Community 1 - "Community 1"
Cohesion: 0.11
Nodes (40): auth(), ct_eq(), err(), handle_inner(), participant(), run_id(), scratchpad_op(), text_of() (+32 more)

### Community 2 - "Community 2"
Cohesion: 0.09
Nodes (20): @xterm/addon-fit, @tauri-apps/api/core, error, onKey(), otherAgent, send(), target, text (+12 more)

### Community 3 - "Community 3"
Cohesion: 0.08
Nodes (15): collab, Run, RunStatus, AgentKind, CloseOutcome, closeTab(), early, loadProjects() (+7 more)

### Community 4 - "Community 4"
Cohesion: 0.17
Nodes (18): connection(), fixture(), handle(), list_filters_to_source_project(), req(), Request, roundtrip(), serve() (+10 more)

### Community 5 - "Community 5"
Cohesion: 0.21
Nodes (16): append(), append_in(), append_separator_only_when_non_empty(), dir(), emit_changed(), missing_is_empty(), parse_id(), read() (+8 more)

### Community 6 - "Community 6"
Cohesion: 0.18
Nodes (12): login_shell(), claude_launch(), launch_flags(), settings_roundtrip(), shell_quote(), write_hooks_settings(), spawn_for(), emit() (+4 more)

### Community 7 - "Community 7"
Cohesion: 0.2
Nodes (12): changed(), set_status(), apply_draft(), can_deliver(), cancel_run(), Decision, execute_run(), on_request_created() (+4 more)

### Community 8 - "Community 8"
Cohesion: 0.34
Nodes (13): branch_exists(), clean_then_dirty(), create(), create_succeeds(), git(), is_clean(), no_commits(), not_a_repo() (+5 more)

### Community 9 - "Community 9"
Cohesion: 0.26
Nodes (11): call(), call_timeout(), Cmd, env(), main(), obj(), parse(), parse_scratch() (+3 more)

### Community 10 - "Community 10"
Cohesion: 0.29
Nodes (9): agentfleet (M1), agentfleet (M2 — inter-session messaging), Out of scope, Primary user story, Scope (M1), Scope (M2), Stack constraints (fixed), Success demo (+1 more)

### Community 11 - "Community 11"
Cohesion: 0.2
Nodes (9): Architecture, Feature inventory, How it was built, Integrations and network, Orchestra 0.3.1 Replication PRD, Overview, Replication requirements, UI and UX (+1 more)

### Community 12 - "Community 12"
Cohesion: 0.25
Nodes (3): Listener, listeners, Scratchpad

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
Cohesion: 0.25
Nodes (7): agentfleet, Delegate work, Exit codes, Find sessions, Handle incoming work, Ids, Scratchpad

### Community 17 - "Community 17"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:json ({"hooks":{), Context, Done when, Goal, Not included

### Community 18 - "Community 18"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, ), Context, Done when, Goal, Not included

### Community 19 - "Community 19"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

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
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 26 - "Community 26"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 27 - "Community 27"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 28 - "Community 28"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 29 - "Community 29"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 30 - "Community 30"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 31 - "Community 31"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 32 - "Community 32"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 33 - "Community 33"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 34 - "Community 34"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 35 - "Community 35"
Cohesion: 0.33
Nodes (5): agentfleet roadmap, Later / optional, M2 — inter-session messaging, M3 — release, Shape fixed in M1 (do not rework)

### Community 36 - "Community 36"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Knowledge Gaps
- **130 isolated node(s):** `Session`, `SessionInfo`, `CloseOutcome`, `Scratchpad`, `Decision` (+125 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `err()` connect `Community 1` to `Community 0`, `Community 4`, `Community 5`, `Community 8`, `Community 9`?**
  _High betweenness centrality (0.071) - this node is a cross-community bridge._
- **Why does `spawn_for()` connect `Community 6` to `Community 0`, `Community 4`?**
  _High betweenness centrality (0.019) - this node is a cross-community bridge._
- **Why does `changed()` connect `Community 7` to `Community 0`, `Community 1`, `Community 4`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **Are the 16 inferred relationships involving `err()` (e.g. with `.parse()` and `attach_session()`) actually correct?**
  _`err()` has 16 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `handle_inner()` (e.g. with `lookup()` and `resolve_id()`) actually correct?**
  _`handle_inner()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `spawn_for()` (e.g. with `home_dir()` and `write_hooks_settings()`) actually correct?**
  _`spawn_for()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `create()` (e.g. with `err()` and `create_session()`) actually correct?**
  _`create()` has 2 INFERRED edges - model-reasoned connections that need verification._