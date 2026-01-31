//! Glimpse Core Entry Point
//!
//! Starts the engine and loads plugins.

mod engine;

fn main() {
    println!("Starting Glimpse core engine...");
    engine::start();
}
