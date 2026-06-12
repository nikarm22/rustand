#[allow(unsafe_code)]
mod unsafe_ring_impl {
    use super::*;
    use std::cell::UnsafeCell;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    };

    #[repr(align(64))]
    pub struct UnsafeSlot<T> {
        pub state: UnsafeCell<T>,
        pub reader_count: AtomicUsize,
    }

    pub struct InnerStore<T, R: ThreadedRuntime>
    where
        T: Send + Sync + 'static,
    {
        pub slots: [UnsafeSlot<T>; 4],
        pub current_idx: AtomicUsize,
        pub subscribers: std::sync::RwLock<Vec<(SubscriberId, SubscriberCallback<T>)>>,
        pub writer_lock: Mutex<()>,
        pub next_id: AtomicUsize,
        pub _runtime: std::marker::PhantomData<R>,
    }

    unsafe impl<T, R: ThreadedRuntime> Send for InnerStore<T, R> where T: Send + Sync + 'static {}
    unsafe impl<T, R: ThreadedRuntime> Sync for InnerStore<T, R> where T: Send + Sync + 'static {}

    impl<T, R: ThreadedRuntime> std::panic::UnwindSafe for InnerStore<T, R> where T: Send + Sync + 'static {}
    impl<T, R: ThreadedRuntime> std::panic::RefUnwindSafe for InnerStore<T, R> where T: Send + Sync + 'static {}
    impl<T, R: ThreadedRuntime> std::panic::UnwindSafe for Store<T, R> where T: Send + Sync + 'static {}
    impl<T, R: ThreadedRuntime> std::panic::RefUnwindSafe for Store<T, R> where T: Send + Sync + 'static {}

    impl<T, R: ThreadedRuntime> InnerStore<T, R>
    where
        T: Send + Sync + 'static,
    {
        pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), StoreError> {
            let mut subs = self.subscribers.write().map_err(|_| StoreError::Poisoned)?;
            subs.retain(|(sub_id, _)| *sub_id != subscriber_id);
            Ok(())
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
            let inner = Arc::new(InnerStore {
                slots: [
                    UnsafeSlot { state: UnsafeCell::new(initial.clone()), reader_count: AtomicUsize::new(0) },
                    UnsafeSlot { state: UnsafeCell::new(initial.clone()), reader_count: AtomicUsize::new(0) },
                    UnsafeSlot { state: UnsafeCell::new(initial.clone()), reader_count: AtomicUsize::new(0) },
                    UnsafeSlot { state: UnsafeCell::new(initial), reader_count: AtomicUsize::new(0) },
                ],
                current_idx: AtomicUsize::new(0),
                subscribers: std::sync::RwLock::new(vec![]),
                writer_lock: Mutex::new(()),
                next_id: AtomicUsize::new(0),
                _runtime: std::marker::PhantomData,
            });
            Self { inner }
        }

        pub fn get(&self) -> Result<T, StoreError>
        where
            T: Clone,
        {
            // Check for poisoning by attempting to acquire the writer lock momentarily 
            // or checking if it's already poisoned.
            if self.inner.writer_lock.is_poisoned() {
                return Err(StoreError::Poisoned);
            }

            let idx = self.inner.current_idx.load(Ordering::Acquire);
            let slot = &self.inner.slots[idx];

            slot.reader_count.fetch_add(1, Ordering::Acquire);

            // SAFETY: We have incremented reader_count, and the writer will not 
            // overwrite this slot until reader_count is 0.
            let val = unsafe { (*slot.state.get()).clone() };

            slot.reader_count.fetch_sub(1, Ordering::Release);

            Ok(val)
        }

        pub async fn get_async(&self) -> Result<T, StoreError>
        where
            T: Clone,
        {
            self.get()
        }

        pub fn set<F>(&self, update: F) -> Result<(), StoreError>
        where
            F: FnOnce(&mut T) + Send + 'static,
            T: Clone,
        {
            let _writer_guard = self.inner.writer_lock.lock().map_err(|_| StoreError::Poisoned)?;

            // Finalize poisoning check by using a separate atomic flag or just relying on writer_lock
            // In our current case, if writer_lock is poisoned, we return StoreError::Poisoned.

            let current_idx = self.inner.current_idx.load(Ordering::Relaxed);
            let next_idx = (current_idx + 1) % 4;
            let next_slot = &self.inner.slots[next_idx];

            // Wait for readers to finish with the next slot
            while next_slot.reader_count.load(Ordering::Acquire) != 0 {
                std::hint::spin_loop();
            }

            let state_snapshot = unsafe {
                let current_state = &*self.inner.slots[current_idx].state.get();
                let next_state_ptr = next_slot.state.get();

                // Clone from current to next
                *next_state_ptr = current_state.clone();

                // Apply update - if this panics, writer_lock remains poisoned
                update(&mut *next_state_ptr);

                // Publish new index
                self.inner.current_idx.store(next_idx, Ordering::Release);

                Arc::new((*next_state_ptr).clone())
            };

            // Notify subscribers
            {
                let subs = self.inner.subscribers.read().map_err(|_| StoreError::Poisoned)?;
                for (_, cb) in &*subs {
                    cb(&state_snapshot);
                }
            }

            Ok(())
        }

        pub async fn set_async<F>(&self, update: F) -> Result<(), StoreError>
        where
            F: FnOnce(&mut T) + Send + 'static,
            T: Clone,
        {
            self.set(update)
        }

        pub fn subscribe<F>(&self, callback: F) -> Result<Subscription<T, R>, StoreError>
        where
            F: Fn(&T) + Send + Sync + 'static,
        {
            let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
            let cb = Arc::new(callback);

            {
                let mut subs = self.inner.subscribers.write().map_err(|_| StoreError::Poisoned)?;
                subs.push((id, cb));
            }

            Ok(Subscription {
                store: Arc::downgrade(&self.inner),
                id,
            })
        }

        pub async fn subscribe_async<F>(&self, callback: F) -> Result<Subscription<T, R>, StoreError>
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

#[cfg(feature = "mt-ring-unsafe")]
pub use unsafe_ring_impl::{InnerStore, Store};
