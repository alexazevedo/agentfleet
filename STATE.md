# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
22-quit-guard-release-script — dispatched (sonnet)

## Next
23-release-workflow → 24-cut-releases

## Decisions
- execution: sequential
- M1/M2 archived under docs/forge-m1, docs/forge-m2; their decisions still apply
- release via tauri-apps/tauri-action (builds, updater json, GitHub release); SHA256SUMS in a follow-up job
- macOS: ad-hoc signature (`APPLE_SIGNING_IDENTITY=-`) unless APPLE_* secrets exist; pre-release flag on
- updater: minisign keypair at ~/.tauri/agentfleet.key (+ .password), never committed; secrets TAURI_SIGNING_PRIVATE_KEY(_PASSWORD) set on repo; pubkey in src-tauri/updater.pubkey + tauri.conf.json
- settings.json in ~/.agentfleet (camelCase: worktreesRoot, defaultAgent, autoPermission, autoDelivery, confirmQuit, updateChecks); SettingsState read-and-release, never held across sessions/projects/collab
- build commit sha via build.rs → AGENTFLEET_COMMIT (option_env!)
- GitHub owner = alexazevedo (gh account switched from plugu for repo ops)

## Blockers
- none
