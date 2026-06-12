use rustand::Store;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_subscription_clone_behavior() {
    let store = Store::new(0);
    let called = Arc::new(Mutex::new(0));

    let sub = store
        .subscribe({
            let called = called.clone();
            move |v| {
                let mut c = called.lock().unwrap();
                *c = *v;
            }
        })
        .unwrap();

    let sub_clone = sub.clone();

    // Drop the clone
    drop(sub_clone);

    // This should have unsubscribed the original too because they share the same ID
    store.set(|s| *s = 42).unwrap();

    // Wait a bit
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(
        *called.lock().unwrap(),
        0,
        "Subscription should have been removed when clone was dropped"
    );
}

#[tokio::test]
#[cfg(not(any(
    feature = "mt-no-reentry",
    feature = "mt-ring",
    feature = "mt-ring-unsafe"
)))]
async fn test_drop_subscription_in_callback() {
    let store = Store::new(0);
    let called_count = Arc::new(Mutex::new(0));

    // We need a way to drop the subscription from inside the callback.
    // Since Subscription doesn't have a way to 'un-drop' itself, we can use an Option.
    let sub_holder = Arc::new(Mutex::new(None));

    let sub = store
        .subscribe({
            let called_count = called_count.clone();
            let sub_holder = sub_holder.clone();
            move |_v| {
                let mut c = called_count.lock().unwrap();
                *c += 1;
                // Drop the subscription on the first call
                let _ = sub_holder.lock().unwrap().take();
            }
        })
        .unwrap();

    *sub_holder.lock().unwrap() = Some(sub);

    // First update should trigger callback and drop subscription
    store.set(|s| *s = 1).unwrap();

    // Give it time to process
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert_eq!(*called_count.lock().unwrap(), 1);

    // Second update should NOT trigger callback
    store.set(|s| *s = 2).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert_eq!(
        *called_count.lock().unwrap(),
        1,
        "Callback should not have been called after it dropped its own subscription"
    );
}

#[tokio::test]
async fn test_high_concurrency_multi_threaded() {
    let store = Store::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let store = store.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                store.set(|s| *s += 1).unwrap();
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(store.get().unwrap(), 1000);
}

#[tokio::test]
async fn test_error_display() {
    let err = rustand::StoreError::Poisoned;
    assert_eq!(format!("{}", err), "Lock poisoned");
}

#[tokio::test]
async fn test_explicit_unsubscribe() {
    let store = Store::new(0);
    let called = Arc::new(Mutex::new(false));

    let _sub = store
        .subscribe({
            let called = called.clone();
            move |_| {
                let mut c = called.lock().unwrap();
                *c = true;
            }
        })
        .unwrap();

    let id = 0;

    store.unsubscribe(id).unwrap();

    store.set(|s| *s = 1).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // It should NOT have been called
    assert!(!*called.lock().unwrap());
}
