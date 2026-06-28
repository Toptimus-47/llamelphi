pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod adapters;
pub mod server;

#[cfg(test)]
mod tests;

use std::sync::Once;
use std::thread;
use tracing::{info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub use domain::{AgentState, WorkflowStep};

pub fn normalize_text(text: &str) -> String {
    text.replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn mask_pii(text: &str) -> String {
    domain::services::pii::PiiService::mask_pii(text)
}

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("magi_core=trace,magi_server=trace"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_ansi(true))
        .try_init();
    
    info!("===============================================================");
    info!("   MAGI CORE SYSTEM: FFI BINDING INITIALIZED");
    info!("===============================================================");
}

// --- FFI EXPORTS ---
static INIT: Once = Once::new();

#[no_mangle]
pub extern "C" fn start_magi_backend() -> i32 {
    let mut success = 0;
    INIT.call_once(|| {
        init_logging();
        thread::spawn(|| {
            if let Err(e) = run_backend_sync() {
                eprintln!("[MAGI Rust] Backend Fatal Error: {}", e);
            }
        });
        success = 1;
    });
    success
}

/// Internal synchronous entry point for the backend.
fn run_backend_sync() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        info!("[MAGI Rust] FFI: Backend thread started.");
        server::run_server().await
    })
}
