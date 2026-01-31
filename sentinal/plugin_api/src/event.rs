//! Event Definitions
//!
//! Events emitted by plugins and consumed by the core.

#[derive(Debug, Clone)]
pub enum Event {
    FileChanged {
        path: String,
    },
    UsbInserted {
        device: String,
    },
}
