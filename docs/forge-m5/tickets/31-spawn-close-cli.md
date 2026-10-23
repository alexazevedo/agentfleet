---
id: 31
status: done
model: sonnet
depends_on: [30]
max_attempts: 3
files:
  - src-tauri/src/broker.rs
  - src-tauri/src/cli.rs
  - src-tauri/src/session.rs
  - src/lib/sessions.svelte.ts
  - src/lib/Sidebar.svelte
  - skills/agentfleet/SKILL.md
  - README.md
---
## Goal
From inside a session, `agentfleet spawn [--agent claude|shell] [--name <label>] [--role <name>] [--agent-arg <arg>]... <prompt...|->` creates a sibling session in the same project, a tab for it appears in the app, the prompt is queued as a run from the caller to the new session and auto-delivered once it reports `waiting`, and `agentfleet close <session>` tears a sibling down. The skill doc teaches both.

## Context
Broker: `src-tauri/src/broker.rs` is a JSON-lines Unix-socket server. `Request` is the deserialized line (`method`, `source`, `token`, optional `target`, `text`, `requestId`, `revision`, …). `serve()` builds a `dispatch` closure per request: `run.wait` is handled specially (async), everything else goes through `handle()` → `handle_inner()` which authenticates (source + constant-time token) while holding the sessions lock, then dispatches by method. After `handle`, `serve` emits `collab::changed` for touched projects and, for `session.request`, calls `deliver::on_request_created`. `session::resolve_id` resolves full id or unique id8 prefix; `session::lookup` gives `(project, agent)`.

CLI: `src-tauri/src/cli.rs` parses argv into `Cmd`, `run()` calls the broker via `call(method, extra)`; `env()` reads `AGENTFLEET_SOCKET/SESSION/TOKEN`. `USAGE` is the help text; `GUIDE` is `include_str!("../../skills/agentfleet/SKILL.md")`. Sessions already delegate via `send` (`session.request`) and `wait` (`run.wait`).

Ticket 30 added `session::register(app, project_id, agent, name, agent_args) -> SessionInfo` (registry entry + worktree, no PTY) and `attach_session(.., resume)`; the frontend attaches a tab with `tab.info` set and `resume: false` for a fresh start.

### Broker
Add to `Request`: `#[serde(default)] pub agent: Option<String>`, `name: Option<String>`, `agent_args: Option<Vec<String>>` (camelCase → `agentArgs`).

Two new methods, handled in `serve`'s `dispatch` before `handle` (they need the `AppHandle`; `handle_inner` stays app-free):

- `session.spawn {agent?, name?, text?, agentArgs?}`: `auth()` → source project. Parse agent with `session::Agent::parse` (make it `pub`); missing agent → settings `default_agent`. Run `session::register(...)` inside `tokio::task::spawn_blocking` (git worktree creation must not block the runtime; clone the `AppHandle` into it). On success: emit `app.emit("session-spawned", &info)` so the UI creates the tab, then, if `agent != Shell` and `text` is `Some`, create the run exactly as `session.request` does (`collab::request(&mut c, source, new_id, text, now, lookup)`, `collab::save`, `collab::changed`, `deliver::on_request_created`). If `agent == Shell` and `text` is `Some` → `{"error":"shell sessions take no prompt"}` before registering. Reply `{"session": info, "run": run | null}`. On `register` error reply `{"error": msg}`. Put the logic in `pub async fn spawn(req, app) -> Value` next to `wait`.
- `session.close {target}`: `auth()` → resolve `target` with `resolve_id` among sessions of the source's project only (`"no such session in this project"` otherwise); target == source → `{"error":"cannot close yourself"}`; else `session::close_one(&app, id)` (sync, run it in `spawn_blocking` too: it joins the PTY reader) and `app.emit("session-closed", json!({"id": id}))`. Reply `{"outcome": CloseOutcome}` (`{"kind":"removed"}` / `{"kind":"kept","path":..}`).

