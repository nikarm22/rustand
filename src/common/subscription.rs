use crate::common::runtime::ThreadedRuntime;
use crate::common::store::InnerStore;
use crate::common::types::SubscriberId;
use std::sync::Weak;

/// A handle to an active subscription.
pub struct Subscription<T: Send + Sync + 'static, R: ThreadedRuntime> {
    pub(crate) store: Weak<InnerStore<T, R>>,
    pub(crate) id: SubscriberId,
}

impl<T: Send + Sync + 'static, R: ThreadedRuntime> Clone for Subscription<T, R> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            id: self.id,
        }
    }
}

impl<T: Send + Sync + 'static, R: ThreadedRuntime> std::fmt::Debug for Subscription<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<T: Send + Sync + 'static, R: ThreadedRuntime> Drop for Subscription<T, R> {
    fn drop(&mut self) {
        if let Some(store) = self.store.upgrade() {
            let _ = store.unsubscribe(self.id);
        }
    }
}
