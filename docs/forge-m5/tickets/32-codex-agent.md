---
id: 32
status: done
model: opus
depends_on: [31]
max_attempts: 3
files:
  - src-tauri/src/agent.rs
  - src-tauri/src/session.rs
  - src-tauri/src/settings.rs
  - src-tauri/src/collab.rs
  - src-tauri/src/deliver.rs
  - src-tauri/src/cli.rs
  - src/lib/sessions.svelte.ts
  - src/lib/NewTabDialog.svelte
  - src/lib/SettingsPane.svelte
  - src/lib/Messages.svelte
  - scripts/probe-codex-hooks.sh
  - skills/agentfleet/SKILL.md
---
## Goal
Codex CLI is a third agent kind: selectable in New session and as `default_agent`, launched with status hooks so its sidebar dot works, resumable on restart, a valid target for `send`/`spawn`, and `agentfleet spawn --agent codex …` works end to end.

## Context
`Agent` enum in `src-tauri/src/session.rs` is `Claude | Shell` (`as_str`/`parse`). `spawn_for` builds the PTY command: for Claude it writes `~/.agentfleet/run/<id>.settings.json` via `agent::write_hooks_settings`, starts `status::spawn_poller` on `<id>.status`, and launches `agent::claude_launch(id, &settings, resume, auto_permission, &agent_args)`. `close_one` deletes `<id>.status` and `<id>.settings.json`. `collab::request` rejects targets whose agent `!= Agent::Claude` ("target is not a claude session"); `deliver::can_deliver` likewise. `settings::validate` accepts `default_agent` in `{claude, shell}`. Frontend: `AgentKind = "claude" | "shell"`, radios in `NewTabDialog.svelte`, `<select>` in `SettingsPane.svelte`, `Messages.svelte:12` filters recipients by `agent === "claude"`. `AgentIcon.svelte` already renders `"codex"`.

