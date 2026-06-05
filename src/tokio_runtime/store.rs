use crate::tokio_runtime::TokioRuntime;

/// A thread-safe, Zustand-inspired state container.
pub type Store<T> = crate::common::store::Store<T, TokioRuntime>;
