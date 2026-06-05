use crate::common::runtime::{EventBus, RuntimeExecutor, StateLock};
use crate::error::StoreError;
use async_std::sync::RwLock;
use std::future::Future;

pub mod store;
pub mod subscription;

#[derive(Clone, Copy)]
pub struct AsyncStdRuntime;

impl StateLock for AsyncStdRuntime {
    type Lock<T: Send + Sync + 'static> = RwLock<T>;

    fn new_lock<T: Send + Sync + 'static>(initial: T) -> Self::Lock<T> {
        RwLock::new(initial)
    }

    async fn read<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&T) -> R + Send,
    {
        let guard = lock.read().await;
        Ok(f(&guard))
    }

    async fn write<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        let mut guard = lock.write().await;
        Ok(f(&mut guard))
    }

    fn read_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&T) -> R + Send,
    {
        let guard = async_std::task::block_on(lock.read());
        Ok(f(&guard))
    }

    fn write_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        let mut guard = async_std::task::block_on(lock.write());
        Ok(f(&mut guard))
    }
}

impl EventBus for AsyncStdRuntime {
    type Sender<T: Send + 'static> = async_std::channel::Sender<T>;
    type Receiver<T: Send + 'static> = async_std::channel::Receiver<T>;

    fn create_channel<T: Send + 'static>() -> (Self::Sender<T>, Self::Receiver<T>) {
        async_std::channel::unbounded()
    }

    fn send<T: Send + 'static>(sender: &Self::Sender<T>, event: T) -> Result<(), StoreError> {
        sender.try_send(event).map_err(|_| StoreError::Poisoned)
    }
}

impl RuntimeExecutor for AsyncStdRuntime {
    fn run_worker<T, F>(receiver: Self::Receiver<T>, mut worker: F)
    where
        T: Send + 'static,
        F: FnMut(T) + Send + 'static,
    {
        async_std::task::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                worker(event);
            }
        });
    }

    fn block_on<F, R>(f: F) -> R
    where
        F: Future<Output = R> + Send,
    {
        async_std::task::block_on(f)
    }
}
