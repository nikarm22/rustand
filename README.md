# rustand

[![Crates.io](https://img.shields.io/crates/v/rustand.svg)](https://crates.io/crates/rustand)
[![Documentation](https://docs.rs/rustand/badge.svg)](https://docs.rs/rustand)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/nikarm22/rustand/actions/workflows/ci.yml/badge.svg)](https://github.com/nikarm22/rustand/actions)

**The minimalist, deadlock-free state manager for Rust.**

`rustand` is a lightweight state management library inspired by [Zustand](https://github.com/pmndrs/zustand). It provides a simple `get`/`set`/`subscribe` pattern designed for high-performance concurrent applications, UI frameworks, and WASM environments.

## 🚀 Key Features

- **🎯 Simple API:** Minimalist `get`, `set`, and `subscribe` pattern.
- **🛡️ Stability & Performance:** Deadlock-free by design. Releases locks before notifying subscribers to ensure minimal lock contention and predictable performance.
- **🏗️ Runtime Agnostic:** Core logic is runtime-independent, with first-class support for **Tokio** and **async-std**.
- **🦀 Zero Dependencies:** The core remains dependency-free. Runtime-specific features only bring in the necessary dependencies.
- **🌐 WASM Ready:** Specialized `single-threaded` mode eliminates `Arc` and atomic lock overhead for maximum UI performance.
- **🔒 Strictly No-Unsafe:** Verified safe Rust.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustand = "0.1"
```

### Feature Matrix

| Feature | Description | Primitives | Concurrency | Runtime |
| :--- | :--- | :--- | :--- | :--- |
| `multi-threaded` | (Default) Thread-safe store. | `Arc` + `RwLock` | `Send + Sync` | `std` |
| `single-threaded`| High-performance UI/WASM mode. | `Rc` + `RefCell` | `!Send + !Sync` | None |
| `tokio` | Tokio-specific sync primitives. | `Arc` + `Tokio RwLock` | `Send + Sync` | `tokio` |
| `async-std` | async-std sync primitives. | `Arc` + `async-std RwLock`| `Send + Sync` | `async-std` |

---

## 💡 Motivation

This project was born out of frustration while developing a [Slint](https://slint.dev/) application. Coming from years of front-end development, I found the ergonomics of existing UI state management in Rust to be severely lacking. I believe that in this day and age, managing state should be intuitive and seamless, even in a systems language. `rustand` is my attempt to bring that front-end simplicity to the Rust ecosystem without compromising on safety or performance.

---

## 💻 Quick Start

### Multi-threaded
```rust
use rustand::Store;

#[tokio::main]
async fn main() {
    let store = Store::new(0);

    let _sub = store.subscribe(|v| println!("Value: {}", v)).unwrap();

    store.set(|s| *s += 1).unwrap();
}
```

### UI/WASM (Sync)
```rust
use rustand::Store;

fn main() {
    let store = Store::new("Hello".to_string());

    store.subscribe(|v| println!("State: {}", v)).unwrap();
    store.set(|s| *s = "World".to_string()).unwrap();
}
```

---

## 🗺️ Ecosystem Roadmap

- [ ] `rustand-slint`
- [ ] `rustand-tauri` (with npm counterpart)
- [ ] `rustand-leptos`
- [ ] `rustand-egui`
- [ ] `rustand-redox`

---

## 🤝 Contributing

Contributions and suggestions are welcomed! If you have ideas for new features or find any bugs, please open an [issue](https://github.com/nikarm22/rustand/issues).

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## ⚖️ License

Licensed under the [MIT License](LICENSE).
