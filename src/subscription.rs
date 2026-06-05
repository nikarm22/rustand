#[cfg(feature = "single-threaded")]
pub use crate::single_threaded::subscription::{SubscriberCallback, SubscriberId, Subscription};

#[cfg(all(
    feature = "multi-threaded",
    not(feature = "tokio"),
    not(feature = "async-std")
))]
pub use crate::multi_threaded::subscription::{SubscriberCallback, SubscriberId, Subscription};

#[cfg(feature = "tokio")]
pub use crate::tokio_runtime::subscription::{SubscriberCallback, SubscriberId, Subscription};

#[cfg(feature = "async-std")]
pub use crate::async_std_runtime::subscription::{SubscriberCallback, SubscriberId, Subscription};
