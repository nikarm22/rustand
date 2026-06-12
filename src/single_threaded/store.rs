use crate::error::StoreError;
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
            }),
        }
    }

    /// Get a clone of the current state.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the state is currently being updated.
    #[allow(clippy::future_not_send, clippy::unused_async)]
    pub async fn get(&self) -> Result<T, StoreError>
    where
        T: Clone,
    {
        self.get_sync()
    }

    /// Synchronous version of [`Store::get`].
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the state is currently being updated.
    pub fn get_sync(&self) -> Result<T, StoreError>
    where
        T: Clone,
    {
        #[cfg(feature = "st-no-reentry")]
        {
            Ok(self.inner.state.borrow().clone())
        }
        #[cfg(not(feature = "st-no-reentry"))]
        {
            let state = self
                .inner
                .state
                .try_borrow()
                .map_err(|_| StoreError::Poisoned)?;
            Ok(state.clone())
        }
    }

    /// Update the state using a closure.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the state is currently being accessed.
    #[allow(clippy::future_not_send, clippy::unused_async)]
    pub async fn set<F>(&self, update: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut T),
        T: Clone,
    {
        self.set_sync(update)
    }

    /// Synchronous version of [`Store::set`].
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the state is currently being accessed.
    pub fn set_sync<F>(&self, update: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut T),
        T: Clone,
    {
        #[cfg(feature = "st-no-reentry")]
        {
            {
                let mut state = self.inner.state.borrow_mut();
                update(&mut state);
            }
            let state = self.inner.state.borrow();
            let subscribers = self.inner.subscribers.borrow();
            for (_, cb) in subscribers.iter() {
                cb(&*state);
            }
            Ok(())
        }
        #[cfg(not(feature = "st-no-reentry"))]
        {
            let (state_snapshot, subscribers_snapshot) = {
                let mut state = self
                    .inner
                    .state
                    .try_borrow_mut()
                    .map_err(|_| StoreError::Poisoned)?;
                update(&mut state);
                let state_snapshot = state.clone();

                let subscribers = self
                    .inner
                    .subscribers
                    .try_borrow()
                    .map_err(|_| StoreError::Poisoned)?;

                let subscribers_snapshot: Vec<_> = subscribers
                    .iter()
                    .map(|(id, cb)| (*id, Arc::clone(cb)))
                    .collect();

                (state_snapshot, subscribers_snapshot)
            };

            for (_, cb) in subscribers_snapshot {
                cb(&state_snapshot);
            }
            Ok(())
        }
    }

    /// Subscribe to state changes.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the subscriber list is currently being accessed.
    #[allow(clippy::future_not_send, clippy::unused_async)]
    pub async fn subscribe<F>(&self, callback: F) -> Result<Subscription<T>, StoreError>
    where
        F: Fn(&T) + 'static,
    {
        self.subscribe_sync(callback)
    }

    /// Synchronous version of [`Store::subscribe`].
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the subscriber list is currently being accessed.
    pub fn subscribe_sync<F>(&self, callback: F) -> Result<Subscription<T>, StoreError>
    where
        F: Fn(&T) + 'static,
    {
        let id = self.inner.next_id.get();
        self.inner.next_id.set(id + 1);

        #[cfg(feature = "st-no-reentry")]
        {
            self.inner.subscribers.borrow_mut().push((id, Arc::new(callback)));
        }
        #[cfg(not(feature = "st-no-reentry"))]
        {
            let mut subscribers = self
                .inner
                .subscribers
                .try_borrow_mut()
                .map_err(|_| StoreError::Poisoned)?;
            subscribers.push((id, Arc::new(callback)));
        }
        Ok(Subscription {
            store: Arc::downgrade(&self.inner),
            id,
        })
    }

    /// Unsubscribe a subscriber by ID.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Poisoned`] if the subscriber list is currently being accessed.
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
        self.inner.unsubscribe(subscriber_id)
    }
}
