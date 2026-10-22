---
id: 01
status: done
model: haiku
depends_on: []
max_attempts: 3
files:
  - package.json
  - package-lock.json
  - vite.config.ts
  - svelte.config.js
  - tsconfig.json
  - tsconfig.node.json
  - index.html
  - src/main.ts
  - src/App.svelte
  - src/lib/TabBar.svelte
  - src-tauri/Cargo.toml
  - src-tauri/Cargo.lock
  - src-tauri/build.rs
  - src-tauri/tauri.conf.json
  - src-tauri/capabilities/default.json
  - src-tauri/icons/icon.png
  - src-tauri/src/main.rs
  - src-tauri/src/lib.rs
  - src-tauri/src/platform/mod.rs
  - src-tauri/src/platform/unix.rs
  - src-tauri/src/worktree.rs
  - src-tauri/src/pty.rs
  - src-tauri/src/session.rs
  - .gitignore
---
## Goal
Tauri 2 + Svelte 5 + TypeScript app that opens a 1200x800 window titled "agentfleet" with an empty tab bar and a "+" button, plus the Rust module skeleton later tickets fill in.

## Context
Scaffold with `npm create tauri-app@latest` (template: svelte, TypeScript, npm) or hand-write the equivalent. App identifier `dev.agentfleet.app`. Frontend: Vite + Svelte 5 (runes). Rust crate name `agentfleet`, edition 2021, deps: `tauri = "2"`, `tauri-build = "2"`, `serde`/`serde_json`, `tokio` (features `full`), `portable-pty = "0.8"`, `uuid` (v4), `dirs = "5"`, `thiserror`, `anyhow`.

`lib.rs` declares `mod platform; mod worktree; mod pty; mod session;` and `pub fn run()` building the Tauri app. `worktree.rs`, `pty.rs`, `session.rs` contain only `//! filled by ticket NN`. `platform/mod.rs` re-exports `unix.rs` under `#[cfg(unix)]`, which provides `pub fn home_dir() -> PathBuf` (via `dirs`) and `pub fn login_shell() -> String` (`$SHELL`, fallback `/bin/sh`). Add `#[cfg(not(unix))] compile_error!("agentfleet M1 targets macOS and Linux only")`.

`TabBar.svelte` renders a horizontal bar with zero tabs and a "+" button (no-op). `App.svelte` mounts it above an empty main area. Add npm scripts: `check` (`svelte-check --tsconfig ./tsconfig.json`), `build`, `tauri`. Add `.gitignore` entries for `node_modules`, `dist`, `src-tauri/target`. `tauri::generate_context!()` needs an icon: add a placeholder `src-tauri/icons/icon.png` (any valid PNG, e.g. 128x128 generated via `cargo tauri icon` or a tiny hand-made PNG) and point `bundle.icon` in `tauri.conf.json` at it.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `npm run check`
3. `cd src-tauri && cargo test`
4. `npm run build`

## Done when
- `npm run tauri dev` opens a window with an empty tab bar and a "+" button.
- `cargo test` passes with the platform module's `login_shell()` unit test (non-empty string).

## Not included
- Any worktree, PTY, dialog, or xterm code.
- Tauri plugins (dialog added in ticket 04).
