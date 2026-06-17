use crate::single_threaded::inner::InnerStore;
use crate::single_threaded::subscription::{SubscriberId, Subscription};
use std::rc::Rc as Arc;

/// A state container optimized for single-threaded usage.
#[derive(Clone)]
pub struct Store<T>
where
    T: 'static,
{
    pub(crate) inner: Arc<InnerStore<T>>,
}

impl<T> Store<T>
where
    T: 'static,
{
    /// Create a new store with initial state
    #[must_use]
    pub fn new(initial: T) -> Self {
        Self {
            inner: Arc::new(InnerStore {
                state: std::cell::RefCell::new(initial),
                subscribers: std::cell::RefCell::new(vec![]),
                next_id: std::cell::Cell::new(0),
                is_updating: std::cell::Cell::new(false),
            }),
        }
    }

    /// Get a clone of the current state.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.inner.state.borrow().clone()
    }

    /// Update the state using a closure.
    ///
    /// # Panics
    ///
    /// Panics if called recursively from within another `set` call or from a subscriber.
    pub fn set<F>(&self, update: F)
    where
        F: FnOnce(&mut T),
    {
        if self.inner.is_updating.get() {
            panic!("Recursive store.set() detected. You cannot update the store from within another update or a subscriber.");
        }

        self.inner.is_updating.set(true);

        // Ensure is_updating is reset even if update or subscribers panic
        struct UpdateGuard<'a> {
            flag: &'a std::cell::Cell<bool>,
        }
        impl Drop for UpdateGuard<'_> {
            fn drop(&mut self) {
                self.flag.set(false);
            }
        }
        let _guard = UpdateGuard {
            flag: &self.inner.is_updating,
        };

        let subscribers_snapshot = {
            let mut state = self.inner.state.borrow_mut();
            update(&mut state);

            let subscribers = self.inner.subscribers.borrow();
            subscribers
                .iter()
                .map(|(id, cb)| (*id, Arc::clone(cb)))
                .collect::<Vec<_>>()
        };

        {
            let state = self.inner.state.borrow();
            for (_, cb) in subscribers_snapshot {
                cb(&*state);
            }
        }
    }

    /// Subscribe to state changes.
    pub fn subscribe<F>(&self, callback: F) -> Subscription<T>
    where
        F: Fn(&T) + 'static,
    {
        let id = self.inner.next_id.get();
        self.inner.next_id.set(id + 1);

        let mut subscribers = self.inner.subscribers.borrow_mut();
        subscribers.push((id, Arc::new(callback)));

        Subscription {
            store: Arc::downgrade(&self.inner),
            id,
        }
    }

    /// Unsubscribe a subscriber by ID.
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) {
        self.inner.unsubscribe(subscriber_id);
    }
}

impl<T> PartialEq for Store<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Eq for Store<T> {}

impl<T> std::fmt::Debug for Store<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store").finish_non_exhaustive()
    }
}
