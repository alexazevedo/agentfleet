> Reference spec, exported 2026-10-22 from the Claude Doc https://claude.ai/artifact/BZcd8f3GFcHPrLxWJqH3EU (rev 9). Two architecture diagrams did not export; see the doc. Requirements only; baton copies no Orchestra code.

# Orchestra 0.3.1 Replication PRD

Oct 21, 2026 · @Alex Azevedo

## Overview

Orchestra 0.3.1 is a macOS desktop cockpit for running many AI coding-agent sessions (Claude Code, Codex, OpenRouter-backed agents) side by side, each in its own git worktree, with embedded terminals, GitHub issue/PR context, scratchpads and inter-session messaging. Bundle id `dev.theorchestra.Orchestra`, signed by Leonardo Avelino (team R38NMT4LA7), 23 MB universal binary, macOS 13+.

**Who it is for:** solo developers and small teams who drive several agent CLIs at once across projects and want one place to launch, watch, diff and clean up those runs.

**End-to-end flow observed in the binary:**

1. Add a git repository as a project (emoji, profile, tracker such as GitHub).
2. Create a session: Orchestra makes a worktree under a configurable worktrees folder and spawns a login-shell PTY running the chosen agent CLI with a model and profile environment.
3. Watch output in an embedded xterm terminal, read the session diff against its base, send requests between sessions through an inbox, keep notes in a scratchpad.
4. Close the session: keep, discard or remove the worktree.
5. App self-updates from GitHub releases, verifies checksums, reports anonymous usage to PostHog only with consent.

