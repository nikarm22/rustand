# Implementation Plan: MPSC Channels for Subscriptions

## Problem Statement
Currently, `rustand` uses a `RwLock<Vec<Subscriber<T>>>` to manage subscriptions in multi-threaded and async contexts (`multi-threaded`, `tokio`, `async-std` features). This causes lock contention during high-frequency state updates and requires taking a read lock every time `set` is called.

## Goal
Transition to using MPSC (Multi-Producer, Single-Consumer) channels for handling state updates and subscription management. This will move the subscriber list into a dedicated background worker, eliminating the need for lock-guarded subscriber access during the hot path of state updates.

## Proposed Changes

### 1. Define `StoreEvent` Enum
A new internal enum will represent actions the background worker needs to perform.
```rust
enum StoreEvent<T> {
    StateChanged(Arc<T>),
    Subscribe(SubscriberId, SubscriberCallback<T>),
    Unsubscribe(SubscriberId),
    Terminate,
}
```

### 2. Update `InnerStore<T>`
Modify `InnerStore` to include the channel sender and remove the `subscribers` lock.
```rust
pub struct InnerStore<T> {
    pub(crate) state: RwLock<T>,
    pub(crate) sender: Sender<StoreEvent<T>>, // New
    pub(crate) next_id: AtomicUsize,
    // Removed: subscribers: SyncRwLock<Vec<Subscriber<T>>>,
}
```

### 3. Background Worker Loop
Each `Store` (or `InnerStore`) will have a background worker responsible for:
- Maintaining the private `Vec<Subscriber<T>>`.
- Receiving `StoreEvent` messages.
- Executing subscriber callbacks on `StateChanged`.
- Adding/Removing subscribers on `Subscribe`/`Unsubscribe`.

#### Runtime-Specific Implementations:
- **Tokio**: Use `tokio::sync::mpsc::unbounded_channel` and `tokio::spawn`.
- **Async-std**: Use `async_std::channel::unbounded` and `async_std::task::spawn`.
- **Multi-threaded (std)**: Use `std::sync::mpsc::channel` and `std::thread::spawn`.

### 4. API Refactoring

#### `Store::new`
- Initialize the MPSC channel.
- Spawn the background worker.
- Return the `Store` containing the `Arc<InnerStore>`.

#### `Store::set` / `Store::set_sync`
- Update the state as before.
- Clone the state into an `Arc`.
- Send `StoreEvent::StateChanged(snapshot)` to the channel.

#### `Store::subscribe`
- Generate a new `SubscriberId`.
- Send `StoreEvent::Subscribe(id, callback)` to the channel.
- Return a `Subscription` handle.

#### `Store::unsubscribe`
- Send `StoreEvent::Unsubscribe(id)` to the channel.

### 5. Termination
The background worker should shut down gracefully when the `Store` and all its clones are dropped.
- In the `Drop` implementation for `InnerStore` (or by using the channel's natural closure), send `StoreEvent::Terminate` or simply allow the receiver to return `None`/`Err` when all senders are dropped.

## Benefits
- **Zero Lock Contention** on the subscriber list during state updates.
- **Improved Performance**: `set` calls return faster as they only need to update the state and send a message.
- **Simplified Concurrency**: The background worker provides a single-threaded environment for managing subscribers, avoiding complex synchronization.

## Implementation Steps
1. Create `src/event.rs` or add `StoreEvent` to `src/inner.rs`.
2. Update `InnerStore` struct definition with feature-gated channel types.
3. Implement the worker spawn logic in `InnerStore::new` (to be called by `Store::new`).
4. Update `Store::set`, `Store::subscribe`, and `Store::unsubscribe` to use the channel.
5. Ensure `single-threaded` feature remains unaffected (it doesn't need channels).
6. Verify with existing tests and add new concurrency tests.
