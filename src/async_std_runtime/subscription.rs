use crate::async_std_runtime::AsyncStdRuntime;

/// Unique identifier for a subscriber.
pub type SubscriberId = crate::common::types::SubscriberId;
/// Callback function type for subscribers.
pub type SubscriberCallback<T> = crate::common::store::SubscriberCallback<T>;
/// A handle to an active subscription.
pub type Subscription<T> = crate::common::subscription::Subscription<T, AsyncStdRuntime>;
