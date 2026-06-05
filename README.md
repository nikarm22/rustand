# rustand

A lightweight, Zustand-inspired state management library for Rust.

`rustand` provides a simple, thread-safe way to manage shared state with subscription support. It is designed to be lean, dependency-free, and easy to use across various concurrency models and async runtimes.

## Key Features

- **Simple API:** Minimalist `get`, `set`, and `subscribe` pattern.
- **Concurrency Safe:** releases locks before notifying subscribers, preventing recursive deadlocks.
- **Pluggable Runtimes:** First-class support for Standard Threads, Tokio, and async-std.
- **WASM Friendly:** Dedicated `single-threaded` mode for browser and UI environments.
- **Zero Dependencies:** Keeps your project's dependency graph small.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustand = "0.1.0"
```

### Feature Flags

- `multi-threaded` (default): Uses `std::sync` primitives.
- `single-threaded`: Uses `Rc` and `RefCell` (no atomic overhead).
- `wasm`: Alias for `single-threaded`, optimized for WebAssembly.
- `tokio`: Integrates with the Tokio async runtime.
- `async-std`: Integrates with the async-std runtime.

## Usage

### Basic Example (Multi-threaded)

```rust
use rustand::Store;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Create a new store
    let store = Store::new(0);

    // Subscribe to changes
    let _sub = store.subscribe(|v| {
        println!("Count changed to: {}", v);
    }).await.unwrap();

    // Update state from multiple tasks
    let mut handles = vec![];
    for _ in 0..10 {
        let store = store.clone();
        handles.push(tokio::spawn(async move {
            store.set(|s| *s += 1).await.unwrap();
        }));
    }

    for h in handles { h.await.unwrap(); }

    assert_eq!(store.get().await.unwrap(), 10);
}
```

### Single-threaded Example (WASM/UI)

```rust
// Cargo.toml: rustand = { version = "0.1.0", default-features = false, features = ["single-threaded"] }
use rustand::Store;

fn main() {
    let store = Store::new("Initial".to_string());

    store.subscribe_sync(|v| println!("State is now: {}", v)).unwrap();

    // No need for Arc or Mutex in single-threaded mode
    store.set_sync(|s| *s = "Updated".to_string()).unwrap();
}
```

## More Examples

Check the `examples/` directory for more advanced patterns:
- `custom_struct.rs`: Using a complex struct for state.
- `global_store.rs`: Setting up a static global store using `OnceLock`.
- `tokio_usage.rs`: Integration with the Tokio runtime and background tasks.

## How it works

`rustand` works by taking a **snapshot** of the state and the subscribers list before notification. This ensures that:
1. Locks are held for the minimum time possible.
2. Subscribers can safely call `store.set()` or `store.get()` without causing a deadlock.
3. The state seen by a subscriber is consistent for the duration of the callback.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
