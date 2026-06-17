use rustand::Store;

#[test]
fn test_store_get_set() {
    let store = Store::new(0);

    // Initial value
    assert_eq!(store.get(), 0);

    // Update value
    store.set(|s| *s += 1);
    assert_eq!(store.get(), 1);
}

#[test]
fn test_deadlock_on_get_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |_v| {
                let _ = store.get();
            }
        });

    store.set(|s| *s = 1);
}

#[test]
#[should_panic(expected = "Recursive store.set() detected")]
fn test_deadlock_on_set_in_subscriber_single() {
    let store = Store::new(0);

    let _sub = store
        .subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    store.set(|s| *s = 2);
                }
            }
        });

    store.set(|s| *s = 1);
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn test_recursive_access_during_update_single() {
    let store = Store::new(0);

    store.set(|_s| {
        // Trying to get state while it's being updated should panic in single-threaded mode
        // because of RefCell's borrow rules.
        let _ = store.get();
    });
}
