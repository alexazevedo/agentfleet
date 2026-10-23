---
id: 38
status: done
model: haiku
depends_on: [37]
max_attempts: 4
files:
  - src-tauri/src/settings.rs
---
## Goal
A hand-edited `settings.json` with an unknown `theme` value loads as `system` instead of leaking an invalid string to the frontend (reviewer finding, the only one confirmed against the code).

## Context
`validate` runs only in `set_settings`; `load` deserialises any string. The frontend then sets `data-theme="purple"` and `theme.resolved` becomes a non-theme value, so the Settings theme control shows no selection.

Fix in `load`: after a successful parse, if `theme` is not one of `system | light | dark`, print `agentfleet: unknown theme "<v>" in settings.json, using system` to stderr and set it to `system`. Add a test `unknown_theme_falls_back` (write a file with `"theme":"purple"`, `load` returns `system`).

## Checks
1. `cd src-tauri && cargo test settings`

## Done when
- 51 Rust tests pass; the new test covers the fallback.

## Not included
- Frontend-side validation; normalising other fields on load.