**Key fact for replication:** the source is public at [github.com/theorchestradev/orchestra](https://github.com/theorchestradev/orchestra). The shipped build is commit `4ec732b`. This PRD is written from the shipped artifact first and cross-checked against that source so a clean-room rebuild and a fork are both possible.

## How it was built

Orchestra is a Wails v2 desktop app: a Go backend (CGO, Objective-C bridges) hosting a Vue 3 single-page frontend inside the system WebView, compiled to one universal Mach-O with the Vite bundle embedded via `go:embed`. Everything below was read from the shipped binary's Go build-info record and confirmed in the public repo.

| Layer | Choice | Version | Evidence |
| --- | --- | --- | --- |
| Language | Go, `CGO_ENABLED=1` | go1.26.3 | build-info in binary (REA `inspect_go_binary`, offset 9924368) |
| Desktop shell | Wails v2 (WKWebView on macOS, WebKitGTK 4.1 on Linux) | v2.16.0 | `go.mod`, `wails.json`, build tags `desktop,production` |
| Frontend | Vue 3 + Vite, plain JS, no router or state library | vue 3.5.44, vite 6.4.4 | `frontend/package.json`; bundle `assets/index-Cpd2TTX3.js` (764 KB) + `index-CC7EHtgQ.css` (66 KB) + lazy `preview-*.js` |
| Terminal | xterm.js + fit addon in the view; `creack/pty` PTY in Go | xterm 6.0.0, pty v1.1.24 | package.json, go.mod |
| Markdown | marked + DOMPurify | 18.1.1 / 3.4.16 | package.json |
| Native bridges | Objective-C `.m` files for window appearance, vibrancy, monospace font list, system palettes, app icons, Keychain | — | `internal/appearance/*.m`, `internal/apps/*.m`, `openrouter_key_darwin.go` |
| Telemetry | Hand-written PostHog HTTP client, no SDK | — | `internal/telemetry`, key and host injected by `-ldflags -X` |
| Icons | Lucide-derived SF-Symbol-named SVGs, agent brand SVGs | — | `frontend/public/symbols`, `frontend/public/agents` |
| Tests | `node --test` with jsdom for the frontend, `go test -race` for Go, three smoke scripts | — | `AGENTS.md`, `.github/workflows/linux.yml` |

**Build and release pipeline** (from `Makefile` and `.github/workflows/release.yml`):

1. Push a SemVer tag `v*`. GitHub Actions runs a `macos-15` job and an `ubuntu-24.04` job.
2. Both run `make build VERSION=<tag> RELEASE_TELEMETRY=1`, which calls `wails build -skipbindings -platform darwin/universal -ldflags "-X main.version=… -X main.buildCommit=… -X main.buildTelemetryKey=… -w -s"`. Linux adds `-tags webkit2_41`.
3. macOS job imports a Developer ID certificate into a temporary keychain, then `scripts/package-macos.sh` signs with hardened runtime, notarizes through an App Store Connect API key, staples, and builds the DMG. Without secrets it ships ad hoc signed and marks the release as pre-release.
4. The release job renames the DMG to the stable name `Orchestra-macos-universal.dmg`, writes `SHA256SUMS`, and publishes with `gh release create`.
5. Minimum macOS is 13.0, set both in `Info.plist` and `-mmacosx-version-min`. Bundle id `dev.theorchestra.Orchestra` must stay stable because the updater, website and macOS permissions key on it.

**Repo layout to mirror:** `cmd/orchestra/main.go` (the same binary doubles as the `orchestra` CLI), `app.go` (Wails bindings, 695 lines), `internal/core` (store, sessions, worktrees, broker, GitHub, updater, usage), `internal/appearance`, `internal/apps`, `internal/telemetry`, `frontend/src/{App.vue,components,shared}`, `skills/orchestra` (installable agent skill stub), `scripts/` (packaging and QA fixtures).

## Feature inventory

Every feature below was traced to a Go binding in `app.go` or a Store method visible in the stripped binary's symbol table (288 Orchestra symbols recovered), then confirmed in source. Nothing here is inferred from the UI alone.

| Area | Feature | Behavior that must be replicated | Evidence |
| --- | --- | --- | --- |
| Projects | Add, edit, remove, reorder, collapse | Folder picker, name, description, one-grapheme emoji; removing never deletes the folder; GitHub remote auto-detected from `origin` then `upstream` | `CreateProject`, `EditProject`, `MoveProject`, `DetectRepository` |
| Sessions | New session per agent | Agents: Claude Code, Codex, OpenCode, Pi, Mistral Vibe, Antigravity CLI, Grok Build, plain Terminal. Default name `<Agent> <n>`; optional initial prompt, model, effort, profile | `CreateSession`, `validateLaunch` |
| Sessions | Worktree isolation (on by default) | `git worktree add -b orchestra/<agent>-<id8> <worktrees>/<repo>/<id8> HEAD`; refuses dirty-less repo, existing branch, project folder outside repo; base commit recorded | `createSessionWorktree` |
| Sessions | Launch in login shell PTY | `$SHELL -l -i -c "export PATH=<runtime>:$PATH; export CLAUDE_CONFIG_DIR=…; cd <dir> && exec /usr/bin/env <agent cmd>"`, `TERM=xterm-256color`, size from the view | `StartSession` |
| Sessions | Agent auto mode and hooks | Claude: `--permission-mode auto --settings {hooks}` with hooks on SessionStart, UserPromptSubmit, PreToolUse, Stop, PermissionRequest writing `waiting`/`working` to a status file. Codex: `--approve-for-me` plus `-c hooks.*`. Pi: `--extension PiStatusExtension.js`. OpenCode: plugin injected via `OPENCODE_CONFIG_CONTENT`. Grok: `--permission-mode auto`. Antigravity: `agy --mode accept-edits` | `launchCommand`, `statusCommand` |
| Sessions | Resume vs new conversation | Agent session id captured (Claude/Pi/Grok use the Orchestra id; Codex and Antigravity parsed from output; OpenCode via plugin; Vibe via session log dir). Restart resumes with `--resume` / `resume` / `--session` / `--conversation` | `StartSession`, `captureLocked`, `readTerminal` |
| Sessions | Activity tracking | Status file polled every 200 ms: `unknown`, `working`, `waiting`, `ended`; Codex flagged `statusNeedsSetup` after 8 s without hooks | `monitorTerminal` |
| Sessions | Close and worktree cleanup | Close keeps worktree by default; remove only when `git status --porcelain` is clean; unmerged branch kept | `CloseSession`, `discardSessionWorktree` |
| Terminal | Embedded xterm.js | Base64 chunks over Wails events, 4 MB scrollback ring in Go, resize, bracketed-paste detection, draft-length tracking so auto-delivery never interleaves typing | `readTerminal`, `WriteTerminal`, `draftLength` |
| Terminal | Attention chime | Plays once per second max when any non-visible session goes working to waiting; Settings toggle | `attention.js`, `sounds/attention.wav` |
| Changes pane | Session diff | Diff of worktree against base commit (or project folder against HEAD), untracked files included, caps 1000 files / 2 MB / 40k lines | `SessionDiff`, `diff.go` |
| GitHub | Issues and PRs browser | Lists via `gh api repos/{repo}/issues?state=…` with pagination; detail plus comments with rendered HTML; refresh interval 0/30/60/300 s | `GitHubItems`, `GitHubDetail` |
| GitHub | Work on / Review | Starts a session from an issue (work\_on) or PR (review); session name clamped to 60 chars from the title | `CreateSession` with `GitHubItem` |
| GitHub | Metadata editor | Edit body (optimistic concurrency against current body), state with close reason incl. duplicate-of, assignees (max 10), labels (max 100); verifies the PATCH response actually applied | `UpdateGitHubMetadata` |
| Collaboration | Inbox requests between sessions | `session.request` queues a Run (queued, running, completed, cancelled, interrupted, needsAttention); same project only; terminals cannot receive; 256 KB request, 128 KB reply; history pruned at 500 runs / 30 days | `collaboration.go` |
| Collaboration | Execute Request / auto-delivery | User clicks Execute, or with the preference on, Store pastes the prompt into the recipient PTY when it is `waiting` with no draft; cycle detection on `run.wait` | `ExecuteRequest`, `deliverLocked`, `waitLocked` |
| Collaboration | Project Scratchpad | One Markdown note per project with revision counter; edit/read modes; CLI read/write/append with `--if-revision` optimistic locking; 256 KB cap | `SaveScratchpad`, `scratchpadLocked` |
| Collaboration | `orchestra` CLI | Same binary via `--orchestra-cli`; shim script on PATH; talks JSON over a 0600 Unix socket with a per-session token; `guide` prints the agent manual | `cli.go`, `main.go` |
| Agents | Orchestra Skill installer | Runs pinned `npx --yes skills@1.7.2 add <repo> --skill orchestra --global --agent …`, once globally and once per profile config folder for Claude | `skill.go` |
| Profiles | Multiple agent accounts | Profile = name, emoji, per-agent config folder under `~/.orchestra/profiles/<slug>/{claude,codex}` exported as `CLAUDE_CONFIG_DIR` / `CODEX_HOME`; Log In opens a terminal session running `claude auth login` or `codex login`; sign-in status via `claude auth status --json` / `codex login status` | `profiles.go` |
| Usage | Subscription limits | Spawns `codex app-server` or `claude --print --input-format stream-json …` and asks `account/rateLimits/read` / `get_usage`; cached 5 min, refresh after 15 s | `usage.go` |
| Usage | OpenRouter account | Management key stored in macOS Keychain (service `com.orchestra.openrouter`), credits/usage/activity read from `openrouter.ai/api/v1` | `openrouter_usage.go`, `openrouter_key_darwin.go` |
| Open in | Open folder in installed app | Detects 31 known editors, terminals, Git tools and Finder by bundle id or executable; remembers last choice; copy path | `apps.go` |
| Appearance | Themes | 30 built-in themes (system, catppuccin, dracula, gruvbox, nord, tokyo-night, …) each with light and dark 8-color palettes plus ANSI; custom themes in `~/.orchestra/themes/<name>/theme.json`; duplicate and reveal | `themes.json`, `themes.go` |
| Appearance | Mode, vibrancy, Arc mode | System/Light/Dark follows macOS; native translucent window with 40 to 95 percent opacity; Arc mode turns projects into Arc-browser-style spaces with a pager | `applyAppearance`, `ArcSpaces.vue` |
| Terminal prefs | Font and padding | Lists installed monospace fonts natively; size 10 to 24; padding 0 to 24 | `MonospaceFonts`, `SetPreferences` |
| Navigation | Command bar | Fuzzy palette (every word must match) with nested lists: go to, open in, new session, theme, appearance | `commands.js`, `CommandBar.vue` |
| Updates | Self-update | Checks GitHub releases 10 s after start and every 24 h (opt-out, beta channel, skip version); downloads zip via `gh` or public URL, verifies SHA256SUMS, codesign, same Team ID, notarization, version; stages beside the app and swaps on quit, relaunches | `update.go`, `update_install_darwin.go` |
| Privacy | Opt-in telemetry | First-launch banner; 8 event types with enumerated properties only; 256-event memory queue, 60 s flush, 20 events/min/name; `DO_NOT_TRACK` and CI disable | `docs/privacy.md`, `internal/telemetry` |
| State | Workspace persistence | `~/.orchestra/state.json` (atomic temp-and-rename, 0600), schema version 1 validated on load, unreadable file goes read-only; migration from the old `Application Support/Orchestra Rewrite` folder | `state.go`, `validation.go` |

## Architecture

A single Go process is the whole product: it serves the Vue view, owns every PTY, runs the message broker, and doubles as the `orchestra` CLI. There is no server component and no database.

&#91;embedded content: process and module layout · 1 Go process, 4 core modules, 4 outside parties\]

The agent process runs `orchestra` from the runtime folder placed first on its PATH; the shim re-executes the app binary with `--orchestra-cli`, which connects to the broker socket with the session's token. The view never talks to agents or GitHub directly.

**Persistent data model** (`state.json`, schema version 1):

| Entity | Fields that matter |
| --- | --- |
| State | `version`, `selectedProjectID`, `projects[]`, `profiles[]`, `preferences` |
| Project | `id`, `name`, `description`, `emoji`, `directory`, `isExpanded`, `selectedSessionID`, `showsGitHub`, `showsScratchpad`, `hasScratchpad`, `scratchpadName`, `scratchpadIndex`, `scratchpadText`, `scratchpadRevision`, `scratchpadMode`, `profileID`, `sessions[]` (`repository` is detected at runtime, never saved) |
| Session | `id`, `name`, `agent`, `agentSessionID`, `hasLaunched`, `initialPrompt`, `model`, `effort`, `worktreeDirectory`, `worktreeBranch`, `worktreeBase`, `gitHubItem{number,pullRequest}`, `profileID`, `loginAgent`; runtime-only: `activity`, `running`, `restored`, `exitMessage`, `statusNeedsSetup` |
| Profile | `id`, `name`, `emoji`, `folders{claude,codex}` |
| Preferences | telemetry trio, `arcMode`, `appearance`, `theme`, terminal font/size/smoothing/padding, `sidebarWidth`, `diffPaneWidth`, `diffPaneOpen`, `defaultAgent`, `startupView`, `confirmQuit`, `confirmSessionClose`, `gitHubRefreshInterval`, `deliverRequests`, `attentionSound`, `translucent`, `windowOpacity`, `updateChecksOff`, `betaUpdates`, `skippedUpdate`, `openInApp`, `hideOpenInButton` |
| Run (`collaboration.json`) | `id`, `projectID`, `fromSessionID`, `toSessionID`, `prompt`, `status`, `response`, `detail`, `createdAt` (seconds since 2001-01-01) |

**Disk layout:** `~/.orchestra/{state.json, collaboration.json, logs/, worktrees/<repo>/<id8>/, profiles/<slug>/{claude,codex}/, themes/<name>/theme.json, AgentSessions/Vibe/<id>/}`. A per-launch temp folder `orchestra-XXXX` (0700) holds `broker.sock`, the `orchestra` shim, `PiStatusExtension.js`, `OpenCodeSessionPlugin.js`, and per-session `<id>.status`, `<id>.agent`, `<id>.json` context files. `ORCHESTRA_STATE_FILE` relocates everything for QA.

**Wire protocol** (CLI to broker, one JSON object per line, 2 MB cap, 10 s deadline): `{method, source, token, target?, text?, requestID?, before?, revision?}` where method is one of `session.list`, `session.inbox`, `session.request`, `session.read`, `run.read`, `run.wait`, `run.wait.clear`, `run.reply`, `run.cancel`, `scratchpad.read`, `scratchpad.write`, `scratchpad.append`. Errors come back as `{error}`.

## Integrations and network

Orchestra has no backend of its own. Every outbound call goes through a CLI the user already installed or to one of three public HTTPS hosts.

| Integration | Mechanism | Auth | Notes |
| --- | --- | --- | --- |
| Agent CLIs (`claude`, `codex`, `pi`, `vibe`, `opencode`, `agy`, `grok`) | Spawned inside a login shell PTY; discovered on the shell's PATH | The CLI's own login; profiles switch `CLAUDE_CONFIG_DIR` / `CODEX_HOME` | Orchestra never reads credential files |
| Claude Code hooks | `--settings` JSON with command hooks on 5 events; `--session-id` / `--resume`; `--effort`; `--model` | — | Models hard-coded: fable, opus, sonnet, haiku; efforts low, medium, high, xhigh, max |
| Codex | `--approve-for-me`, `-c hooks.<Event>=[…]` TOML, `-m`, `-c model_reasoning_effort`; `codex debug models` lists models; `codex app-server` JSON-RPC for `account/rateLimits/read` | Codex login | Session id parsed from `Session ID:` in output |
| Pi | `--session-id`, `--extension` (status + reply forwarder), `--thinking`; `pi --list-models` | — | Extension replies to inbox runs automatically |
| OpenCode | Plugin injected through `OPENCODE_CONFIG_CONTENT.plugin[]`, `--session`, `--prompt`; `opencode models` | — |  |
| Mistral Vibe | `VIBE_SESSION_LOGGING` JSON to capture session id | — |  |
| Antigravity | `agy --mode accept-edits --conversation --prompt-interactive`; `agy models` | — |  |
| Grok Build | `grok --session-id --permission-mode auto`; `grok models` | — |  |
| GitHub | `gh api --hostname github.com` (issues, pulls, comments, assignees, labels, PATCH with `X-GitHub-Api-Version: 2026-03-10`), `gh release download`; `GH_PROMPT_DISABLED=1` | `gh auth login` | Falls back to unauthenticated public API only for release checks and downloads |
| GitHub Releases (updater) | `repos/theorchestradev/orchestra/releases?per_page=30`; asset `Orchestra-<v>-macos-universal.zip` + `SHA256SUMS` | none | Verifies SHA-256, `codesign --verify --deep --strict`, Team ID equals running app, `spctl --assess`, version in Info.plist; uses `ditto` to unpack; relaunch via `open -n` after PID exit |
| PostHog | Custom HTTP batch client to `https://us.i.posthog.com` (EU host default for non-release builds) | project token injected at link time | `$process_person_profile:false`, `$lib:orchestra`; disabled by `DO_NOT_TRACK`, `ORCHESTRA_TELEMETRY_DISABLED`, CI |
| OpenRouter | `GET https://openrouter.ai/api/v1/credits` and activity endpoints | Management key in macOS Keychain | Only totals cross the bridge |
| Skills CLI | `npx --yes skills@1.7.2 add https://github.com/theorchestradev/orchestra --skill orchestra --global --agent <key>` | none | Needs Node.js |
| Open in apps | macOS Launch Services by bundle id (`NSWorkspace`), or executable with cwd args on Linux | — | WezTerm runs its bundle executable with `start --cwd` |
| Native macOS | Objective-C: `NSAppearance`, `NSVisualEffectView` vibrancy, `NSFontManager` monospace list, system accent palettes, `NSWorkspace` icons, `Security.framework` Keychain, `UniformTypeIdentifiers` | — | Each has a `_other.go` stub for Linux |

**Security properties to keep:** broker socket and context files are 0600 in a 0700 temp dir; tokens are two UUIDs compared in constant time; requests from a closed session are rejected; the view cannot start agents or inject input except through Execute Request; agents never get full-bypass permission flags; content from other sessions is labeled as data in the guide.

## UI and UX

One window, 1200 by 800 default, opens maximized with a hidden-inset title bar on macOS. Layout is sidebar (220 to 400 px), main pane, optional Changes pane (280 to 900 px), status bar.

**Screens and panes**

- **Sidebar:** projects as collapsible groups with emoji, each listing sessions plus optional Scratchpad and Issues & PRs rows; drag to reorder; activity dot per session (working, waiting, ended); context menus with Shift+F10. Arc mode replaces it with vertical space icons and a project pager.
- **Session view:** xterm.js terminal with the agent's brand icon, Open in button, model and branch in the header; restored sessions show a Resume / Start New Conversation choice; exit message when the process ends.
- **Changes pane:** file list with additions and deletions, unified diff per file, truncation notices.
- **Issues & PRs:** list with open/closed/all filter and auto-refresh; detail with rendered Markdown and comments; inline editors for description, state and close reason, assignees, labels; Work on / Review buttons that create a session.
- **Scratchpad:** Markdown editor with Edit/Read toggle (Cmd+E), rename, delete, revision conflict error.
- **Messages:** per-session inbox history; compose request to another session (Cmd+Enter), Execute, Cancel; status chips queued, running, completed, cancelled, interrupted, needs attention.
- **Command bar (Cmd+K):** grouped palette with nested pickers.
- **Settings (Cmd+,):** rail with General, Agents, Appearance, Terminal, Shortcuts, Privacy, Profiles. General holds default agent, startup view, confirmations, GitHub refresh, update checks and beta, About. Agents holds auto-delivery, attention sound, Orchestra Skill install, usage. Appearance holds mode, theme with duplicate and reveal, translucency and opacity, Arc mode. Terminal holds font, size, smoothing, padding, hide Open in.
- **Dialogs:** New Project (folder, name, description, emoji, profile), New Session (agent, name, prompt, worktree toggle, model, effort, profile), Rename, Remove project, Close session with Remove worktree checkbox, Quit confirmation while sessions run, Update available, first-launch usage consent banner.

**Keyboard shortcuts** (macOS form; Ctrl replaces Cmd elsewhere, with Shift added inside terminals where the key is taken)

| Shortcut | Action |
| --- | --- |
| Cmd+K | Command bar |
| Cmd+N | New session |
| Shift+Cmd+N | New project |
| Cmd+, | Settings |
| Shift+Cmd+\[ / \] | Previous / next item |
| Cmd+B | Toggle sidebar |
| Shift+Cmd+B | Toggle Changes pane |
| Cmd+C / Cmd+V | Copy / paste in terminal |
| Cmd+E | Scratchpad edit/read |
| Cmd+\[ | Back to issue list |
| Cmd+Enter | Save description / send request |
| Esc | Close dialog, cancel edit |

**Design rules from `AGENTS.md`:** every surface uses the 8 semantic tokens `canvas`, `sidebar`, `surface`, `text`, `muted`, `rule`, `accent`, `selection`; agent brand colors preserved with light and dark SVG variants; full keyboard navigation and accessible names; a `?preview=1` mode runs the whole view against a fake backend (`preview.js`) for visual QA without Wails.

## Replication requirements

Two ways to replicate: fork the MIT source and rebrand (days), or rebuild clean-room from this PRD in any stack (weeks). The requirements below are stack-neutral; the roadmap assumes the same Wails + Vue stack because it is the cheapest proven path.

**Functional requirements**

| ID | Requirement | Priority |
| --- | --- | --- |
| FR-1 | Projects: add from folder, edit name/description/emoji, reorder, remove without touching the folder, detect GitHub remote | Must |
| FR-2 | Sessions for 7 agents plus plain terminal, grouped by project, named `<Agent> <n>` by default | Must |
| FR-3 | Launch each agent through the user's login shell with the exact flags in the Integrations table, in auto-approval mode, never full bypass | Must |
| FR-4 | Optional git worktree per session on branch `orchestra/<agent>-<id8>`; refuse when not a repo, no commit, branch exists, or folder outside repo | Must |
| FR-5 | Embedded terminal with resize, copy/paste, 4 MB scrollback, bracketed-paste awareness | Must |
| FR-6 | Activity state per session from agent hooks (working / waiting), with a fallback hint when hooks are missing | Must |
| FR-7 | Resume the same agent conversation after app restart; explicit Start New Conversation | Must |
| FR-8 | Close session keeps worktree unless user chooses remove, and only when clean; unmerged branch survives | Must |
| FR-9 | `orchestra` CLI reachable from any session with session list, request, inbox, read, run read/wait/reply/cancel, scratchpad read/write/append | Must |
| FR-10 | Inbox runs with the 7 states, same-project scoping, size caps, history pruning, interrupted on close | Must |
| FR-11 | Execute Request pastes the request prompt into the recipient terminal; optional automatic delivery when the agent is idle with no draft | Must |
| FR-12 | Project Scratchpad with revision-based optimistic locking shared by UI and CLI | Must |
| FR-13 | Changes pane: diff of worktree vs base, untracked files, caps, binary flag | Should |
| FR-14 | GitHub issues and PRs browser via `gh`, Work on / Review session creation, metadata editing with server-side verification | Should |
| FR-15 | Profiles with per-agent config folders, Log In sessions, sign-in status, project default profile | Should |
| FR-16 | Usage: Claude and Codex rate-limit windows; OpenRouter balance with Keychain-stored key | Could |
| FR-17 | Orchestra Skill installer through the pinned skills CLI, per profile | Should |
| FR-18 | Open in: detect installed editors, terminals, Git tools; remember last | Could |
| FR-19 | Themes: 30 built-in, custom theme folders, system/light/dark, vibrancy, Arc mode | Could |
| FR-20 | Command bar, full shortcut set, Settings with the 7 sections | Should |
| FR-21 | Self-update from GitHub releases with checksum, signature, Team ID, notarization and version checks, staged swap on quit | Should |
| FR-22 | Opt-in telemetry limited to the 8 enumerated events, with environment kill switches | Could |

**Non-functional requirements**

- Single binary under 25 MB, universal macOS 13+ and Linux x86\_64; start under 2 s.
- All state in `~/.orchestra`, written atomically with 0600, schema-versioned, never corrupting on crash.
- No credentials ever cross the view bridge or get read from disk by the app.
- Store operations serialized under one mutex; PTY reads never block the UI.
- Tests: Go race tests for store, worktree, broker, diff, updater; jsdom tests for every shared JS module; smoke scripts per agent plugin.

&#91;embedded content: replication roadmap · 7 phases, 6 gates\]

Each phase ends when its gate passes on a real machine with real agent CLIs. Phase 3 is the differentiator; phases 4 to 7 can be reordered by taste.

## Unknowns and risks

With the source public, nothing material is unrecoverable. What remains open is about reuse, not understanding.

- **License and branding.** MIT allows forking and selling, but the name Orchestra, the icon, the bundle id `dev.theorchestra.Orchestra`, the `theorchestra.dev` domain and the PostHog token are the author's. A replica needs its own bundle id, update repository, signing identity and telemetry project; the updater refuses builds signed by a different Team ID.
- **Agent CLI drift.** Launch flags, hook event names, `codex app-server` and `claude --print` control protocols, and the Pi and OpenCode plugin APIs are undocumented or fast-moving. The build pins nothing on the agent side; expect breakage with every agent release and keep the smoke scripts.
- **Hook-based status is best effort.** Vibe and OpenCode have no status hooks, so their sessions never show working/waiting; Codex needs its hooks config to exist. The attention chime and auto-delivery depend on this signal.
- **Not reverse-engineered at runtime.** This PRD is static evidence (binary metadata, symbols, embedded assets) plus source reading. No session was launched and no network traffic was captured, so timings, exact PostHog payloads on the wire and agent output parsing were not observed live.
- **Binary-only findings that the source explains.** The shipped build was made on a GitHub Actions `macos-15` runner (`/Users/runner/work/orchestra/orchestra`), Go 1.26.3, from commit `4ec732b` tagged `v0.3.1`, with the US PostHog host and a live project token embedded through `-ldflags`. The x86\_64 slice was not inspected separately; both slices come from one `wails build -platform darwin/universal`.
- **Website and skill repo.** The marketing site and the `/docs/privacy` page live in a separate `theorchestradev/site` repository that reads theme and style files from this one; it was not examined.

**Sources:** [Orchestra repository at the shipped commit](https://github.com/theorchestradev/orchestra/tree/4ec732b), [release workflow](https://github.com/theorchestradev/orchestra/blob/4ec732b/.github/workflows/release.yml), [privacy and event inventory](https://github.com/theorchestradev/orchestra/blob/4ec732b/docs/privacy.md), [Wails v2](https://wails.io/docs/reference/runtime/intro). Binary evidence: REA `inspect_go_binary` evidence id `ev_39d78ed2…`, SHA-256 of the arm64 slice `5b5441e5…dd83e5`.
