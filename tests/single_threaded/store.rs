use rustand::Store;

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
#[cfg(not(feature = "st-no-reentry"))]
async fn test_deadlock_on_get_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |_v| {
                let _ = store.get().unwrap();
            }
        })
        .unwrap();

    store.set(|s| *s = 1).unwrap();
}

#[tokio::test]
#[cfg(not(feature = "st-no-reentry"))]
async fn test_deadlock_on_set_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    store.set(|s| *s = 2).unwrap();
                }
            }
        })
        .unwrap();

    store.set(|s| *s = 1).unwrap();
    assert_eq!(store.get().unwrap(), 2);
}

#[test]
#[cfg(not(feature = "st-no-reentry"))]
fn test_recursive_access_during_update_single() {
    let store = Store::new(0);

    let result = store.set(|_s| {
        // Trying to get state while it's being updated should fail in single-threaded mode
        // because of RefCell's borrow rules.
        let _ = store.get().unwrap_err();
    });

    assert!(result.is_ok());
}
