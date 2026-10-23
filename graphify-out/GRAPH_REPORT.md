# Graph Report - spawn-codex  (2026-10-22)

## Corpus Check
- 90 files · ~44,265 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 762 nodes · 1187 edges · 71 communities (69 shown, 2 thin omitted)
- Extraction: 90% EXTRACTED · 10% INFERRED · 0% AMBIGUOUS · INFERRED: 123 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0f121eb4`
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
- [[_COMMUNITY_Community 54|Community 54]]
- [[_COMMUNITY_Community 55|Community 55]]
- [[_COMMUNITY_Community 56|Community 56]]
- [[_COMMUNITY_Community 57|Community 57]]
- [[_COMMUNITY_Community 58|Community 58]]
- [[_COMMUNITY_Community 59|Community 59]]
- [[_COMMUNITY_Community 60|Community 60]]
- [[_COMMUNITY_Community 61|Community 61]]
- [[_COMMUNITY_Community 62|Community 62]]
- [[_COMMUNITY_Community 63|Community 63]]
- [[_COMMUNITY_Community 64|Community 64]]
- [[_COMMUNITY_Community 65|Community 65]]
- [[_COMMUNITY_Community 67|Community 67]]

## God Nodes (most connected - your core abstractions)
1. `err()` - 35 edges
2. `handle_inner()` - 15 edges
3. `spawn_for()` - 14 edges
4. `persist()` - 13 edges
5. `home_dir()` - 12 edges
6. `create_session()` - 11 edges
7. `close_one()` - 11 edges
8. `serve()` - 11 edges
9. `run()` - 11 edges
10. `spawn()` - 10 edges

## Surprising Connections (you probably didn't know these)
- `register()` --calls--> `err()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/broker.rs
- `spawn()` --calls--> `register()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `create_session()` --calls--> `err()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/broker.rs
- `spawn_for()` --calls--> `home_dir()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/platform/unix.rs
- `spawn_for()` --calls--> `err()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/broker.rs

## Communities (71 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (55): home_dir(), collab_path(), run(), add_project(), add_to(), add_twice_same_id(), Project, remove_from() (+47 more)

### Community 1 - "Community 1"
Cohesion: 0.08
Nodes (60): auth(), close(), close_target(), close_target_rules(), connection(), ct_eq(), err(), fixture() (+52 more)

### Community 2 - "Community 2"
Cohesion: 0.08
Nodes (34): @xterm/addon-fit, error, onKey(), otherAgent, send(), target, text, busy (+26 more)

### Community 3 - "Community 3"
Cohesion: 0.07
Nodes (35): login_shell(), claude_launch(), codex_hook_and_session_id(), codex_hook_overrides(), codex_launch(), codex_launch_flags(), codex_overrides(), codex_session_id() (+27 more)

### Community 4 - "Community 4"
Cohesion: 0.11
Nodes (23): agentfleet (M1), agentfleet (M2 — inter-session messaging), agentfleet (M3.2 — remove project), Out of scope, Primary user story, Scope, Scope (M1), Scope (M2) (+15 more)

### Community 5 - "Community 5"
Cohesion: 0.1
Nodes (11): ClosedSession, CloseOutcome, closeTab(), early, labelOf(), Project, remove(), SessionInfo (+3 more)

### Community 6 - "Community 6"
Cohesion: 0.21
Nodes (16): append(), append_in(), append_separator_only_when_non_empty(), dir(), emit_changed(), missing_is_empty(), parse_id(), read() (+8 more)

### Community 7 - "Community 7"
Cohesion: 0.2
Nodes (15): call(), call_timeout(), Cmd, env(), main(), obj(), parse(), parse_scratch() (+7 more)

### Community 8 - "Community 8"
Cohesion: 0.19
Nodes (11): AppInfo, defaults(), invalid_rejected(), load(), round_trip(), save(), set_settings(), Settings (+3 more)

### Community 9 - "Community 9"
Cohesion: 0.12
Nodes (15): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total., Blockers, Decisions, Next (+7 more)

### Community 10 - "Community 10"
Cohesion: 0.23
Nodes (10): apply_draft(), auto_delivery(), can_deliver(), Decision, execute_run(), on_request_created(), on_status(), paste_bytes() (+2 more)

### Community 11 - "Community 11"
Cohesion: 0.18
Nodes (7): args, dry, edits, t, t2, ver, main()

### Community 12 - "Community 12"
Cohesion: 0.18
Nodes (4): collab, Run, RunStatus, store

