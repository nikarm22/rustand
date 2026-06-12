use crate::common::runtime::ThreadedRuntime;
use crate::common::subscription::Subscription;
use crate::error::StoreError;
use std::sync::Arc;

#[cfg(feature = "mt-ring-unsafe")]
use crate::common::types::SubscriberId;
#[cfg(not(feature = "mt-ring-unsafe"))]
use crate::common::types::{StoreEvent, SubscriberId};

pub type SubscriberCallback<T> = Arc<dyn for<'a> Fn(&'a T) + Send + Sync + 'static>;

#[cfg(feature = "mt-ring-unsafe")]
include!("store_ring_unsafe.rs");

#[cfg(not(feature = "mt-ring-unsafe"))]
mod standard_impl {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(feature = "mt-ring")]
    #[repr(align(64))]
    pub struct AlignedSlot<T>(pub std::sync::RwLock<T>);

    pub struct InnerStore<T, R: ThreadedRuntime>
    where
        T: Send + Sync + 'static,
    {
        #[cfg(feature = "mt-ring")]
        pub slots: [AlignedSlot<T>; 4],
        #[cfg(feature = "mt-ring")]
        pub current_idx: AtomicUsize,
        #[cfg(feature = "mt-ring")]
        pub subscribers: std::sync::RwLock<Vec<(SubscriberId, SubscriberCallback<T>)>>,
        #[cfg(feature = "mt-ring")]
        pub writer_lock: std::sync::Mutex<()>,

        #[cfg(not(feature = "mt-ring"))]
        pub state: R::Lock<T>,
        #[cfg(all(not(feature = "mt-ring"), not(feature = "mt-no-reentry")))]
        pub sender: R::Sender<StoreEvent<T, SubscriberCallback<T>>>,
        #[cfg(all(not(feature = "mt-ring"), feature = "mt-no-reentry"))]
        pub subscribers: R::Lock<Vec<(SubscriberId, SubscriberCallback<T>)>>,

        pub next_id: AtomicUsize,
        pub _runtime: std::marker::PhantomData<R>,
    }

    impl<T, R: ThreadedRuntime> InnerStore<T, R>
    where
        T: Send + Sync + 'static,
    {
        pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
            #[cfg(feature = "mt-ring")]
            {
                let mut subs = self.subscribers.write().map_err(|_| StoreError::Poisoned)?;
                subs.retain(|(sub_id, _)| *sub_id != subscriber_id);
                Ok(())
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                #[cfg(not(feature = "mt-no-reentry"))]
                {
                    R::send(&self.sender, StoreEvent::Unsubscribe(subscriber_id))
                }
                #[cfg(feature = "mt-no-reentry")]
                {
                    R::write_sync(&self.subscribers, |subs| {
                        subs.retain(|(sub_id, _)| *sub_id != subscriber_id);
                    })
                }
            }
        }
    }

    pub struct Store<T, R: ThreadedRuntime>
    where
        T: Send + Sync + 'static,
    {
        pub inner: Arc<InnerStore<T, R>>,
    }

    impl<T, R: ThreadedRuntime> Clone for Store<T, R>
    where
        T: Send + Sync + 'static,
    {
        fn clone(&self) -> Self {
            Self {
                inner: Arc::clone(&self.inner),
            }
        }
    }

    impl<T, R: ThreadedRuntime> Store<T, R>
    where
        T: Send + Sync + 'static,
    {
        #[must_use]
        pub fn new(initial: T) -> Self
        where
            T: Clone,
        {
            #[cfg(feature = "mt-ring")]
            {
                let inner = Arc::new(InnerStore {
                    slots: [
                        AlignedSlot(std::sync::RwLock::new(initial.clone())),
                        AlignedSlot(std::sync::RwLock::new(initial.clone())),
                        AlignedSlot(std::sync::RwLock::new(initial.clone())),
                        AlignedSlot(std::sync::RwLock::new(initial)),
                    ],
                    current_idx: AtomicUsize::new(0),
                    subscribers: std::sync::RwLock::new(vec![]),
                    writer_lock: std::sync::Mutex::new(()),
                    next_id: AtomicUsize::new(0),
                    _runtime: std::marker::PhantomData,
                });
                Self { inner }
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                #[cfg(not(feature = "mt-no-reentry"))]
                {
                    let (tx, rx) = R::create_channel();

                    let inner = Arc::new(InnerStore {
                        state: R::new_lock(initial),
                        sender: tx,
                        next_id: AtomicUsize::new(0),
                        _runtime: std::marker::PhantomData,
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
                #[cfg(feature = "mt-no-reentry")]
                {
                    let inner = Arc::new(InnerStore {
                        state: R::new_lock(initial),
                        subscribers: R::new_lock(vec![]),
                        next_id: AtomicUsize::new(0),
                        _runtime: std::marker::PhantomData,
                    });

                    Self { inner }
                }
            }
        }

        pub fn get(&self) -> Result<T, StoreError>
        where
            T: Clone,
        {
            #[cfg(feature = "mt-ring")]
            {
                if self.inner.writer_lock.is_poisoned() {
                    return Err(StoreError::Poisoned);
                }
                let idx = self.inner.current_idx.load(Ordering::Acquire);
                let guard = self.inner.slots[idx]
                    .0
                    .read()
                    .map_err(|_| StoreError::Poisoned)?;
                Ok(guard.clone())
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                R::read_sync(&self.inner.state, T::clone)
            }
        }

        pub async fn get_async(&self) -> Result<T, StoreError>
        where
            T: Clone,
        {
            #[cfg(feature = "mt-ring")]
            {
                self.get()
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                R::read(&self.inner.state, T::clone).await
            }
        }

        pub fn set<F>(&self, update: F) -> Result<(), StoreError>
        where
            F: FnOnce(&mut T) + Send + 'static,
            T: Clone,
        {
            #[cfg(feature = "mt-ring")]
            {
                let state_snapshot = {
                    let _writer_guard = self
                        .inner
                        .writer_lock
                        .lock()
                        .map_err(|_| StoreError::Poisoned)?;
                    let current = self.inner.current_idx.load(Ordering::Relaxed);
                    let next = (current + 1) % 4;

                    let mut next_guard = self.inner.slots[next]
                        .0
                        .write()
                        .map_err(|_| StoreError::Poisoned)?;

                    {
                        let current_guard = self.inner.slots[current]
                            .0
                            .read()
                            .map_err(|_| StoreError::Poisoned)?;
                        *next_guard = current_guard.clone();
                    }

                    update(&mut next_guard);

                    self.inner.current_idx.store(next, Ordering::Release);
                    Arc::new(next_guard.clone())
                };

                {
                    let subs = self
                        .inner
                        .subscribers
                        .read()
                        .map_err(|_| StoreError::Poisoned)?;
                    for (_, cb) in &*subs {
                        cb(&state_snapshot);
                    }
                }
                Ok(())
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                let state_snapshot = R::write_sync(&self.inner.state, |state| {
                    update(state);
                    Arc::new(state.clone())
                })?;

                #[cfg(not(feature = "mt-no-reentry"))]
                {
                    R::send(&self.inner.sender, StoreEvent::StateChanged(state_snapshot))
                }
                #[cfg(feature = "mt-no-reentry")]
                {
                    R::read_sync(&self.inner.subscribers, |subs| {
                        for (_, cb) in subs {
                            cb(&state_snapshot);
                        }
                    })
                }
            }
        }

        pub async fn set_async<F>(&self, update: F) -> Result<(), StoreError>
        where
            F: FnOnce(&mut T) + Send + 'static,
            T: Clone,
        {
            #[cfg(feature = "mt-ring")]
            {
                self.set(update)
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                let state_snapshot = R::write(&self.inner.state, |state| {
                    update(state);
                    Arc::new(state.clone())
                })
                .await?;

                #[cfg(not(feature = "mt-no-reentry"))]
                {
                    R::send(&self.inner.sender, StoreEvent::StateChanged(state_snapshot))
                }
                #[cfg(feature = "mt-no-reentry")]
                {
                    R::read(&self.inner.subscribers, |subs| {
                        for (_, cb) in subs {
                            cb(&state_snapshot);
                        }
                    })
                    .await
                }
            }
        }

        pub fn subscribe<F>(&self, callback: F) -> Result<Subscription<T, R>, StoreError>
        where
            F: Fn(&T) + Send + Sync + 'static,
        {
            let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
            let cb = Arc::new(callback);

            #[cfg(feature = "mt-ring")]
            {
                let mut subs = self
                    .inner
                    .subscribers
                    .write()
                    .map_err(|_| StoreError::Poisoned)?;
                subs.push((id, cb));
            }
            #[cfg(not(feature = "mt-ring"))]
            {
                #[cfg(not(feature = "mt-no-reentry"))]
                {
                    R::send(&self.inner.sender, StoreEvent::Subscribe(id, cb))?;
                }
                #[cfg(feature = "mt-no-reentry")]
                {
                    R::write_sync(&self.inner.subscribers, |subs| {
                        subs.push((id, cb));
                    })?;
                }
            }

            Ok(Subscription {
                store: Arc::downgrade(&self.inner),
                id,
            })
        }

        #[allow(clippy::unused_async)]
        pub async fn subscribe_async<F>(
            &self,
            callback: F,
        ) -> Result<Subscription<T, R>, StoreError>
        where
            F: Fn(&T) + Send + Sync + 'static,
        {
            self.subscribe(callback)
        }

        pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
            self.inner.unsubscribe(subscriber_id)
        }
    }
}

#[cfg(not(feature = "mt-ring-unsafe"))]
pub use standard_impl::{InnerStore, Store};
