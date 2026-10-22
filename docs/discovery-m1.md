# agentfleet — M1 discovery brief

Context: portfolio project. Own architecture, own frontend. Reference spec: "Orchestra 0.3.1 Replication PRD" (Claude Doc, https://claude.ai/artifact/BZcd8f3GFcHPrLxWJqH3EU). Use it for requirements only; no code reuse.

## Product class
mvp (core-logic tests, documented shortcuts). Owner answers grill questions.

## Who
Solo developer running several AI coding agents in parallel on one repo, who loses track of which terminal is doing what and which branch each one is on.

## One job (M1 only)
Open a tab -> agentfleet creates a git worktree on a fresh branch -> launches Claude Code in a real PTY inside that worktree -> terminal renders live in the app -> closing the tab removes a clean worktree.

## Explicitly out of M1
- Inter-session messaging, inbox, CLI, broker (M2)
- Updater, signing, CI release (M3)
- Agents other than Claude Code and plain shell
- Themes, profiles, GitHub integration, usage/cost views, settings UI beyond what M1 needs
- Windows (keep platform-specific code behind a `platform/` module; macOS + Linux targets only)

## Done when I can
1. `npm run tauri dev` opens window with empty tab bar.
2. Click "+", pick repo folder, pick "Claude Code" -> new tab, branch `agentfleet/claude-<id>` worktree under `~/.agentfleet/worktrees/<repo>/<id>`, Claude Code prompt appears in xterm within 2 s.
3. Second tab same repo -> second worktree, both terminals independent, resize works.
4. Status badge per tab flips working/waiting from Claude hooks status file.
5. Close tab with no uncommitted changes -> worktree and branch removed. Dirty worktree -> tab closes, worktree kept, badge says "kept".
6. Relaunch app -> tabs restored from `~/.agentfleet/state.json` (worktrees reattached, PTY relaunched).

## Stack constraints (fixed, not up for grill)
Tauri 2, Rust backend (`portable-pty`, tokio, serde), frontend Svelte or Solid + @xterm/xterm (not Vue). PTY output over Tauri `Channel<Vec<u8>>`, not `emit` per chunk.
