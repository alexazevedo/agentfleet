#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;

#[cfg(not(unix))]
compile_error!("agentfleet M1 targets macOS and Linux only");
