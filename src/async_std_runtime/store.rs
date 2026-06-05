use crate::async_std_runtime::AsyncStdRuntime;

/// A thread-safe, Zustand-inspired state container.
pub type Store<T> = crate::common::store::Store<T, AsyncStdRuntime>;
