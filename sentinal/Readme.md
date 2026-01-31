# Glimpse Sentinel
**Plugin‑First System Observation Engine (Rust)**

Glimpse Sentinel is the **core backend engine** of the Glimpse ecosystem.
It is responsible for loading plugins, observing the system, emitting events,
and acting as the stable foundation on which all extensions are built.

Sentinel is intentionally minimal, fast, and opinionated.

---

## Philosophy

- **Plugin‑first** — all functionality lives in plugins
- **Stable core** — the engine changes rarely
- **Event‑driven** — plugins emit signals, the system reacts
- **Linux‑native** — designed for Linux systems first
- **Rust‑powered** — memory‑safe, concurrent, reliable

If it doesn’t belong in the core, it belongs in a plugin.

---

## Repository Structure

```text
sentinel/
├── Cargo.toml              # Rust workspace
├── README.md               # This file
│
├── core/                   # Sentinel engine (runtime)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── engine.rs       # Engine bootstrap
│   │   └── plugin_manager.rs (future)
│
├── plugin_api/             # Stable plugin contract
│   └── src/
│       ├── lib.rs          # Plugin trait
│       ├── event.rs        # Event definitions
│       └── context.rs      # PluginContext
│
├── plugins/                # Extensions (compiled as .so)
│   └── fs_watch/           # Example plugin
│
└── plugins-bin/            # Runtime-loaded shared objects (.so)
```

---

## Setting Up Rust (Beginner Friendly)

### Install Rust (Recommended Way)

Run this **once**:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal.

Verify installation:

```bash
rustc --version
cargo --version
```

---

### Install System Dependencies (Linux)

```bash
sudo apt update
sudo apt install -y build-essential pkg-config
```

---

## Building Sentinel

From the root directory:

```bash
cargo build
```

This builds:
- the core engine
- the plugin API
- example plugins

---

## Building a Plugin

Example: build the `fs_watch` plugin

```bash
cd plugins/fs_watch
cargo build --release
```

The output will be a shared library:

```text
target/release/libfs_watch.so
```

Copy it to the runtime plugin directory:

```bash
cp target/release/libfs_watch.so ../../plugins-bin/
```

---

## Running Sentinel

```bash
cargo run -p glimpse-core
```

(Current version prints startup logs and demo plugin events.)

---

## Writing Your Own Plugin

Every plugin must:

1. Be a separate Cargo crate
2. Compile as `cdylib`
3. Implement `GlimpsePlugin`
4. Export `create_plugin()`

Minimal example:

```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn GlimpsePlugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

---

## What Comes Next

Planned features:
- Dynamic plugin loading (`libloading`)
- Event bus (channels)
- Plugin lifecycle management
- Hot reload (optional)
- Sandboxing & fault isolation

---

## Naming Note

**Sentinel** is the *engine*.
**Glimpse** is the *platform*.

Think:
- Glimpse = ecosystem
- Sentinel = heart

---

## License

Parent Project

---

## Final Note

If you are new to Rust:
- Read code slowly
- Ignore lifetimes initially
- Focus on structure, not syntax

Sentinel is designed to **teach by structure**.

Welcome aboard.
