use magi_core::server::run_server;
use magi_core::init_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    run_server().await
}
