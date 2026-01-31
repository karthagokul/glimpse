# Writing a Glimpse Plugin

Each plugin:
- Is its own Cargo crate
- Compiles to a `.so` file
- Implements `GlimpsePlugin`
- Emits events via `PluginContext`

Plugins are loaded at runtime by the core engine.
