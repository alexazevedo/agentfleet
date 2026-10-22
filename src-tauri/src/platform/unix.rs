use std::path::PathBuf;

/// The user's home directory (`~`).
pub fn home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
}

/// The login shell from `$SHELL`, falling back to `/bin/sh`.
pub fn login_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "/bin/sh".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_shell_is_non_empty() {
        assert!(!login_shell().is_empty());
    }
}
