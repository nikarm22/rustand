//! # rustand
//!
//! A lightweight, Zustand-inspired state management library for Rust.
//!
//! `rustand` provides a simple, single-threaded way to manage shared state with subscription support.
//! It is designed to be lean, dependency-free, and easy to use.
//!
//! ## Core Features
//! - **Simple API**: Minimalist `get`, `set`, and `subscribe` pattern.
//! - **Infallible**: API methods don't return `Result`, making usage clean and idiomatic.
//! - **Zero Dependencies**: Core library has no external dependencies.
//!
//! ## Feature Flags
//! - `single-threaded` (default): Optimized for WASM or single-threaded environments. Uses `Rc` and `RefCell` to avoid atomic overhead.
//! - `wasm`: Alias for `single-threaded`, targeting WebAssembly environments.
//!
//! ## Quick Start
//!
//! ```rust
//! use rustand::Store;
//!
//! let store = Store::new(0);
//!
//! // Subscribe to changes
//! let _sub = store.subscribe(|v| println!("New value: {}", v));
//!
//! // Update state
//! store.set(|s| *s += 1);
//!
//! // Get state
//! assert_eq!(store.get(), 1);
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod single_threaded;
mod store;
mod subscription;

pub use rustand_macros::{global_store, store_actions};
pub use store::Store;
pub use subscription::{SubscriberCallback, SubscriberId, Subscription};
