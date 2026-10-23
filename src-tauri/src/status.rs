//! Agent status and model: poll the hook status file and model source, emitting
//! `session-status` and `session-model`.
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::watch;
use uuid::Uuid;

use crate::project::Projects;
use crate::session::Sessions;

/// Where a session's current model is read from.
#[derive(Debug, Clone)]
pub enum ModelSource {
    /// Agent has no model reporting channel.
    None,
    /// Plain text file (Claude: written by `agentfleet statusline`).
    PlainFile(PathBuf),
    /// Codex hook payload JSON with a top-level `model`.
    CodexPayload(PathBuf),
}

/// Current model from `src`; None when unavailable or empty.
pub fn read_model(src: &ModelSource) -> Option<String> {
    match src {
        ModelSource::None => None,
        ModelSource::PlainFile(p) => std::fs::read_to_string(p)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
        ModelSource::CodexPayload(p) => crate::agent::codex_model(p),
    }
}

#[derive(Clone, Serialize)]
struct ModelEvent {
    id: String,
    model: String,
}

/// Record the model on the session, notify the UI (`session-model`), persist.
pub fn emit_model(app: &AppHandle, id: Uuid, model: String) {
    // Lock order: sessions, then projects.
    if let Ok(mut map) = app.state::<Sessions>().lock() {
        if let Some(s) = map.get_mut(&id) {
            s.model = model.clone();
        }
        if let Ok(projects) = app.state::<Projects>().lock() {
            crate::session::persist(&map, &projects);
        }
    }
    let _ = app.emit(
        "session-model",
        ModelEvent {
            id: id.to_string(),
            model,
        },
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Unknown,
    Working,
    Waiting,
    Blocked,
    Ended,
}

pub fn parse(s: &str) -> Status {
    match s.trim() {
        "working" => Status::Working,
        "waiting" => Status::Waiting,
        "blocked" => Status::Blocked,
        "ended" => Status::Ended,
        _ => Status::Unknown,
    }
}

#[derive(Clone, Serialize)]
struct StatusEvent {
    id: String,
    status: Status,
}

/// Single status sink: record on the session, notify the UI, then let delivery react.
pub fn emit(app: &AppHandle, id: Uuid, status: Status) {
    emit_if(app, id, status, None);
}

fn transition(current: &mut Status, next: Status, expected: Option<Status>) -> bool {
    if *current == Status::Ended || expected.is_some_and(|s| s != *current) {
        return false;
    }
    *current = next;
    true
}

/// Compare-and-set readiness so a late startup timer cannot overwrite Working/Ended.
pub fn mark_waiting_if_unknown(app: &AppHandle, id: Uuid) {
    emit_if(app, id, Status::Waiting, Some(Status::Unknown));
}

fn emit_if(app: &AppHandle, id: Uuid, status: Status, expected: Option<Status>) {
    let changed = app
        .state::<Sessions>()
        .lock()
        .ok()
        .and_then(|mut map| {
            map.get_mut(&id)
                .map(|s| transition(&mut s.status, status, expected))
        })
        .unwrap_or(false);
    if !changed {
        return;
    }
    publish(app, id, status);
}

/// Announce a state already recorded under the session registry lock.
pub fn publish(app: &AppHandle, id: Uuid, status: Status) {
    let sessions = app.state::<Sessions>();
    let Ok(map) = sessions.lock() else {
        return;
    };
    if map.get(&id).map(|s| s.status) != Some(status) {
        return;
    }
    let _ = app.emit(
        "session-status",
        StatusEvent {
            id: id.to_string(),
            status,
        },
    );
    drop(map);
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || crate::deliver::on_status(&app, id, status));
}

#[derive(Clone, PartialEq)]
struct StatusSnapshot {
    status: Status,
    modified: SystemTime,
    len: u64,
}

fn read_status(path: &std::path::Path) -> Option<StatusSnapshot> {
    let status = parse(&std::fs::read_to_string(path).ok()?);
    // Hooks truncate then write. An empty/partial file is not a real state change.
    if status == Status::Unknown {
        return None;
    }
    let metadata = std::fs::metadata(path).ok()?;
    Some(StatusSnapshot {
        status,
        modified: metadata.modified().ok()?,
        len: metadata.len(),
    })
}

