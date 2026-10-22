# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
M3.2 28–29 done and approved (mvp 2 of 2, 49 Rust tests, svelte-check 0 errors, build ok); committed on worktree-remove-project

## Next
manual probe in `npm run tauri dev` (remove project with clean + dirty session) → merge to main after M3.1 ticket 27 lands

## Decisions
- execution: sequential (28 then 29)
- work in worktree `.claude/worktrees/remove-project`, branch `worktree-remove-project`, based on main d6a92c2
- M3.1 artifacts archived to docs/forge-m3.1/ (ticket 27 still uncommitted on main; resolve at merge)
- cascade in backend: `remove_project_cascade(id, app)` = `close_one` per session in id8 order (clean worktree removed, dirty kept) then `project::remove_from`; sessions first, project last; stop at first error; unknown id → "no such project"
- `close_session` now takes only `id` + AppHandle (frontend unchanged)
- `project::remove_project` guard kept unchanged as last-line safety
- frontend: `store.removeProjectId` opens `RemoveProjectDialog`; `removeProject()` splices tabs + project on success, store untouched on error; Escape/Cancel disabled while busy
- confirmation always shown; errors render inside the dialog (no toast channel exists)

## Blockers
- none
