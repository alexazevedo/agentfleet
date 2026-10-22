//! PTY spawning: login shell in a directory, streamed output, input, resize, kill.

use std::io::{Read, Write};
use std::path::Path;
use std::sync::Mutex;
use std::thread::JoinHandle;

use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, ChildKiller, CommandBuilder, MasterPty, PtySize};

pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;

pub struct PtyHandle {
    master: Option<Box<dyn MasterPty + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    killer: Box<dyn ChildKiller + Send + Sync>,
    reader: Option<JoinHandle<()>>,
}

pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    shell: &str,
    cwd: &Path,
    command: Option<&str>,
    env: &[(String, String)],
    cols: u16,
    rows: u16,
    on_data: OnData,
    on_exit: Box<dyn Fn(i32) + Send + 'static>,
) -> Result<PtyHandle> {
    let pair = native_pty_system().openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new(shell);
    cmd.arg("-l");
    if let Some(c) = command {
        let cwd_s = cwd.to_string_lossy();
        cmd.arg("-i");
        cmd.arg("-c");
        cmd.arg(format!("cd {} && {}", shell_quote(&cwd_s), c));
    }
    cmd.cwd(cwd);
    cmd.env("TERM", "xterm-256color");
    // Drop Claude Code session markers inherited from a parent session so a
    // nested `claude` behaves as a top-level session. CLAUDE_CONFIG_DIR is kept.
    for (name, _) in std::env::vars_os() {
        let n = name.to_string_lossy();
        if n.starts_with("CLAUDE_CODE_") || n == "CLAUDECODE" {
            cmd.env_remove(&name);
        }
    }
    for (k, v) in env {
        cmd.env(k, v);
    }

    let mut child = pair.slave.spawn_command(cmd)?;
    drop(pair.slave);
    let killer = child.clone_killer();
    let mut reader = pair.master.try_clone_reader()?;
    let writer = pair.master.take_writer()?;

    let join = std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => on_data(buf[..n].to_vec()),
            }
        }
        let code = child.wait().map(|s| s.exit_code() as i32).unwrap_or(-1);
        on_exit(code);
    });

    Ok(PtyHandle {
        master: Some(pair.master),
        writer: Mutex::new(writer),
        killer,
        reader: Some(join),
    })
}

impl PtyHandle {
    pub fn write(&self, bytes: &[u8]) -> Result<()> {
        let mut w = self.writer.lock().map_err(|_| anyhow!("writer poisoned"))?;
        w.write_all(bytes)?;
        w.flush()?;
        Ok(())
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        let m = self.master.as_ref().ok_or_else(|| anyhow!("pty closed"))?;
        m.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    pub fn kill(&mut self) -> Result<()> {
        let r = self.killer.kill();
        self.master = None;
        if let Some(j) = self.reader.take() {
            let _ = j.join();
        }
        r.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{mpsc, Arc};
    use std::time::Duration;

    #[test]
    fn runs_command_and_streams_output() {
        let dir = tempfile::tempdir().unwrap();
        let out = Arc::new(Mutex::new(Vec::new()));
        let out2 = out.clone();
        let (tx, rx) = mpsc::channel();
        let _h = spawn(
            "/bin/sh",
            dir.path(),
            Some("echo PTYOK"),
            &[],
            80,
            24,
            Box::new(move |b| out2.lock().unwrap().extend(b)),
            Box::new(move |c| {
                let _ = tx.send(c);
            }),
        )
        .unwrap();
        rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let text = String::from_utf8_lossy(&out.lock().unwrap()).to_string();
        assert!(text.contains("PTYOK"), "got: {text:?}");
    }

    #[test]
    fn scrubs_claude_code_markers_from_env() {
        std::env::set_var("CLAUDECODE", "1");
        std::env::set_var("CLAUDE_CODE_CHILD_SESSION", "1");
        std::env::set_var("KEEP_ME", "yes");
        let dir = tempfile::tempdir().unwrap();
        let out = Arc::new(Mutex::new(Vec::new()));
        let out2 = out.clone();
        let (tx, rx) = mpsc::channel();
        let _h = spawn(
            "/bin/sh",
            dir.path(),
            Some(r#"echo "[$CLAUDECODE][$CLAUDE_CODE_CHILD_SESSION][$KEEP_ME]""#),
            &[],
            80,
            24,
            Box::new(move |b| out2.lock().unwrap().extend(b)),
            Box::new(move |c| {
                let _ = tx.send(c);
            }),
        )
        .unwrap();
        rx.recv_timeout(Duration::from_secs(5)).unwrap();
        std::env::remove_var("CLAUDECODE");
        std::env::remove_var("CLAUDE_CODE_CHILD_SESSION");
        std::env::remove_var("KEEP_ME");
        let text = String::from_utf8_lossy(&out.lock().unwrap()).to_string();
        assert!(text.contains("[][][yes]"), "got: {text:?}");
    }

    #[test]
    fn write_exit_ends_shell() {
        let dir = tempfile::tempdir().unwrap();
        let (dtx, drx) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let h = spawn(
            "/bin/sh",
            dir.path(),
            None,
            &[],
            80,
            24,
            Box::new(move |_| {
                let _ = dtx.send(());
            }),
            Box::new(move |c| {
                let _ = tx.send(c);
            }),
        )
        .unwrap();
        // Wait for first output (prompt) so the shell is ready.
        drx.recv_timeout(Duration::from_secs(5)).unwrap();
        h.write(b"exit\n").unwrap();
        rx.recv_timeout(Duration::from_secs(5)).unwrap();
    }
}
