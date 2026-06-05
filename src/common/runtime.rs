use crate::error::StoreError;
use std::future::Future;

pub trait StateLock: Send + Sync + 'static {
    type Lock<T: Send + Sync + 'static>: Send + Sync;

    fn new_lock<T: Send + Sync + 'static>(initial: T) -> Self::Lock<T>;

    fn read<T, R, F>(
        lock: &Self::Lock<T>,
        f: F,
    ) -> impl Future<Output = Result<R, StoreError>> + Send
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&T) -> R + Send;

    fn write<T, R, F>(
        lock: &Self::Lock<T>,
        f: F,
    ) -> impl Future<Output = Result<R, StoreError>> + Send
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send;

    fn read_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&T) -> R + Send;

    fn write_sync<T, R, F>(lock: &Self::Lock<T>, f: F) -> Result<R, StoreError>
    where
        T: Send + Sync + 'static,
        R: Send,
        F: FnOnce(&mut T) -> R + Send;
}

pub trait EventBus: Send + Sync + 'static {
    type Sender<T: Send + 'static>: Clone + Send + Sync;
    type Receiver<T: Send + 'static>: Send;

    fn create_channel<T: Send + 'static>() -> (Self::Sender<T>, Self::Receiver<T>);

    fn send<T: Send + 'static>(sender: &Self::Sender<T>, event: T) -> Result<(), StoreError>;
}

pub trait RuntimeExecutor: EventBus {
    fn run_worker<T, F>(receiver: Self::Receiver<T>, worker: F)
    where
        T: Send + 'static,
        F: FnMut(T) + Send + 'static;

    fn block_on<F, R>(f: F) -> R
    where
        F: Future<Output = R> + Send;
}

/// Aggregator trait for threaded runtimes.
pub trait ThreadedRuntime:
    StateLock + EventBus + RuntimeExecutor + Clone + Send + Sync + 'static
{
}

impl<T> ThreadedRuntime for T where
    T: StateLock + EventBus + RuntimeExecutor + Clone + Send + Sync + 'static
{
}
