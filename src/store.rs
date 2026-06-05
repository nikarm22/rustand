#[cfg(feature = "single-threaded")]
pub use crate::single_threaded::store::Store;

#[cfg(all(
    feature = "multi-threaded",
    not(feature = "tokio"),
    not(feature = "async-std")
))]
pub use crate::multi_threaded::store::Store;

#[cfg(feature = "tokio")]
pub use crate::tokio_runtime::store::Store;

#[cfg(feature = "async-std")]
pub use crate::async_std_runtime::store::Store;
