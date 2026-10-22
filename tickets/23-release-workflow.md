---
id: 23
status: doing
model: sonnet
depends_on: [19, 21, 22]
max_attempts: 3
files:
  - .github/workflows/release.yml
  - docs/release.md
---
## Goal
Pushing a `v*` tag builds a universal ad-hoc-signed macOS .dmg and Linux .deb + AppImage, generates signed updater artifacts and `latest.json`, writes `SHA256SUMS`, and publishes a GitHub pre-release.

## Context
`.github/workflows/release.yml`, trigger `push: tags: ['v*']`, `permissions: contents: write`. Job `build`, matrix: `{ platform: macos-14, args: "--target universal-apple-darwin" }`, `{ platform: ubuntu-24.04, args: "" }`. Steps: checkout; setup-node 22 with npm cache; `dtolnay/rust-toolchain@stable` with `targets: aarch64-apple-darwin,x86_64-apple-darwin` on macOS; `Swatinem/rust-cache@v2` (`workspaces: src-tauri`); ubuntu apt deps as in ci.yml; `npm ci`; a step `Enable Apple signing when secrets exist` that, only `if: matrix.platform == 'macos-14' && env.HAVE_APPLE == 'true'` (set `HAVE_APPLE: ${{ secrets.APPLE_CERTIFICATE != '' }}` at job env level), appends `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` from secrets to `$GITHUB_ENV`; otherwise a step that sets `APPLE_SIGNING_IDENTITY=-` (ad-hoc). Then `tauri-apps/tauri-action@v0` with `env: GITHUB_TOKEN, TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}, TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}` and `with: tagName: ${{ github.ref_name }}, releaseName: "agentfleet ${{ github.ref_name }}", releaseBody: "See docs/release.md", releaseDraft: false, prerelease: true, includeUpdaterJson: true, args: ${{ matrix.args }}`. Job `checksums` (`needs: build`, ubuntu): `gh release download ${{ github.ref_name }} -D assets`, `sha256sum assets/* > SHA256SUMS`, `gh release upload ${{ github.ref_name }} SHA256SUMS --clobber`. Lint with `ruby -ryaml`. Update `docs/release.md` with the secrets list (`TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`, optional `APPLE_*`) and the pre-release note (unnotarized builds: right-click → Open on first launch).

## Checks (run in order — cheapest first)
1. `ruby -ryaml -e 'YAML.load_file(".github/workflows/release.yml")'`
2. `grep -q "tauri-apps/tauri-action@v0" .github/workflows/release.yml && grep -q "includeUpdaterJson: true" .github/workflows/release.yml && grep -q "SHA256SUMS" .github/workflows/release.yml`
3. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`

## Done when
- (Exercised by ticket 24) tag push → release with `.dmg`, `.AppImage`, `.deb`, `latest.json`, `*.sig`, `SHA256SUMS`.

## Not included
- Homebrew tap, winget, Windows job, changelog, notarization verification.
