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
        async_std::task::sleep(Duration::from_millis(10)).await;
    }
    panic!("Assertion failed after {:?}", timeout);
}

#[async_std::test]
async fn test_store_get_set() {
    let store = Store::new(0);

    // Initial value
    assert_eq!(store.get().await.unwrap(), 0);

    // Update value
    store.set(|s| *s += 1).await.unwrap();
    assert_eq!(store.get().await.unwrap(), 1);
}

#[async_std::test]
async fn test_deadlock_on_get_in_subscriber_async_std() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |_v| {
                let _ = store.get_sync().unwrap();
            }
        })
        .await
        .unwrap();

    store.set(|s| *s = 1).await.unwrap();
}

#[async_std::test]
async fn test_deadlock_on_set_in_subscriber_async_std() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    store.set_sync(|s| *s = 2).unwrap();
                }
            }
        })
        .await
        .unwrap();

    store.set(|s| *s = 1).await.unwrap();

    assert_eventually(Duration::from_secs(1), || store.get_sync().unwrap() == 2).await;
}

#[async_std::test]
async fn test_async_std_get_set() {
    let store = Store::new(0);

    let t1 = async_std::task::spawn({
        let store = store.clone();
        async move {
            store.set(|s| *s += 1).await.unwrap();
        }
    });

    let t2 = async_std::task::spawn({
        let store = store.clone();
        async move {
            store.set(|s| *s += 1).await.unwrap();
        }
    });

    t1.await;
    t2.await;

    assert_eq!(store.get().await.unwrap(), 2);
}

#[async_std::test]
async fn test_async_std_subscribe() {
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
        .await
        .unwrap();

    store.set(|s| *s = 42).await.unwrap();

    assert_eventually(Duration::from_secs(1), || *called.lock().unwrap() == 42).await;
}
