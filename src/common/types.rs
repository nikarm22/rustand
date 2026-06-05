use std::sync::Arc;

/// Unique identifier for a subscriber.
pub type SubscriberId = usize;

/// Events processed by the background subscriber worker.
pub enum StoreEvent<T, Cb> {
    /// State has changed, notify subscribers with the new snapshot.
    StateChanged(Arc<T>),
    /// A new subscriber has joined.
    Subscribe(SubscriberId, Cb),
    /// A subscriber has left.
    Unsubscribe(SubscriberId),
}