Add broker tests (pure, like `list_filters_to_source_project`): a small `pub fn close_target(map, source, target) -> Result<Uuid, String>` helper that encodes "same project, not self, resolvable" and is unit-tested for: other project → error, self → `"cannot close yourself"`, ok → id. Use the same helper from `session.close`.

### CLI
New `Cmd` variants:
```rust
Spawn { agent: Option<String>, name: Option<String>, role: Option<String>, agent_args: Vec<String>, text: Option<String> }, // text None = read stdin, Some("") never
Close { session: String },
```
Parsing for `spawn`: options may appear in any order before the prompt words; `--agent X`, `--name X`, `--role X` take one value, `--agent-arg X` repeats; a lone `-` means stdin; the remaining words joined by spaces are the prompt. `--agent shell` with any prompt → `Err("shell sessions take no prompt")`; any other agent with no prompt and no `-` → `Err("spawn needs a prompt or -")`. Unknown `--flag` → error. `close` takes exactly one argument.

`run()` for `Spawn`: resolve stdin if needed; if `role` is set read `~/.agentfleet/roles/<role>.md` (`crate::platform::home_dir()`), missing → `Err(format!("no such role: {path}"))`, else `text = format!("{}\n\n{}", file.trim_end(), text)`. Call `session.spawn` with `agent`, `name`, `text`, `agentArgs` (omit `None`s). Print `"{id8} {run-id}"` (run id empty when no run). For `Close`: resolve like `send` does (via `session.list`), call `session.close`, print `kept <path>` on `kind == "kept"`, nothing on `removed`. Add `pub fn with_role(role_text: Option<&str>, prompt: &str) -> String` and test it (prepend + blank line; `None` → prompt unchanged). Extend `parses_verbs` with spawn/close cases incl. the two error cases and repeated `--agent-arg`.

Update `USAGE`: `spawn`, `close`, and `send <session>` wording "send a request to an agent session".

### Frontend
- `src/lib/sessions.svelte.ts`: `initSpawnListener()` (guarded like `initStatusListener`): `listen<SessionInfo>("session-spawned")` → push `{ key, projectId: info.projectId, agent: info.agent, info, status: early.get(info.id) ?? "unknown", resume: false }`, set `store.activeKey ??= key` (do not steal focus); `listen<{id:string}>("session-closed")` → `remove(key)` of the tab whose `info.id` matches (no `close_session` call: the backend already closed it). Call `initSpawnListener()` in `Sidebar.svelte`'s `onMount` next to the other two.
- Terminal attaches it via `attach_session(resume: false)` (ticket 30 behaviour); delivery then happens on the first `waiting` status.

### Docs
`skills/agentfleet/SKILL.md`: add sections **Spawn a session** (`agentfleet spawn --agent claude --role reviewer "Review branch X against main"` → prints `<id8> <run>`; follow with `agentfleet wait <run>`), **Close a session** (`agentfleet close <id8>`, same project only, never yourself), **Roles** (`~/.agentfleet/roles/<name>.md`, prepended to the prompt; example `reviewer.md` of 3–4 lines: review only, report findings as a list, do not edit files), and `--agent-arg` (verbatim extra CLI args for the agent, repeatable). Keep the data-not-instructions rule prominent. README "Messaging between sessions" paragraph: add `spawn` and `close` to the verb list.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`
5. `cd src-tauri && cargo run -- --cli help 2>&1 | grep -q 'spawn \[--agent' && cargo run -- --cli guide | grep -q '^## Spawn'`

## Done when
- All checks pass; new tests: `with_role`, `close_target` rules, spawn/close parsing.
- Manual probe (not a gate, record in STATE.md): in `npm run tauri dev`, from a Claude session run `agentfleet spawn --name reviewer "reply with the word pong"`; a second tab appears under the same project titled "reviewer", its dot goes waiting → working → waiting, `agentfleet wait <run>` prints `pong`, `agentfleet close <id8>` removes the tab.

## Not included
- Codex (ticket 32); cross-project spawn/close; role UI or shipped roles; spawning from the New-session dialog; closing yourself.