fn observe(
    current: &mut Status,
    last: &mut Option<StatusSnapshot>,
    sample: StatusSnapshot,
) -> bool {
    if last.as_ref() == Some(&sample) {
        return false;
    }
    let status = sample.status;
    *last = Some(sample);
    transition(current, status, None)
}

/// Polls `status_file` and `model_source` every 200 ms; send `true` on the
/// returned-by-caller `watch::Sender` (stored in `Session`) to stop it.
pub fn spawn_poller(
    app: AppHandle,
    id: Uuid,
    status_file: PathBuf,
    model_source: ModelSource,
    mut stop: watch::Receiver<bool>,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_millis(200));
        let mut last = None;
        // Seeded from the session so a restored (persisted) model is not re-emitted.
        let mut last_model = app
            .state::<Sessions>()
            .lock()
            .ok()
            .and_then(|m| m.get(&id).map(|s| s.model.clone()))
            .unwrap_or_default();
        loop {
            tokio::select! {
                _ = tick.tick() => {}
                _ = stop.changed() => break,
            }
            if *stop.borrow() {
                break;
            }
            // Read and apply under the same lock as delivery reservations, so a
            // previously sampled idle value cannot overwrite a new reservation.
            let updated = app.state::<Sessions>().lock().ok().and_then(|mut map| {
                let s = map.get_mut(&id)?;
                let sample = read_status(&status_file)?;
                let status = sample.status;
                observe(&mut s.status, &mut last, sample).then_some(status)
            });
            if let Some(status) = updated {
                publish(&app, id, status);
            }
            if let Some(m) = read_model(&model_source) {
                if m != last_model {
                    last_model = m.clone();
                    emit_model(&app, id, m);
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_idle_updates_release_reservations_without_replaying_old_samples() {
        let mut status = Status::Unknown;
        let mut last = None;
        let sample = StatusSnapshot {
            status: Status::Waiting,
            modified: SystemTime::now(),
            len: 7,
        };
        assert!(observe(&mut status, &mut last, sample.clone()));
        status = Status::Working; // request delivery reserves the composer
        assert!(!observe(&mut status, &mut last, sample.clone()));
        assert_eq!(status, Status::Working);
        // A complete fast turn may fit between two polls; its new idle write is still observed.
        let next = StatusSnapshot {
            modified: sample.modified + Duration::from_millis(1),
            ..sample
        };
        assert!(observe(&mut status, &mut last, next));
        assert_eq!(status, Status::Waiting);
    }

    #[test]
    fn late_status_updates_cannot_resurrect_or_reidle_sessions() {
        let mut current = Status::Working;
        assert!(!transition(
            &mut current,
            Status::Waiting,
            Some(Status::Unknown)
        ));
        assert_eq!(current, Status::Working);
        assert!(transition(&mut current, Status::Ended, None));
        assert!(!transition(&mut current, Status::Waiting, None));
        assert_eq!(current, Status::Ended);
        assert_eq!(parse("blocked"), Status::Blocked);
    }

    #[test]
    fn parses_words() {
        assert_eq!(parse("working"), Status::Working);
        assert_eq!(parse(" waiting\n"), Status::Waiting);
        assert_eq!(parse("ended"), Status::Ended);
        assert_eq!(parse(""), Status::Unknown);
        assert_eq!(parse("bogus"), Status::Unknown);
    }

    #[test]
    fn reads_model_sources() {
        let dir = tempfile::tempdir().unwrap();
        let plain = dir.path().join("x.model");
        let src = ModelSource::PlainFile(plain.clone());
        assert_eq!(read_model(&src), None);
        std::fs::write(&plain, "  \n").unwrap();
        assert_eq!(read_model(&src), None);
        std::fs::write(&plain, "Opus 5.5\n").unwrap();
        assert_eq!(read_model(&src).as_deref(), Some("Opus 5.5"));

        let payload = dir.path().join("x.codex-hook.json");
        let src = ModelSource::CodexPayload(payload.clone());
        assert_eq!(read_model(&src), None);
        std::fs::write(&payload, r#"{"model":"gpt-5.5"}"#).unwrap();
        assert_eq!(read_model(&src).as_deref(), Some("gpt-5.5"));
        std::fs::write(&payload, r#"{"session_id":"s"}"#).unwrap();
        assert_eq!(read_model(&src), None);
    }
}
