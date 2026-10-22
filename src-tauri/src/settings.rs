//! User settings persisted at ~/.agentfleet/settings.json.
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::platform;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TerminalSettings {
    /// Empty = system monospace.
    pub font_family: String,
    pub font_size: u8,
    pub line_height: f32,
    pub cursor_blink: bool,
}

impl Default for TerminalSettings {
    fn default() -> Self {
        TerminalSettings {
            font_family: String::new(),
            font_size: 13,
            line_height: 1.0,
            cursor_blink: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub worktrees_root: PathBuf,
    pub default_agent: String,
    pub auto_permission: bool,
    pub auto_delivery: bool,
    pub confirm_quit: bool,
    pub update_checks: bool,
    /// "ask" | "always" | "never"
    pub restore_sessions: String,
    #[serde(default)]
    pub terminal: TerminalSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            worktrees_root: platform::home_dir().join(".agentfleet/worktrees"),
            default_agent: "claude".into(),
            auto_permission: true,
            auto_delivery: true,
            confirm_quit: true,
            update_checks: true,
            restore_sessions: "ask".into(),
            terminal: TerminalSettings::default(),
        }
    }
}

/// Leaf lock: read-and-release; never hold across sessions/projects/collab.
pub type SettingsState = Mutex<Settings>;

pub fn settings_path() -> PathBuf {
    platform::home_dir().join(".agentfleet/settings.json")
}

pub fn load(path: &Path) -> Settings {
    match fs::read_to_string(path) {
        Ok(t) => serde_json::from_str(&t).unwrap_or_else(|e| {
            eprintln!("agentfleet: bad settings.json, using defaults: {e}");
            Settings::default()
        }),
        Err(e) => {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("agentfleet: cannot read settings.json, using defaults: {e}");
            }
            Settings::default()
        }
    }
}

pub fn save(s: &Settings, path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    let json = serde_json::to_string_pretty(s).map_err(io::Error::other)?;
    fs::write(&tmp, json)?;
    fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))?;
    fs::rename(&tmp, path)
}

fn validate(s: &Settings) -> Result<(), String> {
    if !s.worktrees_root.is_absolute() {
        return Err("worktrees root must be an absolute path".into());
    }
    if s.default_agent != "claude" && s.default_agent != "shell" {
        return Err(format!("unknown agent: {}", s.default_agent));
    }
    if !matches!(s.restore_sessions.as_str(), "ask" | "always" | "never") {
        return Err("restore sessions must be ask, always or never".into());
    }
    if !(9..=24).contains(&s.terminal.font_size) {
        return Err("font size must be 9–24".into());
    }
    if !(1.0..=1.6).contains(&s.terminal.line_height) {
        return Err("line height must be 1.0–1.6".into());
    }
    Ok(())
}

#[tauri::command]
pub fn get_settings(state: State<'_, SettingsState>) -> Result<Settings, String> {
    Ok(state.lock().map_err(|e| e.to_string())?.clone())
}

#[tauri::command]
pub fn set_settings(
    settings: Settings,
    state: State<'_, SettingsState>,
) -> Result<Settings, String> {
    validate(&settings)?;
    save(&settings, &settings_path()).map_err(|e| e.to_string())?;
    *state.lock().map_err(|e| e.to_string())? = settings.clone();
    Ok(settings)
}

#[derive(Serialize)]
pub struct AppInfo {
    pub version: &'static str,
    pub commit: &'static str,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        commit: option_env!("AGENTFLEET_COMMIT").unwrap_or("unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let s = Settings::default();
        assert!(s.worktrees_root.ends_with(".agentfleet/worktrees"));
        assert_eq!(s.default_agent, "claude");
        assert!(s.auto_permission && s.auto_delivery && s.confirm_quit && s.update_checks);
        // Partial file fills the rest with defaults.
        let p: Settings = serde_json::from_str(r#"{"autoDelivery":false}"#).unwrap();
        assert!(!p.auto_delivery && p.auto_permission);
        assert_eq!(p.terminal, TerminalSettings::default());
        assert_eq!(s.terminal.font_size, 13);
        assert_eq!(s.terminal.line_height, 1.0);
        assert!(s.terminal.cursor_blink && s.terminal.font_family.is_empty());
    }

    #[test]
    fn round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("sub/settings.json");
        let s = Settings {
            default_agent: "shell".into(),
            auto_permission: false,
            ..Settings::default()
        };
        save(&s, &p).unwrap();
        assert_eq!(load(&p), s);
        assert_eq!(load(&dir.path().join("nope.json")), Settings::default());
    }

    #[test]
    fn invalid_rejected() {
        let bad_agent = Settings {
            default_agent: "vim".into(),
            ..Settings::default()
        };
        assert!(validate(&bad_agent).is_err());
        let rel = Settings {
            worktrees_root: "relative/x".into(),
            ..Settings::default()
        };
        assert!(validate(&rel).is_err());
        assert!(validate(&Settings::default()).is_ok());
        let bad_restore = Settings {
            restore_sessions: "sometimes".into(),
            ..Settings::default()
        };
        assert_eq!(
            validate(&bad_restore).unwrap_err(),
            "restore sessions must be ask, always or never"
        );
        assert_eq!(Settings::default().restore_sessions, "ask");
        for size in [8, 25] {
            let s = Settings {
                terminal: TerminalSettings {
                    font_size: size,
                    ..Default::default()
                },
                ..Settings::default()
            };
            assert_eq!(validate(&s).unwrap_err(), "font size must be 9–24");
        }
        for lh in [0.9, 1.7] {
            let s = Settings {
                terminal: TerminalSettings {
                    line_height: lh,
                    ..Default::default()
                },
                ..Settings::default()
            };
            assert_eq!(validate(&s).unwrap_err(), "line height must be 1.0–1.6");
        }
    }
}
