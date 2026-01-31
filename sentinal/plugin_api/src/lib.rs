//! Glimpse Plugin API
//!
//! This crate defines the **only contract** between the core engine
//! and all plugins.
//!
//! Rules:
//! - No IO here
//! - No threading here
//! - No logic here
//!
//! This crate must remain stable.

pub mod event;
pub mod context;

use context::PluginContext;

/// Every Glimpse plugin must implement this trait.
pub trait GlimpsePlugin {
    /// Human-readable plugin name
    fn name(&self) -> &'static str;

    /// Called once when the plugin is started
    fn start(&mut self, ctx: PluginContext);

    /// Called before plugin shutdown
    fn stop(&mut self);
}
