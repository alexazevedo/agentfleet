//! Auto-delivery of queued runs into the target's terminal (bracketed paste).
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::collab::{self, CollabNotify, CollabState, RunStatus};
use crate::session::{Agent, Sessions};
use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum Decision {
    Deliver,
    Wait,
    NeedsAttention(&'static str),
}

pub fn can_deliver(
    status: Status,
    draft_len: usize,
    agent: Agent,
    has_pty: bool,
    force: bool,
) -> Decision {
    if !has_pty {
        return Decision::NeedsAttention("target has no terminal attached");
    }
    if agent != Agent::Claude {
        return Decision::NeedsAttention("target is not a claude session");
    }
    if status == Status::Ended {
        return Decision::NeedsAttention("session ended");
    }
    if force || (status == Status::Waiting && draft_len == 0) {
        Decision::Deliver
    } else {
        Decision::Wait
    }
}

/// Track the user's draft length across a chunk of terminal input, ignoring
/// control/reply sequences. Shortcut: sequences split across writes are not handled.
pub fn apply_draft(draft: usize, bytes: &[u8]) -> usize {
    let mut d = draft;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        i += 1;
        match b {
            0x1b => match bytes.get(i) {
                Some(b'[') => {
                    i += 1;
                    while i < bytes.len() && !(0x40..=0x7e).contains(&bytes[i]) {
                        i += 1;
                    }
                    i += 1; // final byte
                }
                Some(b'O') => i += 2,
                Some(_) => i += 1,
                None => {}
            },
            b'\r' | b'\n' => d = 0,
            0x7f | 0x08 => d = d.saturating_sub(1),
            0..=0x1f | 0x80..=0xbf => {}
            _ => d += 1,
        }
    }
    d
}

pub fn paste_bytes(prompt: &str) -> Vec<u8> {
    let norm = prompt.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = b"\x1b[200~".to_vec();
    out.extend_from_slice(norm.as_bytes());
    out.extend_from_slice(b"\x1b[201~\r");
    out
}

pub fn try_deliver(app: &AppHandle, run_id: Uuid, force: bool) -> Result<bool, String> {
    let sessions = app.state::<Sessions>();
    let collab_state = app.state::<CollabState>();
    let project = {
        let map = sessions.lock().map_err(|e| e.to_string())?;
        let mut c = collab_state.lock().map_err(|e| e.to_string())?;
        let Some(run) = c.runs.iter().find(|r| r.id == run_id).cloned() else {
            return Ok(false);
        };
        if run.status != RunStatus::Queued {
            return Ok(false);
        }
        let now = collab::now();
        let decision = match map.get(&run.to_session_id) {
            Some(s) => can_deliver(s.status, s.draft_len, s.agent, s.pty.is_some(), force),
            None => Decision::NeedsAttention("session ended"),
        };
        match decision {
            Decision::Wait => return Ok(false),
            Decision::NeedsAttention(msg) => {
                collab::set_status(&mut c, run_id, RunStatus::NeedsAttention, Some(msg), now);
            }
            Decision::Deliver => {
                let pty = map
                    .get(&run.to_session_id)
                    .and_then(|s| s.pty.as_ref())
                    .ok_or("session has no pty")?;
                match pty.write(&paste_bytes(&run.prompt)) {
                    Ok(()) => collab::set_status(&mut c, run_id, RunStatus::Running, None, now),
                    Err(e) => {
                        let msg = format!("delivery failed: {e}");
                        collab::set_status(
                            &mut c,
                            run_id,
                            RunStatus::NeedsAttention,
                            Some(&msg),
                            now,
                        );
                    }
                }
            }
        }
        collab::save(&c);
        run.project_id
    };
    collab::changed(app, &app.state::<CollabNotify>().0, &[project]);
    Ok(true)
}

