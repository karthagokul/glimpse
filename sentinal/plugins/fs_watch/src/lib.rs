//! Filesystem Watch Plugin (Demo)
//!
//! Watches a file and emits events.
//!
//! NOTE: Actual file watching logic will be added later.

use plugin_api::{GlimpsePlugin};
use plugin_api::context::PluginContext;
use plugin_api::event::Event;

pub struct FsWatchPlugin;

impl GlimpsePlugin for FsWatchPlugin {
    fn name(&self) -> &'static str {
        "fs_watch"
    }

    fn start(&mut self, ctx: PluginContext) {
        println!("fs_watch plugin started");

        // Demo event emission
        (ctx.emit_event)(Event::FileChanged {
            path: "/var/log/dpkg.log".into(),
        });
    }

    fn stop(&mut self) {
        println!("fs_watch plugin stopped");
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn GlimpsePlugin {
    Box::into_raw(Box::new(FsWatchPlugin))
}
