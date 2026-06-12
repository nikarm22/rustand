use rustand::Store;
use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn assert_eventually<F>(timeout: Duration, f: F)
where
    F: Fn() -> bool,
{
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if f() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("Assertion failed after {:?}", timeout);
}

#[tokio::test]
async fn test_store_subscribe() {
    let store = Store::new(5);
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let _subscription = store
        .subscribe(move |value| {
            let mut called = called_clone.lock().unwrap();
            *called = *value == 10;
        })
        .unwrap();

    store.set(|s| *s = 10).unwrap();

    assert_eventually(Duration::from_secs(1), || *called.lock().unwrap()).await;
}

#[tokio::test]
async fn test_unsubscribe() {
    let store = Store::new(1);
    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let subscription = store
        .subscribe(move |_| {
            let mut called = called_clone.lock().unwrap();
            *called = true;
        })
        .unwrap();

    // Drop subscription to unsubscribe
    drop(subscription);

    store.set(|s| *s += 1).unwrap();

    // Wait a bit to ensure it WASN'T called
    tokio::time::sleep(Duration::from_millis(50)).await;

    // The callback should NOT have been called
    assert!(!*called.lock().unwrap());
}

#[tokio::test]
async fn test_multiple_subscribers() {
    let store = Store::new(0);
    let called1 = Arc::new(Mutex::new(0));
    let called2 = Arc::new(Mutex::new(0));

    let sub1 = {
        let called1 = called1.clone();
        store
            .subscribe(move |v| {
                let mut c = called1.lock().unwrap();
                *c = *v;
            })
            .unwrap()
    };

    let sub2 = {
        let called2 = called2.clone();
        store
            .subscribe(move |v| {
                let mut c = called2.lock().unwrap();
                *c = *v;
            })
            .unwrap()
    };

    store.set(|s| *s = 42).unwrap();

    assert_eventually(Duration::from_secs(1), || {
        *called1.lock().unwrap() == 42 && *called2.lock().unwrap() == 42
    })
    .await;

    // Dropping subscriptions
    drop(sub1);
    drop(sub2);

    // Reset called flags
    *called1.lock().unwrap() = 0;
    *called2.lock().unwrap() = 0;

    store.set(|s| *s = 100).unwrap();

    // Wait a bit
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Callbacks should not be called after unsubscribe
    assert_eq!(*called1.lock().unwrap(), 0);
    assert_eq!(*called2.lock().unwrap(), 0);
}
