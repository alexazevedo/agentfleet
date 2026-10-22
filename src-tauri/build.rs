use std::process::Command;

fn main() {
    // Short git SHA baked into the binary (About panel reads AGENTFLEET_COMMIT).
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=AGENTFLEET_COMMIT={commit}");
    println!("cargo:rerun-if-changed=../.git/HEAD");

    tauri_build::build()
}
