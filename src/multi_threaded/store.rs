use crate::multi_threaded::MultiThreadedRuntime;

/// A thread-safe, Zustand-inspired state container.
pub type Store<T> = crate::common::store::Store<T, MultiThreadedRuntime>;