Codex facts (codex-cli 0.163.0, verified locally with `codex --help` and the hooks docs):
- Hooks are stable and on by default. Event names match Claude's: `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PermissionRequest`, `Stop`, `Interrupt`. Each command hook gets one JSON object on stdin with `session_id`, `cwd`, `hook_event_name`, …
- Hooks can be supplied inline as config, and `-c key=value` overrides take TOML values, so `-c 'hooks.Stop=[{hooks=[{type="command",command="<path> waiting"}]}]'` adds a Stop hook for this invocation. Hooks need trust; `--dangerously-bypass-hook-trust` runs enabled hooks without persisted trust for the invocation. (This machine's `~/.codex/config.toml` already carries `hooks.state."/<session-flags>/config.toml:stop:0:0"` entries from another tool doing exactly this.)
- `--approve-for-me` = automatic approval review (Codex's analogue of Claude's `--permission-mode auto`). `-a never` exists but is not used here.
- No way to choose the session id at launch; resume is `codex [OPTIONS] resume <SESSION_ID>`, global options go before the subcommand.

### Backend
1. `Agent::Codex` (`"codex"`). `parse`, `as_str`, settings `validate` (`claude | codex | shell`; update the `invalid_rejected` test message if it changes), `settings.rs` `defaults` test unchanged.
2. `agent.rs`:
   - `pub fn write_codex_hook(status_file: &Path, payload_file: &Path, dest: &Path) -> io::Result<()>` writes an executable (0755) `sh` script:
     ```sh
     #!/bin/sh
     # $1 = status word; stdin = Codex hook payload (JSON with session_id).
     cat > '<payload_file>'
     printf '%s' "$1" > '<status_file>'
     ```
     (paths via `shell_quote`). Hooks must print nothing on stdout.
   - `pub fn codex_hook_overrides(script: &Path) -> Vec<String>`: six `-c` values, one per event, mapping `SessionStart`→`waiting`, `UserPromptSubmit`→`working`, `PreToolUse`→`working`, `PermissionRequest`→`waiting`, `Stop`→`waiting`, `Interrupt`→`waiting`, each exactly `hooks.<Event>=[{hooks=[{type="command",command="<script> <word>"}]}]`. The script path must not contain `"` or `\` — return an error string from the caller if it does (home paths; keep simple).
   - `pub fn codex_launch(script: &Path, resume_id: Option<&str>, auto_permission: bool, extra: &[String]) -> String`: `codex --dangerously-bypass-hook-trust` + (`--approve-for-me` if auto) + the six overrides each as `-c <shell_quoted>` + extra args shell-quoted + (`resume <id>` when `resume_id` is `Some`).
   - `pub fn codex_session_id(payload_file: &Path) -> Option<String>`: parse the JSON, return `session_id` if it is a non-empty string.
   - Tests: overrides contain all six events with the right words; launch string with/without `--approve-for-me`, with `resume`, with extra args quoted; `codex_session_id` on a sample payload and on garbage.
3. `session.rs` `spawn_for`: generalize the Claude branch to `agent != Shell`. For Codex: `script = run.join(format!("{id}.codex-hook.sh"))`, `payload = run.join(format!("{id}.codex-hook.json"))`, write the hook script, `resume_id = if resume { agent::codex_session_id(&payload) } else { None }` (no id → fresh launch, never an error), command = `codex_launch(...)`, start the poller exactly like Claude. `close_one`: also delete `codex-hook.sh` and `codex-hook.json`. Worktree branch naming already uses `agent.as_str()` (`agentfleet/codex-<id8>`).
4. `collab::request` and `deliver::can_deliver`: accept `Agent::Claude | Agent::Codex`, reject only `Shell`; messages become "target is not an agent session". Update the two existing tests (`request_rules`, `decisions`) and add a Codex-target positive case to each. `cli.rs`: the "agent args need …" message and USAGE wording ("claude or codex"); no parsing change.

### Frontend
- `AgentKind = "claude" | "codex" | "shell"`; `labelOf("codex") === "Codex"`.
- `NewTabDialog.svelte`: third radio "Codex" between Claude Code and Shell. `SettingsPane.svelte`: `<option value="codex">Codex</option>`. `Messages.svelte:12`: `t.agent !== "shell"`.

### Probe script
`scripts/probe-codex-hooks.sh` (executable): creates a temp dir, writes the same hook script shape by hand (status + payload files), runs `codex exec --skip-git-repo-check --dangerously-bypass-hook-trust -c 'hooks.SessionStart=[{hooks=[{type="command",command="<tmp>/hook.sh waiting"}]}]' -c 'hooks.Stop=[{hooks=[{type="command",command="<tmp>/hook.sh waiting"}]}]' 'Reply with the single word pong.'` with a 180 s `timeout`, then asserts the status file reads `waiting` and the payload file contains `"session_id"`. Exit 0 on success, 1 with the files' contents on failure, 2 if `codex` is not on PATH (check `codex exec --help` for the exact flag names first; if `--skip-git-repo-check` does not exist, run inside the temp dir after `git init`). It is the runtime check for the hook contract; it uses the user's Codex login and makes one model call.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`
5. `scripts/probe-codex-hooks.sh`

## Done when
- All five checks pass; new tests cover overrides, launch string, session-id parse, Codex as a valid request/delivery target.
- In `npm run tauri dev`: New session → Codex opens a Codex TUI in its own worktree, the dot goes waiting → working → waiting across one prompt; quitting and restoring the session resumes the same Codex conversation; `agentfleet spawn --agent codex "reply with pong"` from a Claude session works like the Claude case in ticket 31. Record the probe in STATE.md.

## Not included
- Codex profiles/accounts (`CODEX_HOME`), model/effort pickers, usage views; Codex-specific settings beyond `default_agent`; `-a never` / sandbox selection (users pass those via `--agent-arg`); Windows.
