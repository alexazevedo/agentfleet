# Release runbook

1. Clean tree on `main`. Run `npm run release -- X.Y.Z` (bumps all version
   fields, refreshes Cargo.lock, commits `chore: release vX.Y.Z`, tags `vX.Y.Z`).
   Preview first with `npm run release -- X.Y.Z --dry-run`.
2. Push: `git push --follow-tags origin main`.
3. CI (ticket 23) builds and publishes the GitHub release.
4. Verify the release assets (dmg, updater `latest.json`, signatures).
5. Updater check: from the previous version, Settings > Check for updates.
