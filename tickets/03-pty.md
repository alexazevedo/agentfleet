---
id: 03
status: done
model: sonnet
depends_on: [01]
max_attempts: 3
files:
  - src-tauri/src/pty.rs
---
## Goal
Rust PTY module that spawns a command in a login shell inside a directory, streams raw output bytes to a callback, accepts input, resizes, and kills, with a unit test.

## Context
Pure module, no Tauri types. Use `portable-pty`. API:

```rust
pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;
pub struct PtyHandle { /* master, writer, child, reader thread join handle */ }
pub fn spawn(shell: &str, cwd: &Path, command: Option<&str>, env: &[(String, String)], cols: u16, rows: u16, on_data: OnData, on_exit: Box<dyn Fn(i32) + Send + 'static>) -> anyhow::Result<PtyHandle>
impl PtyHandle {
    pub fn write(&self, bytes: &[u8]) -> anyhow::Result<()>;
    pub fn resize(&self, cols: u16, rows: u16) -> anyhow::Result<()>;
    pub fn kill(&mut self) -> anyhow::Result<()>;
}
```

`command == None` → run `<shell> -l`. `command == Some(cmd)` → run `<shell> -l -i -c "cd <cwd> && exec /usr/bin/env <cmd>"` (shell-quote `cwd`). Always set `TERM=xterm-256color` plus the given `env`, and set the PTY `cwd`. A dedicated `std::thread` reads the master in 8 KiB chunks and calls `on_data`; on EOF it waits the child and calls `on_exit(code)`. `kill` sends kill to the child and drops the master so the reader thread ends.

Test: `spawn("/bin/sh", tempdir, Some("echo PTYOK"), &[], 80, 24, …)` collecting bytes into an `Arc<Mutex<Vec<u8>>>`; wait for `on_exit` (channel with 5 s timeout); assert output contains `PTYOK`. Second test: `write(b"exit\n")` to a `None` command shell and assert `on_exit` fires.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test pty`

## Done when
- Both tests pass on macOS and Linux.
- No busy loop: reader thread blocks on read, exits on EOF.

## Not included
- Tauri channel plumbing (ticket 04).
- Scrollback buffering, bracketed-paste detection, draft tracking.
