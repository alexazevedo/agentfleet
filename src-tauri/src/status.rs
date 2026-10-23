//! Agent status and model: poll the hook status file and model source, emitting
//! `session-status` and `session-model`.
use std::path::PathBuf;
use std::time::Duration;

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
    Ended,
}

pub fn parse(s: &str) -> Status {
    match s.trim() {
        "working" => Status::Working,
        "waiting" => Status::Waiting,
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
    if let Ok(mut map) = app.state::<Sessions>().lock() {
        if let Some(s) = map.get_mut(&id) {
            s.status = status;
        }
    }
    let _ = app.emit(
        "session-status",
        StatusEvent {
            id: id.to_string(),
            status,
        },
    );
    crate::deliver::on_status(app, id, status);
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
        let mut last = Status::Unknown;
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
            let cur = std::fs::read_to_string(&status_file)
                .map(|s| parse(&s))
                .unwrap_or(Status::Unknown);
            if cur != last {
                last = cur;
                emit(&app, id, cur);
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
