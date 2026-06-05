use crate::common::runtime::{EventBus, RuntimeExecutor, StateLock};
use crate::error::StoreError;
use std::future::Future;
use tokio::sync::{RwLock, mpsc};

pub mod store;
pub mod subscription;

#[derive(Clone, Copy)]
pub struct TokioRuntime;

impl StateLock for TokioRuntime {
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
        let result = std::thread::scope(|s| {
            s.spawn(|| {
                let guard = lock.blocking_read();
                f(&guard)
            })
            .join()
            .map_err(|_| StoreError::Poisoned)
        })?;
        Ok(result)
    }

    fn write_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        let result = std::thread::scope(|s| {
            s.spawn(|| {
                let mut guard = lock.blocking_write();
                f(&mut guard)
            })
            .join()
            .map_err(|_| StoreError::Poisoned)
        })?;
        Ok(result)
    }
}

impl EventBus for TokioRuntime {
    type Sender<T: Send + 'static> = mpsc::UnboundedSender<T>;
    type Receiver<T: Send + 'static> = mpsc::UnboundedReceiver<T>;

    fn create_channel<T: Send + 'static>() -> (Self::Sender<T>, Self::Receiver<T>) {
        mpsc::unbounded_channel()
    }

    fn send<T: Send + 'static>(sender: &Self::Sender<T>, event: T) -> Result<(), StoreError> {
        sender.send(event).map_err(|_| StoreError::Poisoned)
    }
}

impl RuntimeExecutor for TokioRuntime {
    fn run_worker<T, F>(mut receiver: Self::Receiver<T>, mut worker: F)
    where
        T: Send + 'static,
        F: FnMut(T) + Send + 'static,
    {
        std::thread::spawn(move || {
            futures_lite::future::block_on(async move {
                while let Some(event) = receiver.recv().await {
                    worker(event);
                }
            });
        });
    }

    fn block_on<F, R>(f: F) -> R
    where
        F: Future<Output = R> + Send,
    {
        futures_lite::future::block_on(f)
    }
}
