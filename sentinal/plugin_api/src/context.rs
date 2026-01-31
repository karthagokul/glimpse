//! Plugin Context
//!
//! Provided to plugins by the core engine.
//! Plugins use this to emit events.

use crate::event::Event;

#[derive(Clone)]
pub struct PluginContext {
    pub emit_event: fn(Event),
}
