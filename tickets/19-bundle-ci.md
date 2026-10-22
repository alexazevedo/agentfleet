---
id: 19
status: doing
model: haiku
depends_on: []
max_attempts: 3
files:
  - src-tauri/tauri.conf.json
  - src-tauri/icons/
  - src-tauri/build.rs
  - .github/workflows/ci.yml
  - package.json
  - .gitignore
---
## Goal
`npm run tauri build` produces a launchable macOS app bundle locally, and a CI workflow runs the project's checks on every push and pull request.

## Context
Icons: generate a 1024×1024 source PNG with python3 (stdlib only: zlib + struct — a dark rounded square with an orange eight-ray asterisk is fine) at `src-tauri/icons/source.png`, then `npm run tauri icon src-tauri/icons/source.png` to produce the full set (32x32, 128x128, 128x128@2x, icon.icns, icon.ico, …) in `src-tauri/icons/`. Remove the old placeholder `icon.png` if the generator replaces it. `tauri.conf.json`: `bundle.active: true`, `bundle.targets: ["dmg", "app", "deb", "appimage"]`, `bundle.icon` listing the generated files, `bundle.macOS.minimumSystemVersion: "13.0"`, `bundle.createUpdaterArtifacts: true` (needed by ticket 23; harmless now), `bundle.category: "DeveloperTool"`. `build.rs`: before `tauri_build::build()`, run `git rev-parse --short HEAD` and emit `cargo:rustc-env=AGENTFLEET_COMMIT=<sha or "unknown">` plus `cargo:rerun-if-changed=../.git/HEAD` (About panel in ticket 20 reads `env!("AGENTFLEET_COMMIT")`).

`.github/workflows/ci.yml`: trigger `push` (branches: main) and `pull_request`; one `ubuntu-24.04` job: checkout, `actions/setup-node@v4` (node 22, `cache: npm`), `dtolnay/rust-toolchain@stable` with `components: clippy, rustfmt`, `Swatinem/rust-cache@v2` (`workspaces: src-tauri`), apt install `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libssl-dev`, `npm ci`, then the four check commands used by every ticket (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run check && npm run build`, all run from the right directories). Lint the YAML locally with `ruby -ryaml -e 'YAML.load_file(".github/workflows/ci.yml")'`. Add `src-tauri/icons/source.png` to git (keep), nothing else to ignore.

## Checks (run in order — cheapest first)
1. `ruby -ryaml -e 'YAML.load_file(".github/workflows/ci.yml")'`
2. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
3. `npm run check && npm run build`
4. `npm run tauri build -- --debug --bundles app && test -d src-tauri/target/debug/bundle/macos/agentfleet.app`

## Done when
- `open src-tauri/target/debug/bundle/macos/agentfleet.app` launches the app with the new icon in the Dock.
- The workflow file is valid YAML and lists the four checks (CI itself is exercised by the orchestrator after push).

## Not included
- Release workflow, signing, updater, notarization (tickets 21, 23).
