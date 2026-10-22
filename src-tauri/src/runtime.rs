//! Per-launch runtime dir: broker socket + PATH shim for the `agentfleet` CLI.
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::PathBuf;

pub struct Runtime {
    pub dir: PathBuf,
    pub socket: PathBuf,
    pub shim: PathBuf,
}

impl Runtime {
    /// Create `$TMPDIR/agentfleet-<pid>` (0700) with the shim (0755).
    pub fn create() -> std::io::Result<Runtime> {
        let dir = std::env::temp_dir().join(format!("agentfleet-{}", std::process::id()));
        // A stale dir from a previous process with the same pid is ours to replace.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::DirBuilder::new().mode(0o700).create(&dir)?;
        let exe = std::env::current_exe()?;
        let shim = dir.join("agentfleet");
        let script = format!(
            "#!/bin/sh\nexec {} --cli \"$@\"\n",
            crate::pty::shell_quote(&exe.to_string_lossy())
        );
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o755)
            .open(&shim)?;
        f.write_all(script.as_bytes())?;
        Ok(Runtime {
            socket: dir.join("broker.sock"),
            shim,
            dir,
        })
    }

    /// Directory containing the shim, to prepend to PATH.
    pub fn shim_dir(&self) -> &std::path::Path {
        self.shim.parent().unwrap_or(&self.dir)
    }
}
