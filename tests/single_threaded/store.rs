use rustand::Store;

#[tokio::test]
async fn test_store_get_set() {
    let store = Store::new(0);

    // Initial value
    assert_eq!(store.get().await.unwrap(), 0);

    // Update value
    store.set(|s| *s += 1).await.unwrap();
    assert_eq!(store.get().await.unwrap(), 1);
}

#[tokio::test]
async fn test_deadlock_on_get_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |_v| {
                let _ = futures_lite::future::block_on(store.get()).unwrap();
            }
        })
        .await
        .unwrap();

    store.set(|s| *s = 1).await.unwrap();
}

#[tokio::test]
async fn test_deadlock_on_set_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    futures_lite::future::block_on(store.set(|s| *s = 2)).unwrap();
                }
            }
        })
        .await
        .unwrap();

    store.set(|s| *s = 1).await.unwrap();
    assert_eq!(store.get().await.unwrap(), 2);
}

#[test]
fn test_recursive_access_during_update_single() {
    let store = Store::new(0);

    let result = store.set_sync(|_s| {
        // Trying to get state while it's being updated should fail in single-threaded mode
        // because of RefCell's borrow rules.
        let _ = store.get_sync().unwrap_err();
    });

    assert!(result.is_ok());
}
