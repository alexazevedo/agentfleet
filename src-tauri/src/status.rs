//! Claude status: parse the hook status file and poll it, emitting `session-status`.
use std::path::PathBuf;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::watch;
use uuid::Uuid;

use crate::session::Sessions;

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

/// Polls `status_file` every 200 ms; send `true` on the returned-by-caller
/// `watch::Sender` (stored in `Session`) to stop it.
pub fn spawn_poller(
    app: AppHandle,
    id: Uuid,
    status_file: PathBuf,
    mut stop: watch::Receiver<bool>,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_millis(200));
        let mut last = Status::Unknown;
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
}