### Community 13 - "Community 13"
Cohesion: 0.18
Nodes (10): Broker, Checks (run in order — cheapest first), CLI, code:rust (Spawn { agent: Option<String>, name: Option<String>, role: O), Context, Docs, Done when, Frontend (+2 more)

### Community 14 - "Community 14"
Cohesion: 0.18
Nodes (10): agentfleet, Close a session, Delegate work, Exit codes, Find sessions, Handle incoming work, Ids, Roles (+2 more)

### Community 15 - "Community 15"
Cohesion: 0.2
Nodes (9): Backend, Checks (run in order — cheapest first), code:sh (#!/bin/sh), Context, Done when, Frontend, Goal, Not included (+1 more)

### Community 16 - "Community 16"
Cohesion: 0.2
Nodes (9): Architecture, Feature inventory, How it was built, Integrations and network, Orchestra 0.3.1 Replication PRD, Overview, Replication requirements, UI and UX (+1 more)

### Community 17 - "Community 17"
Cohesion: 0.22
Nodes (4): @tauri-apps/api/core, Listener, listeners, Scratchpad

### Community 18 - "Community 18"
Cohesion: 0.25
Nodes (7): Backend changes, Checks (run in order — cheapest first), Context, Done when, Frontend changes, Goal, Not included

### Community 19 - "Community 19"
Cohesion: 0.25
Nodes (7): agentfleet — M1 discovery brief, Done when I can, Explicitly out of M1, One job (M1 only), Product class, Stack constraints (fixed, not up for grill), Who

### Community 20 - "Community 20"
Cohesion: 0.25
Nodes (7): agentfleet (M1), Out of scope, Primary user story, Scope (M1), Stack constraints (fixed), Success demo, What / why

### Community 21 - "Community 21"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:json ({"hooks":{), Context, Done when, Goal, Not included

### Community 22 - "Community 22"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, ), Context, Done when, Goal, Not included

### Community 23 - "Community 23"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:ts (export async function removeProject(id: string): Promise<voi), Context, Done when, Goal, Not included

### Community 24 - "Community 24"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (#[derive(Serialize, Debug, PartialEq)]), Context, Done when, Goal, Not included

### Community 25 - "Community 25"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;), Context, Done when, Goal, Not included

### Community 26 - "Community 26"
Cohesion: 0.29
Nodes (4): AgentKind, AppInfo, Settings, TerminalSettings

### Community 27 - "Community 27"
Cohesion: 0.33
Nodes (3): startScheduler(), stopScheduler(), updater

### Community 28 - "Community 28"
Cohesion: 0.29
Nodes (6): agentfleet, Messaging between sessions, agentfleet, Messaging between sessions, agentfleet, Messaging between sessions

### Community 29 - "Community 29"
Cohesion: 0.29
Nodes (6): agentfleet (M3.1 — terminal preferences + restore prompt), Out of scope, Primary user story, Scope, Success demo, What / why

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
Cohesion: 0.29
Nodes (6): agentfleet (M3.2 — remove project), Out of scope, Primary user story, Scope, Success demo, What / why

### Community 37 - "Community 37"
Cohesion: 0.29
Nodes (6): agentfleet (M3 — release), Out of scope, Primary user story, Scope (M3), Success demo, What / why

### Community 38 - "Community 38"
Cohesion: 0.29
Nodes (6): agentfleet (M2 — inter-session messaging), Out of scope, Primary user story, Scope (M2), Success demo, What / why

### Community 39 - "Community 39"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 40 - "Community 40"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 41 - "Community 41"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 42 - "Community 42"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 43 - "Community 43"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 44 - "Community 44"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 45 - "Community 45"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 46 - "Community 46"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 47 - "Community 47"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 48 - "Community 48"
Cohesion: 0.33
Nodes (5): agentfleet roadmap, Later / optional, M2 — inter-session messaging, M3 — release, Shape fixed in M1 (do not rework)

### Community 49 - "Community 49"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 50 - "Community 50"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 51 - "Community 51"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 52 - "Community 52"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 53 - "Community 53"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 54 - "Community 54"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 55 - "Community 55"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 56 - "Community 56"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 57 - "Community 57"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 58 - "Community 58"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 59 - "Community 59"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 60 - "Community 60"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 61 - "Community 61"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 62 - "Community 62"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 64 - "Community 64"
Cohesion: 0.5
Nodes (3): Release runbook, Secrets, Unnotarized builds

### Community 65 - "Community 65"
Cohesion: 1.0
Nodes (3): listRestorable(), loadProjects(), restoreTabs()

## Knowledge Gaps
- **231 isolated node(s):** `dry`, `ver`, `edits`, `t`, `t2` (+226 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `err()` connect `Community 1` to `Community 0`, `Community 3`, `Community 6`, `Community 7`, `Community 8`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **Why does `spawn_for()` connect `Community 3` to `Community 0`, `Community 1`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **Why does `home_dir()` connect `Community 0` to `Community 8`, `Community 3`, `Community 6`, `Community 7`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Are the 23 inferred relationships involving `err()` (e.g. with `.parse()` and `register()`) actually correct?**
  _`err()` has 23 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `handle_inner()` (e.g. with `lookup()` and `resolve_id()`) actually correct?**
  _`handle_inner()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 11 inferred relationships involving `spawn_for()` (e.g. with `home_dir()` and `err()`) actually correct?**
  _`spawn_for()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `persist()` (e.g. with `state_path()` and `add_project()`) actually correct?**
  _`persist()` has 4 INFERRED edges - model-reasoned connections that need verification._