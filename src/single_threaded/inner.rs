use crate::single_threaded::subscription::{SubscriberCallback, SubscriberId};
use std::cell::{Cell, RefCell};

/// Internal store state management.
pub struct InnerStore<T>
where
    T: 'static,
{
    pub(crate) state: RefCell<T>,
    pub(crate) subscribers: RefCell<Vec<(SubscriberId, SubscriberCallback<T>)>>,
    pub(crate) next_id: Cell<SubscriberId>,
    pub(crate) is_updating: Cell<bool>,
}

impl<T> InnerStore<T>
where
    T: 'static,
{
    /// Unsubscribe a subscriber by ID.
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) {
        let mut subs = self.subscribers.borrow_mut();
        subs.retain(|(sub_id, _)| *sub_id != subscriber_id);
    }
}
