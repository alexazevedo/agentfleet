// Prevents an extra console window on Windows release builds (not a target in M1).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    agentfleet_lib::run()
}
