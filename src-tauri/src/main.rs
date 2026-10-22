// Prevents an extra console window on Windows release builds (not a target in M1).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--cli") {
        // CLI path: no Tauri initialisation at all.
        let code = agentfleet_lib::cli::main(args[2..].to_vec());
        std::process::exit(code);
    }
    agentfleet_lib::run()
}
