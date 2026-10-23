# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
pi and opencode added as agent kinds (enum, launch, status, resume, skill install, UI); contracts probed live: pi ext waiting→working→waiting with "pong" (ollama gemma4:12b-mlx), opencode plugin waiting→working→waiting(idle) + session id payload with "pong" (nemotron-3.5-lightning-free)

## Next
probe in `npm run tauri dev`: New session → Pi / opencode open their TUIs, sidebar dot cycles across one prompt, restore resumes the same conversation; `agentfleet spawn --agent pi|opencode "reply with pong"` end to end; M4 probe (theme, settings tabs, popover) still pending

## Decisions
- pi (0.84.2): status via `-e <run>/<id>.pi-ext.js` extension (session_start/agent_end→waiting, agent_start→working); `--session-id <uuid>` creates-or-loads, so restore = same command, no resume flag; guide via `--append-system-prompt <run>/<id>.guide.md` (file contents); pi has no permission prompts → Permission ignored
- opencode (1.18.18): status via plugin at `<run>/<id>.opencode-plugin.js` loaded through `env OPENCODE_CONFIG_CONTENT` (merges on top of user config; plugin entries need `file://`); plugin tracks root session (no parentID), writes `{"session_id":…}` to `<id>.opencode-session.json` → resume `--session <id>`; guide via config `instructions`; Auto→`--auto`, Bypass→`--auto` + permission all-allow overlay
- `codex_session_id` renamed `payload_session_id` (same JSON shape for codex hook + opencode plugin)
- 6 s waiting-fallback timer now covers codex, pi (trust prompt precedes session_start) and opencode (plugin loads only once a session exists); OSC 7501 ready scan stays codex-only
- skill install adds `~/.pi/agent/skills` when `~/.pi` exists; opencode discovers `~/.claude/skills` itself (no new target)
- probe caveat: a user-configured MCP that never exits (e.g. npx rea-agents) keeps `opencode run` alive past the watchdog; TUI unaffected; `OPENCODE_PROBE_MODEL` / `PI_PROBE_ARGS` override probe models
- M5/Codex decisions: see git history of this file (3815a92 and earlier)
- M4 decisions: see docs/forge-m4/STATE.md

## Blockers
- none
