# rustand 🦀

[![Crates.io](https://img.shields.io/crates/v/rustand.svg)](https://crates.io/crates/rustand)
[![Docs.rs](https://docs.rs/rustand/badge.svg)](https://docs.rs/rustand)
[![CI](https://github.com/nikarm22/rustand/actions/workflows/ci.yml/badge.svg)](https://github.com/nikarm22/rustand/actions)
[![License](https://img.shields.io/crates/l/rustand.svg)](https://github.com/nikarm22/rustand/blob/main/LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://github.com/rust-lang/rust/releases/tag/1.85.0)
[![Downloads](https://img.shields.io/crates/d/rustand.svg)](https://crates.io/crates/rustand)

A lightweight, Zustand-inspired state management library for Rust.

`rustand` provides simple API to create and use globally available state stores. Its philosophy is simplicity and ease of use, without any third-party runtime dependencies.

Notes:
- As an initial release current implementation is for single-threaded environments like UI loops or WASM.
- We do have 3 build time dependencies: `syn`, `quote`, `proc-macro2`, which you can opt out by disabling the default `macros` feature.

## Quick Start

```bash
cargo add rustand
```

```rust
use rustand::Store;

let store = Store::new(0);

// subscribe method returns RAII handle, while the handle is alive, subscription is alive
// you can safely drop it to unsubscribe, or you can unsubscribe via subscription ID
let _sub = store.subscribe(|v| println!("Count is: {}", v));

store.set(|s| *s += 1);

// Manual getter
assert_eq!(store.get(), 1);
```

## Global Stores & Actions

Since traditional zustand stores are globally available, you can use `std::cell::OnceCell` (or `thread_local!`) to have a static global store:

```rust
use std::cell::OnceCell;
use rustand::Store;

thread_local! {
    static COUNTER: OnceCell<Store<i32>> = OnceCell::new();
}

fn get_counter() -> Store<i32> {
    COUNTER.with(|c| c.get_or_init(|| Store::new(0)).clone())
}
```

To have a zustand like look and feel, you can define a trait extension for your stores:

```rust
pub trait CounterExt {
    fn increment(&self);
}

impl CounterExt for Store<i32> {
    fn increment(&self) {
        self.set(|s| *s += 1);
    }
}
```

To reduce boilerplate, the same can be achieved with `rustand::global_store` and `rustand::store_actions` macros.


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

## Implementation limitation

To keep the API as simple as possible, `rustand` explicitly denies updates inside subscription and `get()`s in `set` callback (you don't need it anyway, `set` provides `&mut T` so naturally you can't get `&T` during update).
Note: during development and testing, this functionality forced us to handle `BorrowError`s and propagate `Result` adding API usage friction.

## Motivation

I was writing a rust app and I needed a small UI for non-technical user interactions, I used `slint` for lightweight UI framework, but coming from Front-End world, I saw a massive friction in rust ecosystem with UI state management. I strongly believe that UI App state should be easy to use. This project started as a simple wrapper over `Arc<RwLock<T>>`.

## Multiple threads, async and what's next

As of `0.1.0` multi-threaded or async runtime support is work in progress. I'm experimenting with different architectural choices for thread synchronization and subscriber notifications. This is heavily stripped-down version for the package, so I can get a bit of feedback on API and potential use-cases, to make more informed decisions.

Note: Likely in the next release, I'll re-enable `multi-threaded` feature flag, with ring buffer safe implementation.

## License

MIT