pub fn on_status(app: &AppHandle, session: Uuid, status: Status) {
    match status {
        Status::Waiting => {
            let next = {
                let cs = app.state::<CollabState>();
                let Ok(c) = cs.lock() else {
                    return;
                };
                c.runs
                    .iter()
                    .filter(|r| r.to_session_id == session && r.status == RunStatus::Queued)
                    .min_by_key(|r| r.created_at)
                    .map(|r| r.id)
            };
            if let Some(id) = next {
                if let Err(e) = try_deliver(app, id, false) {
                    eprintln!("agentfleet: delivery failed: {e}");
                }
            }
        }
        Status::Ended => {
            let projects = {
                let cs = app.state::<CollabState>();
                let Ok(mut c) = cs.lock() else {
                    return;
                };
                let now = collab::now();
                let mut projects = Vec::new();
                for r in c.runs.iter_mut() {
                    if r.to_session_id == session && !r.status.is_terminal() {
                        r.status = RunStatus::NeedsAttention;
                        r.detail = Some("session ended".into());
                        r.updated_at = now;
                        if !projects.contains(&r.project_id) {
                            projects.push(r.project_id);
                        }
                    }
                }
                if !projects.is_empty() {
                    collab::save(&c);
                }
                projects
            };
            if !projects.is_empty() {
                collab::changed(app, &app.state::<CollabNotify>().0, &projects);
            }
        }
        _ => {}
    }
}

pub fn on_request_created(app: &AppHandle, run_id: Uuid) {
    if let Err(e) = try_deliver(app, run_id, false) {
        eprintln!("agentfleet: delivery failed: {e}");
    }
}

#[tauri::command]
pub fn execute_run(id: String, app: AppHandle) -> Result<bool, String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    try_deliver(&app, id, true)
}

#[tauri::command]
pub fn cancel_run(id: String, app: AppHandle) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let project = {
        let cs = app.state::<CollabState>();
        let mut c = cs.lock().map_err(|e| e.to_string())?;
        let live = c
            .runs
            .iter()
            .find(|r| r.id == id)
            .filter(|r| !r.status.is_terminal());
        let Some(project) = live.map(|r| r.project_id) else {
            return Ok(());
        };
        collab::set_status(&mut c, id, RunStatus::Cancelled, None, collab::now());
        collab::save(&c);
        project
    };
    collab::changed(&app, &app.state::<CollabNotify>().0, &[project]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decisions() {
        use Decision::*;
        let c = Agent::Claude;
        assert_eq!(can_deliver(Status::Waiting, 0, c, true, false), Deliver);
        assert_eq!(can_deliver(Status::Waiting, 3, c, true, false), Wait);
        assert_eq!(can_deliver(Status::Working, 0, c, true, false), Wait);
        assert_eq!(can_deliver(Status::Unknown, 0, c, true, false), Wait);
        assert_eq!(can_deliver(Status::Working, 5, c, true, true), Deliver);
        assert!(matches!(
            can_deliver(Status::Waiting, 0, c, false, false),
            NeedsAttention(_)
        ));
        assert!(matches!(
            can_deliver(Status::Waiting, 0, Agent::Shell, true, true),
            NeedsAttention(_)
        ));
        assert_eq!(
            can_deliver(Status::Ended, 0, c, true, true),
            NeedsAttention("session ended")
        );
    }

    #[test]
    fn draft_tracking() {
        assert_eq!(apply_draft(0, b"\x1b[I"), 0);
        assert_eq!(apply_draft(2, b"\x1b[12;40R"), 2);
        assert_eq!(apply_draft(2, b"\x1b[A"), 2);
        assert_eq!(apply_draft(0, b"abc"), 3);
        assert_eq!(apply_draft(0, "é".as_bytes()), 1);
        assert_eq!(apply_draft(0, b"ab\x7f"), 1);
        assert_eq!(apply_draft(0, b"abc\r"), 0);
        assert_eq!(apply_draft(0, b"\x1b[200~hi\x1b[201~"), 2);
    }

    #[test]
    fn paste_encoding() {
        assert_eq!(paste_bytes("hi"), b"\x1b[200~hi\x1b[201~\r");
        assert_eq!(
            paste_bytes("a\r\nb\rc\nd"),
            b"\x1b[200~a\nb\nc\nd\x1b[201~\r"
        );
    }
}
