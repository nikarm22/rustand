use rustand::Store;
use std::time::Instant;

fn main() {
    let store = Store::new(0);
    let iterations = 1_000_000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = store.set_sync(|s| *s += 1);
    }
    let duration = start.elapsed();

    println!("Time for {} updates: {:?}", iterations, duration);
    println!("Average time per update: {:?}", duration / iterations);
}
