use rustand::Store;
#[cfg(any(feature = "mt-no-reentry", feature = "mt-ring", feature = "mt-ring-unsafe"))]
use std::sync::{Arc, Mutex};

#[tokio::test]
#[cfg(any(feature = "mt-no-reentry", feature = "mt-ring", feature = "mt-ring-unsafe"))]
async fn test_panic_safety_multi_threaded() {
    let store = Store::new(0);
    let called = Arc::new(Mutex::new(false));

    let _sub1 = store
        .subscribe(|_| {
            panic!("Subscriber panicked!");
        })
        .unwrap();

    let _sub2 = store
        .subscribe({
            let called = called.clone();
            move |_| {
                let mut c = called.lock().unwrap();
                *c = true;
            }
        })
        .unwrap();

    // This might panic if run in the same thread, but since it's an async test,
    // it depends on how the runtime handles it.
    // In multi-threaded mode, set is called directly.
    let result = std::panic::catch_unwind(|| {
        store.set(|s| *s = 1).unwrap();
    });

    assert!(
        result.is_err(),
        "Set should have panicked because a subscriber panicked"
    );

    // In the current implementation, sub2 will NOT be called because it comes after sub1
    // and they are called sequentially.
    assert!(
        !*called.lock().unwrap(),
        "Subscriber after panicking one should not have been called"
    );
}

#[test]
fn test_store_poisoning() {
    let store = Store::new(0);

    // Trigger a panic inside set to poison the lock
    let _ = std::thread::spawn({
        let store = store.clone();
        move || {
            let _ = store.set(|_| panic!("Poisoning!"));
        }
    })
    .join();

    // Subsequent calls should return Error::Poisoned
    let result = store.get();
    assert!(matches!(result, Err(rustand::StoreError::Poisoned)));

    let result = store.set(|s| *s = 1);
    assert!(matches!(result, Err(rustand::StoreError::Poisoned)));
}
