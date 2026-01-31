//! Glimpse Core Entry Point
//!
//! Starts the engine and loads plugins.

mod engine;

use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("sentinel=info"))
        )
        .init();

    tracing::info!("Sentinel starting...");
     engine::start();
}

