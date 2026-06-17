# rustand 🦀

A lightweight, Zustand-inspired state management library for Rust.

If you've used Zustand in the React world, you'll feel right at home. `rustand` provides a simple, infallible, and zero-dependency way to manage shared state in single-threaded environments like WASM or local UI loops.

## Why rustand?

- **Infallible API**: No `.unwrap()` or `Result` everywhere. It just works.
- **Zero Dependencies**: Core library is 100% dependency-free.
- **WASM Ready**: Optimized for single-threaded performance using `Rc` and `RefCell`.
- **Boilerplate Free**: Use macros to turn any struct into a global store or add actions directly to the store.

## Quick Start

```rust
use rustand::Store;

// 1. Create a store
let store = Store::new(0);

// 2. Subscribe to changes
let _sub = store.subscribe(|v| println!("Count is: {}", v));

// 3. Update state (no .unwrap() needed!)
store.set(|s| *s += 1);

// 4. Get state
assert_eq!(store.get(), 1);
```

## Global Stores & Actions

Using the included macros, you can define a global store and attach methods to it for a clean, centralized state management experience.

```rust
use rustand::{global_store, store_actions, Store};

#[global_store]
#[derive(Default, Clone)]
struct Counter {
    value: i32,
}

#[store_actions]
impl Store<Counter> {
    fn increment(&self) {
        self.set(|s| s.value += 1);
    }
}

fn main() {
    let store = Counter::store();
    
    store.subscribe(|state| println!("Value: {}", state.value));
    
    store.increment();
}
```

## Safety & Re-entrancy

To keep the API infallible and safe, `rustand` protects you from recursive updates.

- **Recursive `set`**: If you try to call `store.set()` from inside a `subscribe` callback or another `set` closure, it will panic with a helpful message. This prevents infinite loops and deadlocks.
- **Concurrent `get`**: You can call `store.get()` inside a subscriber, but calling it inside a `set` closure will panic (as the state is currently being mutated).

## Target Use Case

`rustand` is explicitly designed for **single-threaded** environments:
- 🌐 **WASM / Frontend Rust**: Perfect for Yew, Leptos, or Dioxus.
- 🎮 **Games**: Great for managing global game state in engines like Bevy (in single-threaded systems) or Macroquad.
- 🛠️ **CLI Tools**: Simple way to share configuration or state across your app.

If you need a multi-threaded, `Send + Sync` store, this isn't the crate for you (yet!).

## License

MIT
