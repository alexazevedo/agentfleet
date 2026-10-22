---
mode: mvp                      # prototype | mvp | production
routing_profile: cloud         # cloud | cloud+local (local is opt-in, never a default)
parallel_default: sequential   # sequential | parallel
owner: human                   # human | agent (set by forge:auto)
---

# agentfleet (M3 — release)

## What / why
M1 and M2 (docs/forge-m1, docs/forge-m2) work only from `npm run tauri dev` on this machine. M3 makes agentfleet installable and self-updating: a public GitHub repo, a tag-driven CI release that ships a macOS .dmg and Linux AppImage/.deb, an in-app updater, a Settings panel for the few knobs the first two milestones hardcoded, and a quit guard. No Apple Developer ID exists yet, so macOS builds are ad-hoc signed; the updater's integrity rests on Tauri's own signing key, and the notarization step is wired but gated on secrets. Shape rules in docs/roadmap.md still apply: settings are a pane in the main area, state stays in `~/.agentfleet/`.

## Primary user story
As a solo developer, I download agentfleet from a GitHub release, run it, and get told when a new version exists and have it install itself, so that I stop running the app from a dev checkout and can hand the link to someone else.

## Scope (M3)
- Repo: public `github.com/alexazevedo/agentfleet`, `main` pushed, releases from tags `v*`.
- CI (GitHub Actions): on tag push, `macos-14` job builds a universal .dmg (ad-hoc signature; notarize/staple steps run only when `APPLE_*` secrets exist), `ubuntu-24.04` job builds AppImage + .deb; both upload artifacts; a release job writes `SHA256SUMS`, generates the Tauri updater `latest.json`, and publishes a GitHub release (pre-release when unnotarized). A PR/push workflow runs fmt, clippy, cargo test, svelte-check, vite build.
- Updater: `tauri-plugin-updater` + `tauri-plugin-process`, minisign keypair (public key in `tauri.conf.json`, private key as a CI secret), endpoint = the repo's `latest.json`. Checks 10 s after start and every 24 h unless disabled; "Update available" dialog with Install / Later; install then relaunch.
- Settings pane (sidebar Settings row → main pane): worktrees root, default agent, Claude auto-permission on/off, auto-delivery on/off, confirm quit on/off, update checks on/off + "Check now", About (version, commit). Persisted in `~/.agentfleet/settings.json`; M1/M2 code reads these instead of hardcoded values.
- Quit guard: closing the window with running sessions asks to confirm (when enabled); confirmed quit kills PTYs and removes the runtime dir.
- Version bump script (`npm run release -- <semver>`) updates `package.json`, `tauri.conf.json`, `Cargo.toml`, commits and tags.

## Out of scope
- Apple notarization with a real certificate (wired, not exercised); Windows; Homebrew/winget
- Telemetry, crash reporting, beta channel, skip-version
- Themes, appearance, terminal font settings, shortcuts, profiles, GitHub integration
- Attention chime, Changes pane

## Success demo
Done when I can push tag `v0.1.0`, watch CI publish a GitHub release with .dmg, AppImage, .deb, SHA256SUMS and latest.json, install the .dmg, change the worktrees root and default agent in Settings and see new sessions honour them, push `v0.1.1`, and have the installed app show "Update available", install it and relaunch as 0.1.1.
