#[cfg(not(feature = "single-threaded"))]
use rustand::Store;
#[cfg(not(feature = "single-threaded"))]
use std::time::Duration;

#[cfg(not(feature = "single-threaded"))]
#[tokio::main]
async fn main() {
    // Create a store with a vector of strings
    let store = Store::new(vec!["Initial".to_string()]);

    // Subscribe to changes
    let _sub = store
        .subscribe(|list| {
            println!("Subscriber: List updated, size is now {}", list.len());
        })
        .await
        .unwrap();

    // Perform an async update
    tokio::spawn({
        let store = store.clone();
        async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            store
                .set(|s| s.push("From Task 1".to_string()))
                .await
                .unwrap();
        }
    });

    // Perform a synchronous update using the sync API
    // This is useful if you are calling from a non-async context
    // or just want to fire-and-forget.
    store
        .set_sync(|s| s.push("From Sync Call".to_string()))
        .unwrap();

    // Wait a bit to let the async task finish
    tokio::time::sleep(Duration::from_millis(200)).await;

    let final_state = store.get().await.unwrap();
    println!("Final State: {:?}", final_state);
}

#[cfg(feature = "single-threaded")]
fn main() {}
