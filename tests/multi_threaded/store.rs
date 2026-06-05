use rustand::Store;
use std::sync::mpsc;
use std::time::Duration;

fn run_with_timeout<F>(timeout: Duration, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        f();
        let _ = tx.send(());
    });

    if rx.recv_timeout(timeout).is_err() {
        panic!("Test timed out after {:?}", timeout);
    }
}

#[tokio::test]
async fn test_store_get_set() {
    let store = Store::new(0);

    // Initial value
    assert_eq!(store.get().await.unwrap(), 0);

    // Update value
    store.set(|s| *s += 1).await.unwrap();
    assert_eq!(store.get().await.unwrap(), 1);

    // Multiple updates
    store.set(|s| *s *= 10).await.unwrap();
    assert_eq!(store.get().await.unwrap(), 10);
}

#[test]
fn test_deadlock_on_get_in_subscriber() {
    run_with_timeout(Duration::from_secs(2), || {
        let store = Store::new(0);

        let _sub = futures_lite::future::block_on(store.subscribe({
            let store = store.clone();
            move |_v| {
                let _ = futures_lite::future::block_on(store.get()).unwrap();
            }
        }))
        .unwrap();

        futures_lite::future::block_on(store.set(|s| *s = 1)).unwrap();
    });
}

#[test]
fn test_deadlock_on_set_in_subscriber() {
    run_with_timeout(Duration::from_secs(2), || {
        let store = Store::new(0);

        let _sub = futures_lite::future::block_on(store.subscribe({
            let store = store.clone();
            move |v| {
                if *v == 1 {
                    futures_lite::future::block_on(store.set(|s| *s = 2)).unwrap();
                }
            }
        }))
        .unwrap();

        futures_lite::future::block_on(store.set(|s| *s = 1)).unwrap();

        // Use a simple polling loop since we don't have async/await here easily in a sync test
        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_secs(1) {
            if futures_lite::future::block_on(store.get()).unwrap() == 2 {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("State did not reach 2 after 1 second");
    });
}

#[tokio::test]
async fn test_thread_safety() {
    let store = Store::new(0);
    let store_clone = store.clone();

    let handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            for _ in 0..100 {
                store_clone.set(|s| *s += 1).await.unwrap();
            }
        });
    });

    for _ in 0..100 {
        let _ = store.set(|s| *s += 1).await;
    }

    handle.join().unwrap();

    assert_eq!(store.get().await.unwrap(), 200);
}
