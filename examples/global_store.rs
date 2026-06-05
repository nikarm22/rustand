#[cfg(not(feature = "single-threaded"))]
use rustand::Store;
#[cfg(not(feature = "single-threaded"))]
use std::sync::OnceLock;

// Define a global store using OnceLock.
// This allows any part of your application to access the same state.
#[cfg(not(feature = "single-threaded"))]
static GLOBAL_STORE: OnceLock<Store<i32>> = OnceLock::new();

/// Helper function to get a reference to the global store
#[cfg(not(feature = "single-threaded"))]
fn store() -> &'static Store<i32> {
    GLOBAL_STORE.get_or_init(|| Store::new(0))
}

#[cfg(not(feature = "single-threaded"))]
async fn increment_task(name: &str) {
    for _ in 0..5 {
        store().set(|s| *s += 1).await.unwrap();
        println!("Task {} incremented the global store.", name);
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[cfg(not(feature = "single-threaded"))]
#[tokio::main]
async fn main() {
    println!("Initial global value: {}", store().get().await.unwrap());

    // Multiple tasks can easily access the global store without passing it around
    let t1 = tokio::spawn(increment_task("A"));
    let t2 = tokio::spawn(increment_task("B"));

    // We can also subscribe to the global store
    let _sub = store()
        .subscribe(|v| {
            println!("Global Subscriber: value is now {}", v);
        })
        .await
        .unwrap();

    let _ = tokio::join!(t1, t2);

    println!("Final global value: {}", store().get().await.unwrap());
    assert_eq!(store().get().await.unwrap(), 10);
}

#[cfg(feature = "single-threaded")]
fn main() {}
