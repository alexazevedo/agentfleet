# STATE — rewrite after every ticket completes; never append. ≤30 lines total.

## Now
idle — M2 approved and committed

## Next
M3 per docs/roadmap.md (/forge:idea after archiving M2 under docs/forge-m2/)

## Decisions
- execution: sequential
- M1 archived under docs/forge-m1/; M1 decisions still apply (Svelte 5, Channel Raw bytes, UUID ids, state.json v1)
- CLI = same binary, hidden `--cli` first arg in main.rs; shim script on PATH re-execs current_exe; `guide` works without session env
- PTY `-c` payload `cd <cwd> && <cmd>`; Shell tabs `export PATH=<shim>:$PATH; exec "$SHELL" -i`
- broker: tokio UnixListener in temp_dir()/agentfleet-<pid> (0700), socket 0600, JSON lines, per-session token (2×UUID), ct_eq; dispatch returns boxed futures (run.wait awaits Notify outside the mutex, 25 s budget)
- lock order: sessions → projects → collab
- draft tracking backend-side in write_session; delivery = bracketed paste + CR; status::emit is the single status sink → deliver::on_status
- collaboration.json v1; scratchpads/<projectId>.json {revision,text}, process-wide write lock
- frontend store file scratchpadStore.svelte.ts (case-insensitive clash with Scratchpad.svelte)
- shortcuts: runtime dir removed only on clean quit (SIGTERM leaves it); auto-delivery hardcoded on; no re-delivery on failure

## Blockers
- none
