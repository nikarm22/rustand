use crate::common::runtime::ThreadedRuntime;
use crate::common::subscription::Subscription;
use crate::common::types::{StoreEvent, SubscriberId};
use crate::error::StoreError;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

pub type SubscriberCallback<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;

pub struct InnerStore<T, R: ThreadedRuntime>
where
    T: Send + Sync + 'static,
{
    pub(crate) state: R::Lock<T>,
    pub(crate) sender: R::Sender<StoreEvent<T, SubscriberCallback<T>>>,
    pub(crate) next_id: AtomicUsize,
}

impl<T, R: ThreadedRuntime> InnerStore<T, R>
where
    T: Send + Sync + 'static,
{
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
        R::send(&self.sender, StoreEvent::Unsubscribe(subscriber_id))
    }
}

/// A thread-safe, Zustand-inspired state container.
#[derive(Clone)]
pub struct Store<T, R: ThreadedRuntime>
where
    T: Send + Sync + 'static,
{
    pub(crate) inner: Arc<InnerStore<T, R>>,
}

impl<T, R: ThreadedRuntime> Store<T, R>
where
    T: Send + Sync + 'static,
{
    #[must_use]
    pub fn new(initial: T) -> Self {
        let (tx, rx) = R::create_channel();

        let inner = Arc::new(InnerStore {
            state: R::new_lock(initial),
            sender: tx,
            next_id: AtomicUsize::new(0),
        });

        R::run_worker(rx, {
            let mut subscribers: Vec<(SubscriberId, SubscriberCallback<T>)> = vec![];
            move |event| match event {
                StoreEvent::StateChanged(state) => {
                    for (_, cb) in &subscribers {
                        cb(&state);
                    }
                }
                StoreEvent::Subscribe(id, cb) => {
                    subscribers.push((id, cb));
                }
                StoreEvent::Unsubscribe(id) => {
                    subscribers.retain(|(sub_id, _)| *sub_id != id);
                }
            }
        });

        Self { inner }
    }

    pub async fn get(&self) -> Result<T, StoreError>
    where
        T: Clone,
    {
        R::read(&self.inner.state, T::clone).await
    }

    pub fn get_sync(&self) -> Result<T, StoreError>
    where
        T: Clone,
    {
        R::read_sync(&self.inner.state, T::clone)
    }

    pub async fn set<F>(&self, update: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut T) + Send + 'static,
        T: Clone,
    {
        let state_snapshot = R::write(&self.inner.state, |state| {
            update(state);
            Arc::new(state.clone())
        })
        .await?;

        R::send(&self.inner.sender, StoreEvent::StateChanged(state_snapshot))
    }

    pub fn set_sync<F>(&self, update: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut T) + Send + 'static,
        T: Clone,
    {
        let state_snapshot = R::write_sync(&self.inner.state, |state| {
            update(state);
            Arc::new(state.clone())
        })?;

        R::send(&self.inner.sender, StoreEvent::StateChanged(state_snapshot))
    }

    #[allow(clippy::unused_async)]
    pub async fn subscribe<F>(&self, callback: F) -> Result<Subscription<T, R>, StoreError>
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.subscribe_sync(callback)
    }

    pub fn subscribe_sync<F>(&self, callback: F) -> Result<Subscription<T, R>, StoreError>
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let cb = Arc::new(callback);

        R::send(&self.inner.sender, StoreEvent::Subscribe(id, cb))?;

        Ok(Subscription {
            store: Arc::downgrade(&self.inner),
            id,
        })
    }

    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
        self.inner.unsubscribe(subscriber_id)
    }
}
