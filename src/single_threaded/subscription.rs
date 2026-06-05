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
#[derive(Clone, Debug)]
pub struct Subscription<T: 'static> {
    pub(crate) store: Weak<InnerStore<T>>,
    pub(crate) id: SubscriberId,
}

impl<T: 'static> Drop for Subscription<T> {
    fn drop(&mut self) {
        if let Some(store) = self.store.upgrade() {
            let _ = store.unsubscribe(self.id);
        }
    }
}
