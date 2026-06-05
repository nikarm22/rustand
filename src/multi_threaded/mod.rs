use crate::common::runtime::{EventBus, RuntimeExecutor, StateLock};
use crate::error::StoreError;
use std::future::Future;
use std::sync::{RwLock, mpsc};

pub mod store;
pub mod subscription;

#[derive(Clone, Copy)]
pub struct MultiThreadedRuntime;

impl StateLock for MultiThreadedRuntime {
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
        let guard = lock.read().map_err(|_| StoreError::Poisoned)?;
        Ok(f(&guard))
    }

    async fn write<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        let mut guard = lock.write().map_err(|_| StoreError::Poisoned)?;
        Ok(f(&mut guard))
    }

    fn read_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&T) -> R + Send,
    {
        let guard = lock.read().map_err(|_| StoreError::Poisoned)?;
        Ok(f(&guard))
    }

    fn write_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send,
    {
        let mut guard = lock.write().map_err(|_| StoreError::Poisoned)?;
        Ok(f(&mut guard))
    }
}

impl EventBus for MultiThreadedRuntime {
    type Sender<T: Send + 'static> = mpsc::Sender<T>;
    type Receiver<T: Send + 'static> = mpsc::Receiver<T>;

    fn create_channel<T: Send + 'static>() -> (Self::Sender<T>, Self::Receiver<T>) {
        mpsc::channel()
    }

    fn send<T: Send + 'static>(sender: &Self::Sender<T>, event: T) -> Result<(), StoreError> {
        sender.send(event).map_err(|_| StoreError::Poisoned)
    }
}

impl RuntimeExecutor for MultiThreadedRuntime {
    fn run_worker<T, F>(receiver: Self::Receiver<T>, mut worker: F)
    where
        T: Send + 'static,
        F: FnMut(T) + Send + 'static,
    {
        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                worker(event);
            }
        });
    }

    fn block_on<F, R>(_f: F) -> R
    where
        F: Future<Output = R> + Send,
    {
        panic!("block_on not implemented for MultiThreadedRuntime");
    }
}
