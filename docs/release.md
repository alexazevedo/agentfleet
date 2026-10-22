# Release runbook

1. Clean tree on `main`. Run `npm run release -- X.Y.Z` (bumps all version
   fields, refreshes Cargo.lock, commits `chore: release vX.Y.Z`, tags `vX.Y.Z`).
   Preview first with `npm run release -- X.Y.Z --dry-run`.
2. Push: `git push --follow-tags origin main`.
3. CI (ticket 23) builds and publishes the GitHub release.
4. Verify the release assets (dmg, updater `latest.json`, signatures).
5. Updater check: from the previous version, Settings > Check for updates.

## Secrets

Required repository secrets:

- `TAURI_SIGNING_PRIVATE_KEY` - updater signing key.
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - password for that key.

Optional (enable Apple Developer ID signing and notarization; when
`APPLE_CERTIFICATE` is absent the macOS build is ad-hoc signed):

- `APPLE_CERTIFICATE` - base64 .p12 certificate.
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_SIGNING_IDENTITY`
- `APPLE_ID`
- `APPLE_PASSWORD` - app-specific password.
- `APPLE_TEAM_ID`

## Unnotarized builds

Until a Developer ID is configured, macOS builds are ad-hoc signed and not
notarized, and releases are marked pre-release. On first launch, right-click
the app and choose Open (then confirm) to get past Gatekeeper.
