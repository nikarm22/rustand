use crate::error::StoreError;
use crate::single_threaded::subscription::{SubscriberCallback, SubscriberId};
use std::cell::{Cell, RefCell};

pub type AtomicUsizeType = Cell<usize>;

/// Internal store state management.
pub struct InnerStore<T>
where
    T: 'static,
{
    pub(crate) state: RefCell<T>,
    pub(crate) subscribers: RefCell<Vec<(SubscriberId, SubscriberCallback<T>)>>,
    pub(crate) next_id: AtomicUsizeType,
}

impl<T> InnerStore<T>
where
    T: 'static,
{
    /// Unsubscribe a subscriber by ID.
    #[allow(clippy::unnecessary_wraps)]
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
        let mut subs = self.subscribers.borrow_mut();
        subs.retain(|(sub_id, _)| *sub_id != subscriber_id);
        Ok(())
    }
}
