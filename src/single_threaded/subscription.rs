use crate::single_threaded::inner::InnerStore;
use std::rc::{Rc as Arc, Weak};

/// Unique identifier for a subscriber.
pub type SubscriberId = usize;
/// Callback function type for subscribers.
pub type SubscriberCallback<T> = Arc<dyn Fn(&T) + 'static>;

/// A handle to an active subscription.
///
/// When this handle is dropped, the subscriber is automatically removed
/// from the store. This ensures that subscribers don't outlive their
/// intended scope and prevents memory leaks.
///
/// `Subscription` can be cloned, but note that dropping any clone will
/// unsubscribe the callback, as all clones share the same subscriber ID.
#[derive(Clone)]
pub struct Subscription<T: 'static> {
    pub(crate) store: Weak<InnerStore<T>>,
    pub(crate) id: SubscriberId,
}

impl<T: 'static> PartialEq for Subscription<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && Weak::ptr_eq(&self.store, &other.store)
    }
}

impl<T: 'static> Eq for Subscription<T> {}

impl<T: 'static> std::fmt::Debug for Subscription<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<T: 'static> Drop for Subscription<T> {
    fn drop(&mut self) {
        if let Some(store) = self.store.upgrade() {
            store.unsubscribe(self.id);
        }
    }
}
