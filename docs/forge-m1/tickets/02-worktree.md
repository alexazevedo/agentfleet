---
id: 02
status: done
model: sonnet
depends_on: [01]
max_attempts: 3
files:
  - src-tauri/src/worktree.rs
---
## Goal
Rust module that creates and removes git worktrees on `agentfleet/<agent>-<id8>` branches, enforcing the refusal rules, with unit tests against temporary repos.

## Context
Pure module, no Tauri types. Shell out to `git` via `std::process::Command`. API:

```rust
pub struct Worktree { pub repo: PathBuf, pub path: PathBuf, pub branch: String, pub base_commit: String }
pub enum WorktreeError { NotARepo, NoCommits, BranchExists(String), Git(String), Io(std::io::Error) }
pub fn create(repo: &Path, agent: &str, id8: &str, root: &Path) -> Result<Worktree, WorktreeError>
pub fn is_clean(path: &Path) -> Result<bool, WorktreeError>   // `git status --porcelain` empty
pub fn remove(wt: &Worktree) -> Result<(), WorktreeError>     // `git worktree remove` then `git branch -D`
pub fn exists(wt: &Worktree) -> bool                          // path is a dir containing `.git`
```

`create`: `git rev-parse --show-toplevel` fails → `NotARepo`; `git rev-parse HEAD` fails → `NoCommits`; `git show-ref --verify refs/heads/<branch>` succeeds → `BranchExists`. Branch `agentfleet/<agent>-<id8>`, path `<root>/<repo-basename>/<id8>`, run `git worktree add -b <branch> <path> HEAD` from `repo`. Record `base_commit` from `git rev-parse HEAD`. `remove` must succeed on a clean tree and must not delete the worktree if it is dirty (caller checks `is_clean`; `remove` itself uses `git worktree remove` without `--force`).

Tests build throwaway repos in `tempfile::tempdir()` (add `tempfile` as dev-dependency) with `git init`, `git -c user.email=t@t -c user.name=t commit --allow-empty -m init`.

## Checks (run in order — cheapest first)
1. `cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings`
2. `cd src-tauri && cargo test worktree`

## Done when
- Tests cover: create succeeds and path/branch exist; NotARepo; NoCommits; BranchExists; is_clean true then false after writing a file; remove on clean tree deletes path and branch.

## Not included
- Tauri commands or state (ticket 04).
- Any "kept" semantics beyond `remove` refusing dirty trees (ticket 06).
