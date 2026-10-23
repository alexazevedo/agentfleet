//! Git worktree management. Pure module, no Tauri types.

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Worktree {
    pub repo: PathBuf,
    pub path: PathBuf,
    pub branch: String,
    pub base_commit: String,
}

#[derive(Debug)]
pub enum WorktreeError {
    NotARepo,
    NoCommits,
    BranchExists(String),
    Git(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for WorktreeError {
    fn from(e: std::io::Error) -> Self {
        WorktreeError::Io(e)
    }
}

/// Runs git in `dir`; Ok(stdout trimmed) on success, Err(stderr) on failure.
fn git(dir: &Path, args: &[&str]) -> Result<Result<String, String>, WorktreeError> {
    let out = Command::new("git").current_dir(dir).args(args).output()?;
    if out.status.success() {
        Ok(Ok(String::from_utf8_lossy(&out.stdout).trim().to_string()))
    } else {
        Ok(Err(String::from_utf8_lossy(&out.stderr).trim().to_string()))
    }
}

pub fn create(repo: &Path, agent: &str, id8: &str, root: &Path) -> Result<Worktree, WorktreeError> {
    if !repo.is_dir() || git(repo, &["rev-parse", "--show-toplevel"])?.is_err() {
        return Err(WorktreeError::NotARepo);
    }
    let base_commit = git(repo, &["rev-parse", "HEAD"])?.map_err(|_| WorktreeError::NoCommits)?;
    let branch = format!("agentfleet/{agent}-{id8}");
    let refname = format!("refs/heads/{branch}");
    if git(repo, &["show-ref", "--verify", &refname])?.is_ok() {
        return Err(WorktreeError::BranchExists(branch));
    }
    let name = repo
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "repo".into());
    let parent = root.join(name);
    std::fs::create_dir_all(&parent)?;
    let path = parent.join(id8);
    let path_str = path.to_string_lossy().to_string();
    git(repo, &["worktree", "add", "-b", &branch, &path_str, "HEAD"])?
        .map_err(WorktreeError::Git)?;
    Ok(Worktree {
        repo: repo.to_path_buf(),
        path,
        branch,
        base_commit,
    })
}

/// Use the repo itself as the "worktree": current branch (or short HEAD if detached), nothing created.
pub fn current(repo: &Path) -> Result<Worktree, WorktreeError> {
    if !repo.is_dir() || git(repo, &["rev-parse", "--show-toplevel"])?.is_err() {
        return Err(WorktreeError::NotARepo);
    }
    let base_commit = git(repo, &["rev-parse", "HEAD"])?.map_err(|_| WorktreeError::NoCommits)?;
    let branch = match git(repo, &["symbolic-ref", "--short", "HEAD"])? {
        Ok(b) => b,
        Err(_) => git(repo, &["rev-parse", "--short", "HEAD"])?.map_err(WorktreeError::Git)?,
    };
    Ok(Worktree {
        repo: repo.to_path_buf(),
        path: repo.to_path_buf(),
        branch,
        base_commit,
    })
}

pub fn is_clean(path: &Path) -> Result<bool, WorktreeError> {
    let out = git(path, &["status", "--porcelain"])?.map_err(WorktreeError::Git)?;
    Ok(out.is_empty())
}

pub fn remove(wt: &Worktree) -> Result<(), WorktreeError> {
    // A clean index says nothing about commits made by the agent. Keep both the
    // checkout and its branch unless their tips are reachable from the repo HEAD.
    for revision in ["HEAD", wt.branch.as_str()] {
        let commit = git(&wt.path, &["rev-parse", revision])?.map_err(WorktreeError::Git)?;
        if git(&wt.repo, &["merge-base", "--is-ancestor", &commit, "HEAD"])?.is_err() {
            return Err(WorktreeError::Git(format!(
                "keeping worktree: {revision} has unmerged commits"
            )));
        }
    }
    let path_str = wt.path.to_string_lossy().to_string();
    git(&wt.repo, &["worktree", "remove", &path_str])?.map_err(WorktreeError::Git)?;
    git(&wt.repo, &["branch", "-d", &wt.branch])?.map_err(WorktreeError::Git)?;
    Ok(())
}

pub fn exists(wt: &Worktree) -> bool {
    wt.path.is_dir() && wt.path.join(".git").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(dir: &Path, args: &[&str]) {
        let s = Command::new("git")
            .current_dir(dir)
            .args(args)
            .status()
            .unwrap();
        assert!(s.success());
    }

    fn repo_with_commit(base: &Path) -> PathBuf {
        let repo = base.join("myrepo");
        std::fs::create_dir_all(&repo).unwrap();
        run(&repo, &["init"]);
        run(
            &repo,
            &[
                "-c",
                "user.email=t@t",
                "-c",
                "user.name=t",
                "commit",
                "--allow-empty",
                "-m",
                "init",
            ],
        );
        repo
    }

    #[test]
    fn create_succeeds() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        let root = tmp.path().join("wts");
        let wt = create(&repo, "claude", "abcd1234", &root).unwrap();
        assert_eq!(wt.branch, "agentfleet/claude-abcd1234");
        assert!(exists(&wt));
        assert_eq!(wt.path, root.join("myrepo").join("abcd1234"));
        assert!(!wt.base_commit.is_empty());
        run(
            &repo,
            &[
                "show-ref",
                "--verify",
                "refs/heads/agentfleet/claude-abcd1234",
            ],
        );
    }

