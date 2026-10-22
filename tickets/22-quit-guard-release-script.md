---
id: 22
status: doing
model: sonnet
depends_on: [20]
max_attempts: 3
files:
  - src-tauri/src/lib.rs
  - src-tauri/src/session.rs
  - scripts/release.mjs
  - package.json
  - docs/release.md
---
## Goal
Closing the window with running sessions asks for confirmation (when enabled) and quits cleanly; `npm run release -- <semver>` bumps every version field, commits and tags.

## Context
Quit guard in `lib.rs`: `.on_window_event(|window, event| if let WindowEvent::CloseRequested { api, .. } = event { ... })`: read `SettingsState.confirm_quit` and count sessions with a PTY; if both, `api.prevent_close()` and ask via `tauri_plugin_dialog::DialogExt` `window.dialog().message("N sessions are running. Quit anyway?").kind(Warning).buttons(OkCancelCustom("Quit","Cancel")).show(move |yes| if yes { shutdown(&app) })` where `pub fn shutdown(app)` in `session.rs` kills every PTY (take the handles out of the map first, drop the lock, then kill), sends poller stop flags, then `app.exit(0)` (the existing `RunEvent::Exit` handler removes the runtime dir). When the guard does not trigger, call `shutdown` too so quitting always kills children.

`scripts/release.mjs` (node, no deps): arg `<semver>`; refuses a dirty tree or a non-semver arg; rewrites `version` in `package.json`, `package-lock.json` (both top-level and `packages[""]`), `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` (`[package] version`), runs `cargo update -p agentfleet --offline` inside `src-tauri` to refresh `Cargo.lock`, commits `chore: release v<semver>`, tags `v<semver>`, prints the push command but does not push. `package.json` script `"release": "node scripts/release.mjs"`. `docs/release.md`: 10-line runbook (bump → push tag → CI → verify assets → updater). Test the script on a throwaway branch: run it, verify the four files, then `git tag -d` and `git reset --hard HEAD~1` back (leave the tree exactly as found).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `node scripts/release.mjs 1.2.3 --dry-run && git status --porcelain | wc -l | grep -q "^ *0$"`  (implement `--dry-run`: print the planned changes, touch nothing)
4. `npm run check && npm run build`

## Done when
- With two sessions running, closing the window shows the confirmation; Cancel keeps the app; Quit kills both agents (no `claude` processes left) and removes the temp runtime dir.
- `npm run release -- 0.1.0` on a clean tree produces one commit and tag `v0.1.0` with all version fields updated.

## Not included
- Changelog generation, pushing, GitHub release creation (CI does that).
