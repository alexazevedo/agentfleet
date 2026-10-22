---
id: 10
status: done
model: opus
depends_on: []
max_attempts: 3
files:
  - src-tauri/src/main.rs
  - src-tauri/src/lib.rs
  - src-tauri/src/cli.rs
  - src-tauri/src/broker.rs
  - src-tauri/src/runtime.rs
  - src-tauri/src/session.rs
  - src-tauri/src/pty.rs
  - src-tauri/Cargo.toml
---
## Goal
`agentfleet list`, run from inside any session's terminal, prints the sessions of that session's project by talking to an authenticated Unix-socket broker inside the app; the same binary is the CLI.

## Context
`main.rs`: if `std::env::args().nth(1) == Some("--cli")` call `agentfleet_lib::cli::main(args[2..])` and `std::process::exit(code)`; otherwise `run()`. Nothing Tauri-related may initialise on the CLI path.

`runtime.rs`: `pub struct Runtime { pub dir: PathBuf, pub socket: PathBuf, pub shim: PathBuf }`, created once at startup: `dir = std::env::temp_dir().join(format!("agentfleet-{pid}"))` with mode 0700; `shim = dir/agentfleet` = `#!/bin/sh\nexec '<current_exe>' --cli "$@"\n`, mode 0755; `socket = dir/broker.sock`. Managed as `tauri::State<Runtime>`. On `tauri::RunEvent::Exit` remove the dir (best effort).

`broker.rs`: started from `lib.rs` `.setup()` via `tauri::async_runtime::spawn(broker::serve(app_handle))`. `tokio::net::UnixListener::bind(socket)`, chmod 0600. Protocol: one JSON object per line, one response line, 2 MB cap, 10 s deadline. Request `{ "method": String, "source": Uuid, "token": String, "target"?: String, "text"?: String, "requestId"?: String, "before"?: String, "revision"?: u64 }`; response is a JSON object, errors as `{ "error": String }`. Auth: look up `source` in `Sessions`, compare `token` to `Session.token` with a constant-time byte compare (equal length + XOR-fold), reject unknown or closed sessions with `{"error":"unauthorized"}`. This ticket implements only `session.list` → `{ "sessions": [ { id, id8, agent, status } ] }` filtered to the source's project. Put the dispatch in `pub async fn handle(app, req) -> serde_json::Value` so later tickets add methods; also include a tokio test that binds a socket in a tempdir, inserts a fake session+token via the real `Sessions` type, and asserts `session.list` round-trips and a bad token is rejected (construct `tauri::test::mock_app()` with `tauri = { features = ["test"] }` in dev-dependencies, or factor `handle` to take `&Sessions, &Projects` directly so no AppHandle is needed in tests — prefer the latter).

`session.rs`: `Session.token: String` (two UUID v4 simple strings concatenated, new per spawn); `Session.status: Status` (updated by `status::emit` callers — add a `set_status` helper here; ticket 12 uses it) — minimal: store `Status::Unknown` on spawn and leave updates to ticket 12. `spawn_for` builds the PTY `env` = `[("AGENTFLEET_SOCKET", socket), ("AGENTFLEET_SESSION", id), ("AGENTFLEET_TOKEN", token)]` and a command that first does `export PATH=<shim dir quoted>:"$PATH";` then `exec claude …` (Claude) or `exec "$SHELL" -i` (Shell). `pty.rs`: the `-c` payload becomes `cd <cwd> && <cmd>` (drop `exec /usr/bin/env`); `command == None` path unchanged; existing tests keep passing (`echo PTYOK`).

`cli.rs`: `pub fn main(args: Vec<String>) -> i32`. Reads `AGENTFLEET_SOCKET/SESSION/TOKEN` from env (missing → print "agentfleet: not running inside an agentfleet session" to stderr, return 2). Uses `std::os::unix::net::UnixStream` (sync, no tokio on the CLI path), writes one JSON line, reads one line, 10 s read timeout. Verbs this ticket: `list` (table: `id8  agent  status  id`), `--help`/no args (usage, exit 2). Unknown verb → usage, exit 2. Add `pub fn parse(args) -> Result<Cmd, String>` with a unit test.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `cd src-tauri && cargo build -q && ./target/debug/agentfleet --cli list; test $? -eq 2`
4. `npm run check && npm run build`

## Done when
- In a running Shell or Claude session, `agentfleet list` prints that project's sessions; from a plain terminal outside the app it exits 2 with the "not running inside" message and opens no window.
- The broker socket and runtime dir are 0600/0700 (`ls -la` on the temp dir).

## Not included
- Any run/scratchpad methods or verbs (tickets 11, 14); delivery (12); UI (13).
