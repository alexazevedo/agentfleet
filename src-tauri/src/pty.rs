//! PTY spawning: login shell in a directory, streamed output, input, resize, kill.

use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use portable_pty::{native_pty_system, ChildKiller, CommandBuilder, MasterPty, PtySize};

pub type OnData = Box<dyn Fn(Vec<u8>) + Send + 'static>;

pub struct PtyHandle {
    master: Option<Box<dyn MasterPty + Send>>,
    writer: PtyWriter,
    killer: Box<dyn ChildKiller + Send + Sync>,
    pid: Option<i32>,
    stop: Arc<AtomicBool>,
    reader: Option<JoinHandle<()>>,
}

/// Clone before releasing the registry lock; terminal I/O must not hold that lock.
#[derive(Clone)]
pub struct PtyWriter {
    file: Arc<Mutex<File>>,
    stop: Arc<AtomicBool>,
}

impl PtyWriter {
    pub fn write(&self, mut bytes: &[u8]) -> Result<()> {
        let mut file = self.file.lock().map_err(|_| anyhow!("writer poisoned"))?;
        let deadline = Instant::now() + Duration::from_secs(5);
        while !bytes.is_empty() {
            if self.stop.load(Ordering::Acquire) {
                return Err(anyhow!("pty closed"));
            }
            if Instant::now() >= deadline {
                return Err(anyhow!("terminal write timed out"));
            }
            match file.write(&bytes[..bytes.len().min(8192)]) {
                Ok(0) => return Err(anyhow!("terminal closed during write")),
                Ok(n) => bytes = &bytes[n..],
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    poll(file.as_raw_fd(), libc::POLLOUT)?;
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}

fn poll(fd: i32, events: i16) -> std::io::Result<()> {
    let mut pfd = libc::pollfd {
        fd,
        events,
        revents: 0,
    };
    // A short timeout lets both reader and writer notice cancellation.
    if unsafe { libc::poll(&mut pfd, 1, 50) } < 0 {
        let e = std::io::Error::last_os_error();
        if e.kind() != std::io::ErrorKind::Interrupted {
            return Err(e);
        }
    }
    Ok(())
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

    let fd = pair
        .master
        .as_raw_fd()
        .ok_or_else(|| anyhow!("pty has no file descriptor"))?;
    let dup = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if dup < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut reader = unsafe { File::from_raw_fd(dup) };
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let stop = Arc::new(AtomicBool::new(false));
    let writer = PtyWriter {
        file: Arc::new(Mutex::new(reader.try_clone()?)),
        stop: stop.clone(),
    };
    let mut child = pair.slave.spawn_command(cmd)?;
    drop(pair.slave);
    let pid = child.process_id().map(|p| p as i32);
    let killer = child.clone_killer();

    let reader_stop = stop.clone();
    let join = std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        while !reader_stop.load(Ordering::Acquire) {
            match reader.read(&mut buf) {
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if poll(reader.as_raw_fd(), libc::POLLIN).is_err() {
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Ok(0) | Err(_) => break,
                Ok(n) => on_data(buf[..n].to_vec()),
            }
        }
        let code = child.wait().map(|s| s.exit_code() as i32).unwrap_or(-1);
        on_exit(code);
    });

    Ok(PtyHandle {
        master: Some(pair.master),
        writer,
        killer,
        pid,
        stop,
        reader: Some(join),
    })
}

impl PtyHandle {
    #[cfg(test)]
    pub fn write(&self, bytes: &[u8]) -> Result<()> {
        self.writer.write(bytes)
    }

    pub fn writer(&self) -> PtyWriter {
        self.writer.clone()
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
        let Some(reader) = self.reader.take() else {
            return Ok(());
        };
        if reader.is_finished() {
            let _ = reader.join();
            self.stop.store(true, Ordering::Release);
            self.master = None;
            return Ok(());
        }
        let foreground = self.master.as_ref().and_then(|m| m.process_group_leader());
        let signal = |sig| {
            for pid in [foreground, self.pid].into_iter().flatten() {
                // portable-pty creates a new session/process group for the child.
                unsafe {
                    libc::kill(-pid, sig);
                }
            }
        };
        signal(libc::SIGTERM);
        let _ = self.killer.kill();
        self.stop.store(true, Ordering::Release);
        std::thread::sleep(Duration::from_millis(100));
        signal(libc::SIGKILL);
        self.master = None;
        let deadline = Instant::now() + Duration::from_secs(1);
        while !reader.is_finished() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        if !reader.is_finished() {
            return Err(anyhow!("terminal reader did not stop"));
        }
        reader
            .join()
            .map_err(|_| anyhow!("terminal reader panicked"))
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
    fn large_write_drains_output_and_close_cancels_stalled_writes() {
        let dir = tempfile::tempdir().unwrap();
        for command in [
            "stty raw -echo; echo READY; exec cat",
            "stty raw -echo; echo READY; exec sleep 30",
        ] {
            let (tx, rx) = mpsc::channel();
            let mut h = spawn(
                "/bin/sh",
                dir.path(),
                Some(command),
                &[],
                80,
                24,
                Box::new(move |b| {
                    let _ = tx.send(b);
                }),
                Box::new(|_| {}),
            )
            .unwrap();
            let mut output = String::new();
            while !output.contains("READY") {
                output.push_str(&String::from_utf8_lossy(
                    &rx.recv_timeout(Duration::from_secs(3)).unwrap(),
                ));
            }
            let writer = h.writer();
            let (done, result) = mpsc::channel();
            let join = std::thread::spawn(move || {
                let _ = done.send(writer.write(&vec![b'x'; 256 * 1024]));
            });
            if command.ends_with("cat") {
                assert!(result.recv_timeout(Duration::from_secs(3)).unwrap().is_ok());
                let mut count = 0;
                while count < 256 * 1024 {
                    count += rx
                        .recv_timeout(Duration::from_secs(3))
                        .unwrap()
                        .iter()
                        .filter(|&&b| b == b'x')
                        .count();
                }
                h.kill().unwrap();
            } else {
                std::thread::sleep(Duration::from_millis(100));
                h.kill().unwrap();
                assert!(result
                    .recv_timeout(Duration::from_secs(1))
                    .unwrap()
                    .is_err());
            }
            join.join().unwrap();
        }
    }

    #[test]
    fn kill_is_bounded_when_child_ignores_hangup() {
        let dir = tempfile::tempdir().unwrap();
        let (ready, rx) = mpsc::channel();
        let mut h = spawn(
            "/bin/sh",
            dir.path(),
            Some("trap '' HUP TERM; echo READY=$$; exec sleep 30"),
            &[],
            80,
            24,
            Box::new(move |b| {
                let _ = ready.send(b);
            }),
            Box::new(|_| {}),
        )
        .unwrap();
        let mut output = String::new();
        while !output.contains("READY=") {
            output.push_str(&String::from_utf8_lossy(
                &rx.recv_timeout(Duration::from_secs(3)).unwrap(),
            ));
        }
        let pid = output
            .split("READY=")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        let (done, result) = mpsc::channel();
        let join = std::thread::spawn(move || {
            let _ = done.send(h.kill());
        });
        let outcome = result.recv_timeout(Duration::from_secs(2));
        // Always clean up the fixture, including when exercising the old implementation.
        if outcome.is_err() {
            let _ = std::process::Command::new("/bin/kill")
                .args(["-KILL", &pid])
                .status();
        }
        join.join().unwrap();
        assert!(outcome.unwrap().is_ok());
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
