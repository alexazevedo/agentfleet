# Graph Report - agentfleet  (2026-10-23)

## Corpus Check
- 126 files · ~55,810 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 967 nodes · 1502 edges · 87 communities (85 shown, 2 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 134 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `43142076`
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
- [[_COMMUNITY_Community 66|Community 66]]
- [[_COMMUNITY_Community 67|Community 67]]
- [[_COMMUNITY_Community 68|Community 68]]
- [[_COMMUNITY_Community 69|Community 69]]
- [[_COMMUNITY_Community 70|Community 70]]
- [[_COMMUNITY_Community 71|Community 71]]
- [[_COMMUNITY_Community 72|Community 72]]
- [[_COMMUNITY_Community 73|Community 73]]
- [[_COMMUNITY_Community 74|Community 74]]
- [[_COMMUNITY_Community 75|Community 75]]
- [[_COMMUNITY_Community 76|Community 76]]
- [[_COMMUNITY_Community 77|Community 77]]
- [[_COMMUNITY_Community 78|Community 78]]
- [[_COMMUNITY_Community 79|Community 79]]
- [[_COMMUNITY_Community 80|Community 80]]
- [[_COMMUNITY_Community 82|Community 82]]

## God Nodes (most connected - your core abstractions)
1. `err()` - 37 edges
2. `spawn_for()` - 17 edges
3. `handle_inner()` - 15 edges
4. `persist()` - 14 edges
5. `home_dir()` - 13 edges
6. `create_session()` - 11 edges
7. `close_one()` - 11 edges
8. `serve()` - 11 edges
9. `run()` - 11 edges
10. `./sessions.svelte` - 11 edges

## Surprising Connections (you probably didn't know these)
- `onKey()` --calls--> `close()`  [INFERRED]
  src/lib/ui/Popover.svelte → src-tauri/src/broker.rs
- `onDown()` --calls--> `close()`  [INFERRED]
  src/lib/ui/Popover.svelte → src-tauri/src/broker.rs
- `register()` --calls--> `err()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/broker.rs
- `spawn()` --calls--> `register()`  [INFERRED]
  src-tauri/src/broker.rs → src-tauri/src/session.rs
- `create_session()` --calls--> `err()`  [INFERRED]
  src-tauri/src/session.rs → src-tauri/src/broker.rs

## Communities (87 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.05
Nodes (54): @xterm/addon-fit, svelte/elements, error, onKey(), otherAgent, send(), target, text (+46 more)

### Community 1 - "Community 1"
Cohesion: 0.08
Nodes (59): auth(), close(), close_target(), close_target_rules(), connection(), ct_eq(), err(), fixture() (+51 more)

### Community 2 - "Community 2"
Cohesion: 0.07
Nodes (49): home_dir(), collab_path(), run(), add_project(), add_to(), add_twice_same_id(), git(), git_ok() (+41 more)

### Community 3 - "Community 3"
Cohesion: 0.1
Nodes (28): login_shell(), claude_launch(), codex_hook_and_session_id(), codex_hook_overrides(), codex_launch(), codex_launch_flags(), codex_overrides(), codex_ready_scan() (+20 more)

### Community 4 - "Community 4"
Cohesion: 0.1
Nodes (26): PtyHandle, runs_command_and_streams_output(), scrubs_claude_code_markers_from_env(), spawn(), write_exit_ends_shell(), bad_json_errors(), load(), old_rows_default_name_and_args() (+18 more)

### Community 5 - "Community 5"
Cohesion: 0.1
Nodes (18): addProject(), addProjectOrOfferInit(), ClosedSession, CloseOutcome, closeTab(), early, initProject(), labelOf() (+10 more)

### Community 6 - "Community 6"
Cohesion: 0.13
Nodes (25): agentfleet (M1), agentfleet (M2 — inter-session messaging), agentfleet (M3.2 — remove project), agentfleet (M4 — native-feeling UI), agentfleet (M5 — spawn sessions from a session + Codex agent), Out of scope, Primary user story, Scope (+17 more)

### Community 7 - "Community 7"
Cohesion: 0.16
Nodes (20): call(), call_timeout(), Cmd, env(), hook_context_is_valid_json_with_guide(), hook_context_json(), install_skill(), main() (+12 more)

### Community 8 - "Community 8"
Cohesion: 0.21
Nodes (16): append(), append_in(), append_separator_only_when_non_empty(), dir(), emit_changed(), missing_is_empty(), parse_id(), read() (+8 more)

### Community 9 - "Community 9"
Cohesion: 0.17
Nodes (12): AppInfo, defaults(), invalid_rejected(), load(), round_trip(), save(), set_settings(), Settings (+4 more)

### Community 10 - "Community 10"
Cohesion: 0.14
Nodes (11): AgentKind, AppInfo, chain, DeepPartial, Settings, TerminalSettings, updateSettings(), a (+3 more)

### Community 11 - "Community 11"
Cohesion: 0.12
Nodes (15): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total., Blockers, Decisions, Next (+7 more)

### Community 12 - "Community 12"
Cohesion: 0.21
Nodes (11): apply_draft(), auto_delivery(), can_deliver(), cancel_run(), Decision, execute_run(), on_request_created(), on_status() (+3 more)

### Community 13 - "Community 13"
Cohesion: 0.19
Nodes (12): Button.svelte, Checks (run in order — cheapest first), Context, Dialog.svelte, Done when, Goal, Not included, Popover.svelte (+4 more)

### Community 14 - "Community 14"
Cohesion: 0.21
Nodes (11): Checks (run in order — cheapest first), Cleanup, code:ts (newSession: null as { anchor: HTMLElement; projectId: string), code:svelte (<Popover anchor={store.newSession?.anchor ?? null} open={sto), Context, Done when, Goal, NewSessionPopover.svelte (new) (+3 more)

### Community 15 - "Community 15"
Cohesion: 0.21
Nodes (11): Checks (run in order — cheapest first), code:ts (/** Deep-merge `patch` into a snapshot of the current settin), Context, Done when, `Field.svelte` (layout helper), Goal, Not included, SettingsPane.svelte (+3 more)

### Community 16 - "Community 16"
Cohesion: 0.23
Nodes (10): AgentIcon.svelte, App.svelte, Checks (run in order — cheapest first), Context, Done when, Goal, Messages.svelte, Scratchpad.svelte (substitution only), Not included (+2 more)

### Community 17 - "Community 17"
Cohesion: 0.18
Nodes (7): args, dry, edits, t, t2, ver, main()

### Community 18 - "Community 18"
Cohesion: 0.18
Nodes (4): collab, Run, RunStatus, store

### Community 19 - "Community 19"
Cohesion: 0.18
Nodes (10): Broker, Checks (run in order — cheapest first), CLI, code:rust (Spawn { agent: Option<String>, name: Option<String>, role: O), Context, Docs, Done when, Frontend (+2 more)

### Community 20 - "Community 20"
Cohesion: 0.18
Nodes (10): agentfleet, Close a session, Delegate work, Exit codes, Find sessions, Handle incoming work, Ids, Roles (+2 more)

### Community 21 - "Community 21"
Cohesion: 0.27
Nodes (8): applyTheme(), initTheme(), prefersDark(), ResolvedTheme, resolveTheme(), theme, ThemeSetting, onchange

### Community 22 - "Community 22"
Cohesion: 0.2
Nodes (9): Backend, Checks (run in order — cheapest first), code:sh (#!/bin/sh), Context, Done when, Frontend, Goal, Not included (+1 more)

### Community 23 - "Community 23"
Cohesion: 0.2
Nodes (9): Architecture, Feature inventory, How it was built, Integrations and network, Orchestra 0.3.1 Replication PRD, Overview, Replication requirements, UI and UX (+1 more)

### Community 24 - "Community 24"
Cohesion: 0.22
Nodes (4): @tauri-apps/api/core, Listener, listeners, Scratchpad

### Community 25 - "Community 25"
Cohesion: 0.22
Nodes (8): agentfleet, Features, Messaging between sessions, agentfleet, Messaging between sessions, agentfleet, Features, Messaging between sessions

### Community 26 - "Community 26"
Cohesion: 0.25
Nodes (7): Backend changes, Checks (run in order — cheapest first), Context, Done when, Frontend changes, Goal, Not included

### Community 27 - "Community 27"
Cohesion: 0.25
Nodes (7): agentfleet — M1 discovery brief, Done when I can, Explicitly out of M1, One job (M1 only), Product class, Stack constraints (fixed, not up for grill), Who

### Community 28 - "Community 28"
Cohesion: 0.25
Nodes (7): agentfleet (M1), Out of scope, Primary user story, Scope (M1), Stack constraints (fixed), Success demo, What / why

### Community 29 - "Community 29"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;), Context, Done when, Goal, Not included

### Community 30 - "Community 30"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:json ({"hooks":{), Context, Done when, Goal, Not included

### Community 31 - "Community 31"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, ), Context, Done when, Goal, Not included

### Community 32 - "Community 32"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:rust (#[derive(Serialize, Debug, PartialEq)]), Context, Done when, Goal, Not included

### Community 33 - "Community 33"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:ts (export async function removeProject(id: string): Promise<voi), Context, Done when, Goal, Not included

### Community 34 - "Community 34"
Cohesion: 0.39
Nodes (6): Checks (run in order — cheapest first), code:ts (test: { environment: "jsdom", include: ["src/**/*.test.ts"],), Context, Done when, Goal, Not included

### Community 35 - "Community 35"
Cohesion: 0.33
Nodes (3): startScheduler(), stopScheduler(), updater

### Community 36 - "Community 36"
Cohesion: 0.29
Nodes (6): Changes, Checks (run in order — cheapest first), Context (verified 2026-10-23 against codex-cli 0.163.0 by driving the TUI in a pty), Done when, Goal, Not included

### Community 37 - "Community 37"
Cohesion: 0.29
Nodes (6): Changes, Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 38 - "Community 38"
Cohesion: 0.29
Nodes (6): agentfleet roadmap, Later / optional, M2 — inter-session messaging, M3 — release, M4 — native-feeling UI (done), Shape fixed in M1 (do not rework)

### Community 39 - "Community 39"
Cohesion: 0.29
Nodes (6): code:sh (grep -rnE '#[0-9a-fA-F]{3,8}\b' src --include='*.svelte' --i), Primitives (`src/lib/ui/*.svelte`), Rule, Theming, Tokens (`src/lib/ui/tokens.css`), UI layer

### Community 40 - "Community 40"
Cohesion: 0.29
Nodes (6): agentfleet (M3.1 — terminal preferences + restore prompt), Out of scope, Primary user story, Scope, Success demo, What / why

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
Cohesion: 0.29
Nodes (6): agentfleet (M3.2 — remove project), Out of scope, Primary user story, Scope, Success demo, What / why

### Community 48 - "Community 48"
Cohesion: 0.29
Nodes (6): agentfleet (M3 — release), Out of scope, Primary user story, Scope (M3), Success demo, What / why

### Community 49 - "Community 49"
Cohesion: 0.29
Nodes (6): agentfleet (M4 — native-feeling UI), Out of scope, Primary user story, Scope, Success demo, What / why

### Community 50 - "Community 50"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 51 - "Community 51"
Cohesion: 0.48
Nodes (5): Checks, Context, Done when, Goal, Not included

### Community 52 - "Community 52"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 53 - "Community 53"
Cohesion: 0.29
Nodes (6): agentfleet (M2 — inter-session messaging), Out of scope, Primary user story, Scope (M2), Success demo, What / why

### Community 54 - "Community 54"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 55 - "Community 55"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 56 - "Community 56"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 57 - "Community 57"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 58 - "Community 58"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 59 - "Community 59"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 60 - "Community 60"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 61 - "Community 61"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 62 - "Community 62"
Cohesion: 0.48
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 63 - "Community 63"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 64 - "Community 64"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 65 - "Community 65"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 66 - "Community 66"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 67 - "Community 67"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 68 - "Community 68"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 69 - "Community 69"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 70 - "Community 70"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 71 - "Community 71"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 72 - "Community 72"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 73 - "Community 73"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 74 - "Community 74"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 75 - "Community 75"
Cohesion: 0.33
Nodes (5): Checks (run in order — cheapest first), Context, Done when, Goal, Not included

### Community 76 - "Community 76"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 77 - "Community 77"
Cohesion: 0.33
Nodes (5): Blockers, Decisions, Next, Now, STATE — rewrite after every ticket completes; never append. ≤30 lines total.

### Community 78 - "Community 78"
Cohesion: 0.4
Nodes (4): ADR 0001 — Hand-rolled UI primitives on native `<dialog>` and a positioned `<div>`, Consequences, Context, Decision

### Community 80 - "Community 80"
Cohesion: 0.5
Nodes (3): Release runbook, Secrets, Unnotarized builds

## Knowledge Gaps
- **297 isolated node(s):** `dry`, `ver`, `edits`, `t`, `t2` (+292 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `close()` connect `Community 1` to `Community 0`, `Community 2`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Why does `err()` connect `Community 1` to `Community 2`, `Community 3`, `Community 4`, `Community 7`, `Community 8`, `Community 9`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._
- **Are the 25 inferred relationships involving `err()` (e.g. with `.parse()` and `register()`) actually correct?**
  _`err()` has 25 INFERRED edges - model-reasoned connections that need verification._
- **Are the 13 inferred relationships involving `spawn_for()` (e.g. with `home_dir()` and `.from_settings()`) actually correct?**
  _`spawn_for()` has 13 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `handle_inner()` (e.g. with `lookup()` and `resolve_id()`) actually correct?**
  _`handle_inner()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 5 inferred relationships involving `persist()` (e.g. with `state_path()` and `register()`) actually correct?**
  _`persist()` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Are the 12 inferred relationships involving `home_dir()` (e.g. with `spawn_for()` and `close_one()`) actually correct?**
  _`home_dir()` has 12 INFERRED edges - model-reasoned connections that need verification._