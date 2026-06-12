// To run this example:
// cargo run --example single_threaded_usage --no-default-features --features single-threaded

#[cfg(feature = "single-threaded")]
use rustand::Store;

#[cfg(feature = "single-threaded")]
fn main() {
    println!("Running single-threaded example...");

    // In single-threaded mode, we use Rc and RefCell internally.
    // This is much faster as it avoids atomic operations.
    let store = Store::new(0);

    // Subscriptions work the same way
    let _sub = store
        .subscribe(|v| println!("Value changed to: {}", v))
        .unwrap();

    for i in 1..=5 {
        store.set(|s| *s = i).unwrap();
    }

    assert_eq!(store.get().unwrap(), 5);
}

#[cfg(not(feature = "single-threaded"))]
fn main() {}
