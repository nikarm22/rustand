//! # rustand
//!
//! A lightweight, Zustand-inspired state management library for Rust.
//!
//! `rustand` provides a simple, thread-safe way to manage shared state with subscription support.
//! It is designed to be lean, dependency-free, and easy to use in both multi-threaded and
//! single-threaded environments.
//!
//! ## Core Features
//! - **Simple API**: Minimalist `get`, `set`, and `subscribe` pattern.
//! - **Concurrency Safe**: Handles recursive updates (calling `set` inside a subscriber) without deadlocking.
//! - **Pluggable Runtimes**: Supports standard threads, Tokio, and async-std via feature flags.
//! - **Zero Dependencies**: In its default configuration, it has no external dependencies.
//!
//! ## Feature Flags
//! - `multi-threaded` (default): Uses `std::sync` primitives. Suitable for most server and desktop applications.
//! - `single-threaded`: Optimized for WASM or single-threaded environments. Uses `Rc` and `RefCell` to avoid atomic overhead.
//! - `tokio`: Integrates with the Tokio runtime, using `tokio::sync::RwLock` for async operations.
//! - `async-std`: Integrates with the async-std runtime.
//!
//! ## Quick Start
//!
//! ```rust
//! use rustand::Store;
//!
//! # tokio_test::block_on(async {
//! let store = Store::new(0);
//!
<<<<<<< Updated upstream
//! // Subscribe to changes
=======
>>>>>>> Stashed changes
//! let _sub = store.subscribe(|v| println!("New value: {}", v)).await.unwrap();
//!
//! // Update state
//! store.set(|s| *s += 1).await.unwrap();
<<<<<<< Updated upstream
//!
//! // Get state
//! assert_eq!(store.get().await.unwrap(), 1);
=======
>>>>>>> Stashed changes
//! # });
//! ```
//!
//! ## Ecosystem Integrations (Planned)
//! - `rustand-slint`
//! - `rustand-tauri`
//! - `rustand-leptos`
//! - `rustand-egui`
//! - `rustand-redox`

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

// Ensure features' mutual exclusion
#[cfg(any(
    all(feature = "multi-threaded", feature = "single-threaded"),
    all(feature = "multi-threaded", feature = "tokio"),
    all(feature = "multi-threaded", feature = "async-std"),
    all(feature = "single-threaded", feature = "tokio"),
    all(feature = "single-threaded", feature = "async-std"),
    all(feature = "tokio", feature = "async-std")
))]
compile_error!(
    "Features \"multi-threaded\", \"single-threaded\", \"tokio\", and \"async-std\" are mutually exclusive."
);

// Ensure that at least one feature is enabled
#[cfg(not(any(
    feature = "multi-threaded",
    feature = "single-threaded",
    feature = "tokio",
    feature = "async-std"
)))]
compile_error!(
    "One of the following features must be enabled: \"multi-threaded\", \"single-threaded\", \"tokio\", or \"async-std\"."
);

#[cfg(feature = "async-std")]
mod async_std_runtime;

#[cfg(all(
    feature = "multi-threaded",
    not(feature = "tokio"),
    not(feature = "async-std")
))]
mod multi_threaded;

#[cfg(feature = "single-threaded")]
mod single_threaded;

#[cfg(feature = "tokio")]
mod tokio_runtime;

#[cfg(not(feature = "single-threaded"))]
mod common;
mod error;
mod store;
mod subscription;

pub use error::StoreError;
pub use store::Store;
pub use subscription::{SubscriberCallback, SubscriberId, Subscription};
