//! Error types for rustand.

/// Error types returned by [`Store`](crate::Store) operations.
#[derive(Debug)]
pub enum StoreError {
    /// The internal lock was poisoned.
    ///
    /// This occurs if a thread panicked while holding a lock (in `multi-threaded`,
    /// `tokio`, or `async-std` modes) or if a recursive update caused a
    /// `BorrowError` (in `single-threaded` mode).
    Poisoned,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poisoned => write!(f, "Lock poisoned"),
        }
    }
}

impl std::error::Error for StoreError {}
