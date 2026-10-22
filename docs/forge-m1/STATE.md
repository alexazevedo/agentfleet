# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
idle — 9/9 tickets done; user smoke-testing sidebar build before approval 2 of 2 (uncommitted)

## Next
(none) — after approval: commit, then M2 per docs/roadmap.md

## Decisions
- execution: sequential
- frontend: Svelte 5 + TypeScript
- PTY bytes over `Channel<InvokeResponseBody>` with `Raw(chunk)` (tauri 2.12.5)
- tab id = UUID v4; id8 = first 8 hex for branch/folder
- claude launch: `claude --permission-mode auto --settings <file> --session-id <uuid>` | `--resume <uuid>`
- state.json v1 `{version, projects, sessions}` — schema mutable until first release (docs/roadmap.md)
- scope change 2026-10-22: sidebar + Project entity pulled into M1 (tickets 08–09); roadmap re-based
- lock order in Rust: sessions then projects
- shortcuts: placeholder icon.png; poller cancel via watch flag only; `--resume` failure shows "ended"; old state.json without `projects` → treated as empty

## Blockers
- none
