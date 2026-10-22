---
id: 16
status: done
model: sonnet
depends_on: [15]
max_attempts: 3
files:
  - src-tauri/src/deliver.rs
  - src-tauri/src/session.rs
---
## Goal
Auto-delivery fires immediately for an idle recipient: draft tracking ignores terminal control/reply sequences, and clearing a draft re-triggers delivery.

## Context
Smoke-test finding: a request to an idle Claude session stays queued. Cause: `write_session` counts every byte as draft, but xterm also sends replies Claude Code asks for — focus events `ESC[I` / `ESC[O` on each tab switch, cursor-position reports `ESC[<r>;<c>R`, device attributes, arrow keys `ESC[A`. One focus change poisons `draft_len` forever, so `can_deliver` never sees `draft_len == 0`.

Fix in `deliver.rs`: `pub fn apply_draft(draft: usize, bytes: &[u8]) -> usize`, a per-chunk parser (shortcut: sequences split across writes are not handled; note in a comment): `ESC [` … skip through the final byte in `0x40..=0x7E` (CSI, includes `ESC[200~`/`ESC[201~` bracketed-paste markers — the pasted text between them still counts); `ESC O x` (SS3) skip 1; `ESC` + any other byte skip 1; `\r` or `\n` → 0; `0x7f`/`0x08` → saturating −1; other bytes `< 0x20` ignored; UTF-8 continuation bytes `0x80..=0xBF` ignored; everything else +1. Tests: `b"\x1b[I"` → unchanged; `b"\x1b[12;40R"` → unchanged; `b"\x1b[A"` → unchanged; `b"abc"` → 3; `"é".as_bytes()` → 1; `b"ab\x7f"` → 1; `b"abc\r"` → 0; `b"\x1b[200~hi\x1b[201~"` → 2.

`session.rs`: `write_session` uses `apply_draft`; if `draft_len` was `> 0` and becomes `0` without a `\r`/`\n` in the chunk (user erased the draft) and `s.status == Status::Waiting`, drop the lock after the PTY write and call `deliver::on_status(&app, id, Status::Waiting)` (add `app: AppHandle` to the command signature; frontend args unchanged).

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test`
3. `npm run check && npm run build`

## Done when
- With B idle, `agentfleet send <B> "..."` from A appears in B within a second even after switching tabs several times before sending.

## Not included
- Cross-chunk escape parsing, frontend draft reporting, delivery toggle.
