---
id: 24
status: todo
model: haiku
depends_on: [23]
max_attempts: 3
files:
  - docs/release.md
---
## Goal
Two real releases exist and the installed app updates itself from the first to the second.

## Context
Runbook ticket, executed by the orchestrator with the user confirming each outward step (push, tag, release); the implementer only verifies and documents. Steps: `gh secret set TAURI_SIGNING_PRIVATE_KEY < ~/.tauri/agentfleet.key` and the password secret; `npm run release -- 0.1.0`, `git push --follow-tags`; `gh run watch`; `gh release view v0.1.0 --json assets`; install the .dmg; `npm run release -- 0.1.1`, push; wait; in the installed 0.1.0 app press "Check now" → dialog → Install → relaunch shows 0.1.1 in About. Record outcomes and any gotchas in `docs/release.md` under "First release log".

## Checks (run in order — cheapest first)
1. `gh release view v0.1.0 --json assets -q '.assets[].name' | grep -E '\.dmg$|\.AppImage$|\.deb$|latest\.json|SHA256SUMS'`
2. `gh release view v0.1.1 --json assets -q '.assets[].name' | grep -q 'latest.json'`
3. `curl -sL https://github.com/alexazevedo/agentfleet/releases/latest/download/latest.json | grep -q '"version": *"0.1.1"'`

## Done when
- The installed 0.1.0 app updated itself to 0.1.1 (About shows 0.1.1).

## Not included
- Notarized builds, announcements, version 0.2 planning.
