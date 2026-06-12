use rustand::Store;
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
async fn test_store_get_set() {
    let store = Store::new(0);

    // Initial value
    assert_eq!(store.get().unwrap(), 0);

    // Update value
    store.set(|s| *s += 1).unwrap();
    assert_eq!(store.get().unwrap(), 1);
}

#[tokio::test]
async fn test_deadlock_on_get_in_subscriber_tokio() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |_v| {
                let store = store.clone();
                std::thread::spawn(move || {
                    let _ = store.get().unwrap();
                })
                .join()
                .unwrap();
            }
        })
        .unwrap();

    store.set(|s| *s = 1).unwrap();
}

#[tokio::test]
async fn test_deadlock_on_set_in_subscriber_tokio() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    let store = store.clone();
                    std::thread::spawn(move || {
                        store.set(|s| *s = 2).unwrap();
                    })
                    .join()
                    .unwrap();
                }
            }
        })
        .unwrap();

    store.set(|s| *s = 1).unwrap();

    assert_eventually(Duration::from_secs(1), || store.get().unwrap() == 2).await;
}

#[tokio::test]
async fn test_tokio_async_get_set() {
    let store = Store::new(0);

    let t1 = tokio::spawn({
        let store = store.clone();
        async move {
            store.set(|s| *s += 1).unwrap();
        }
    });

    let t2 = tokio::spawn({
        let store = store.clone();
        async move {
            store.set(|s| *s += 1).unwrap();
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();

    assert_eq!(store.get().unwrap(), 2);
}

#[tokio::test]
async fn test_tokio_subscribe() {
    let store = Store::new(0);
    let called = std::sync::Arc::new(std::sync::Mutex::new(0));

    let _sub = store
        .subscribe({
            let called = called.clone();
            move |v| {
                let mut c = called.lock().unwrap();
                *c = *v;
            }
        })
        .unwrap();

    store.set(|s| *s = 42).unwrap();

    assert_eventually(Duration::from_secs(1), || *called.lock().unwrap() == 42).await;
}