    #[test]
    fn current_uses_repo_root_and_branch() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        let wt = current(&repo).unwrap();
        assert_eq!(wt.path, repo);
        let expect = git(&repo, &["symbolic-ref", "--short", "HEAD"])
            .unwrap()
            .unwrap();
        assert_eq!(wt.branch, expect);
        assert!(!wt.base_commit.is_empty());
        let branches = git(&repo, &["branch", "--list", "agentfleet/*"])
            .unwrap()
            .unwrap();
        assert!(branches.is_empty());
        assert!(current(tmp.path()).is_err());
    }

    #[test]
    fn current_detached_uses_short_commit() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        run(&repo, &["checkout", "--detach", "HEAD"]);
        let wt = current(&repo).unwrap();
        assert!(wt.base_commit.starts_with(&wt.branch));
        assert!(wt.branch.len() < wt.base_commit.len());
    }

    #[test]
    fn not_a_repo() {
        let tmp = tempfile::tempdir().unwrap();
        let r = create(tmp.path(), "a", "12345678", &tmp.path().join("w"));
        assert!(matches!(r, Err(WorktreeError::NotARepo)));
    }

    #[test]
    fn no_commits() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path().join("r");
        std::fs::create_dir_all(&repo).unwrap();
        run(&repo, &["init"]);
        let r = create(&repo, "a", "12345678", &tmp.path().join("w"));
        assert!(matches!(r, Err(WorktreeError::NoCommits)));
    }

    #[test]
    fn branch_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        run(&repo, &["branch", "agentfleet/a-12345678"]);
        let r = create(&repo, "a", "12345678", &tmp.path().join("w"));
        assert!(matches!(r, Err(WorktreeError::BranchExists(_))));
    }

    #[test]
    fn clean_then_dirty() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        let wt = create(&repo, "a", "12345678", &tmp.path().join("w")).unwrap();
        assert!(is_clean(&wt.path).unwrap());
        std::fs::write(wt.path.join("f.txt"), "x").unwrap();
        assert!(!is_clean(&wt.path).unwrap());
        assert!(remove(&wt).is_err());
        assert!(exists(&wt));
    }

    #[test]
    fn remove_preserves_unmerged_commits() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        let wt = create(&repo, "a", "abcd1234", &tmp.path().join("w")).unwrap();
        run(
            &wt.path,
            &[
                "-c",
                "user.email=t@t",
                "-c",
                "user.name=t",
                "commit",
                "--allow-empty",
                "-m",
                "work",
            ],
        );
        assert!(is_clean(&wt.path).unwrap());
        assert!(remove(&wt).is_err());
        assert!(exists(&wt));
        assert!(git(
            &repo,
            &["show-ref", "--verify", &format!("refs/heads/{}", wt.branch)]
        )
        .unwrap()
        .is_ok());
        run(&repo, &["merge", "--ff-only", &wt.branch]);
        remove(&wt).unwrap();
        assert!(!exists(&wt));
    }

    #[test]
    fn remove_clean() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = repo_with_commit(tmp.path());
        let wt = create(&repo, "a", "12345678", &tmp.path().join("w")).unwrap();
        remove(&wt).unwrap();
        assert!(!wt.path.exists());
        let s = Command::new("git")
            .current_dir(&repo)
            .args([
                "show-ref",
                "--verify",
                "--quiet",
                "refs/heads/agentfleet/a-12345678",
            ])
            .status()
            .unwrap();
        assert!(!s.success());
    }
}
