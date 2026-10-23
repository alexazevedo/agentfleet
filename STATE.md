# STATE

## Now
M6 (worktree opt-out, session name in popover, live model in sidebar; tickets 35–39) merged via PR #1, and pi + opencode agent kinds (enum, launch, status, resume, skill install, UI) merged on top. Both probed live before merge; merge conflicts (cli.rs, session.rs, SKILL.md) resolved by hand: pi/opencode get `ModelSource::None`.

## Next
Probe in `npm run tauri dev`: New session → Pi / opencode open their TUIs, sidebar dot cycles across one prompt, restore resumes the same conversation; `agentfleet spawn --agent pi|opencode "reply with pong"` end to end; popover name + no-worktree → session in repo root; model appears in sidebar for Claude (statusLine) and Codex (payload `model`, may stay blank); M4 probe (theme, settings tabs, popover) still pending.

## Decisions
- No-worktree session = Worktree{path: repo root, branch: current HEAD}, owns_worktree=false; close returns Removed without touching files; old state.json rows default owns_worktree=true
- Model channel: Claude = per-session settings statusLine command → ~/.agentfleet/run/<id>.model; Codex = `model` key from captured hook payload, blank if absent; pi/opencode = none
- pi (0.84.2): status via `-e <run>/<id>.pi-ext.js` extension (session_start/agent_end→waiting, agent_start→working); `--session-id <uuid>` creates-or-loads, so restore = same command; guide via `--append-system-prompt <run>/<id>.guide.md`; pi has no permission prompts → Permission ignored
- opencode (1.18.18): status via plugin at `<run>/<id>.opencode-plugin.js` loaded through `env OPENCODE_CONFIG_CONTENT` (plugin entries need `file://`); plugin tracks root session, writes `{"session_id":…}` to `<id>.opencode-session.json` → resume `--session <id>`; guide via config `instructions`; Auto→`--auto`, Bypass→`--auto` + permission all-allow overlay
- `codex_session_id` renamed `payload_session_id` (same JSON shape for codex hook + opencode plugin)
- 6 s waiting-fallback timer covers codex, pi (trust prompt precedes session_start) and opencode (plugin loads once a session exists); OSC 7501 ready scan stays codex-only
- skill install adds `~/.pi/agent/skills` when `~/.pi` exists; opencode discovers `~/.claude/skills` itself
- probe caveat: a user MCP that never exits (e.g. npx rea-agents) keeps `opencode run` alive past the watchdog; `OPENCODE_PROBE_MODEL` / `PI_PROBE_ARGS` override probe models
- M5 artifacts archived to docs/forge-m5/; M4 decisions in docs/forge-m4/STATE.md

## Blockers
none
