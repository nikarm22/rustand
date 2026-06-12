#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use rustand::Store;
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::hint::black_box;
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::sync::Arc;
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::sync::Barrier;
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::thread;
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
use std::time::{Duration, Instant};

#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
const TEST_DURATION: Duration = Duration::from_secs(5);
#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
const NUM_CORES: usize = 6;

#[cfg(not(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
)))]
fn main() {
    println!("mt-bench requires a multi-threaded feature to be enabled.");
}

#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
fn main() {
    println!("Starting Multithreaded Benchmarks...");
    println!("System Cores: {}", NUM_CORES);
    println!("Each test runs for {:?}\n", TEST_DURATION);

    println!("--- Balanced Baseline ---");
    run_perf_test("1W, 1R", 1, 1);

    println!("\n--- {}-Core Performance Tests ---", NUM_CORES);
    run_perf_test("1. 0W, 6R", 0, NUM_CORES);
    run_perf_test("2. 1W, 5R", 1, NUM_CORES - 1);
    run_perf_test("3. 3W, 3R", 3, 3);
    run_perf_test("4. 5W, 1R", NUM_CORES - 1, 1);
    run_perf_test("5. 6W, 0R", NUM_CORES, 0);

    println!("\n--- 6-Core Latency & Subscription Tests ---");
    run_latency_test("1. Minimal Subscriptions (6W, 1 Sub)", 6, 1);
    run_latency_test("2. Moderate Subscriptions (6W, 50 Subs)", 6, 50);
    run_latency_test("3. High Subscriptions (6W, 1000 Subs)", 6, 1000);

    println!("\n--- Original 8-Thread Tests ---");
    run_perf_test("1. Read-Only (0W, 8R)", 0, 8);
    run_perf_test("2. Read-Heavy (1W, 7R)", 1, 7);
    run_perf_test("3. Contested (4W, 4R)", 4, 4);
    run_perf_test("4. Write-Heavy (7W, 1R)", 7, 1);
    run_perf_test("5. Write-Only (8W, 0R)", 8, 0);
}

#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
fn run_perf_test(name: &str, num_writers: usize, num_readers: usize) {
    let store = Store::new(0);
    let running = Arc::new(AtomicBool::new(true));
    let read_count = Arc::new(AtomicU64::new(0));
    let write_count = Arc::new(AtomicU64::new(0));
    let barrier = Arc::new(Barrier::new(num_writers + num_readers + 1));

    let mut threads = vec![];

    for _ in 0..num_writers {
        let store = store.clone();
        let running = running.clone();
        let write_count = write_count.clone();
        let barrier = barrier.clone();
        threads.push(thread::spawn(move || {
            barrier.wait();
            while running.load(Ordering::Relaxed) {
                let _ = black_box(store.set(|s| *s += 1));
                write_count.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for _ in 0..num_readers {
        let store = store.clone();
        let running = running.clone();
        let read_count = read_count.clone();
        let barrier = barrier.clone();
        threads.push(thread::spawn(move || {
            barrier.wait();
            while running.load(Ordering::Relaxed) {
                let _ = black_box(store.get());
                read_count.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    barrier.wait();
    let start = Instant::now();
    thread::sleep(TEST_DURATION);
    running.store(false, Ordering::Relaxed);

    for t in threads {
        t.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let reads = read_count.load(Ordering::Relaxed);
    let writes = write_count.load(Ordering::Relaxed);

    println!("{}:", name);
    println!("  Reads/sec:  {:.2}", reads as f64 / elapsed);
    println!("  Writes/sec: {:.2}", writes as f64 / elapsed);
}

#[cfg(any(
    feature = "multi-threaded",
    feature = "tokio",
    feature = "async-std",
    feature = "mt-ring",
    feature = "mt-ring-unsafe",
    feature = "mt-no-reentry"
))]
fn run_latency_test(name: &str, num_writers: usize, num_subs: usize) {
    let store = Store::new(0);
    let running = Arc::new(AtomicBool::new(true));
    let write_count = Arc::new(AtomicU64::new(0));
    let total_latency_ns = Arc::new(AtomicU64::new(0));
    let barrier = Arc::new(Barrier::new(num_writers + 1));

    // Add subscribers
    let mut _subs = vec![];
    for _ in 0..num_subs {
        _subs.push(store.subscribe(|_| {}).unwrap());
    }

    let mut threads = vec![];

    for _ in 0..num_writers {
        let store = store.clone();
        let running = running.clone();
        let write_count = write_count.clone();
        let total_latency_ns = total_latency_ns.clone();
        let barrier = barrier.clone();
        threads.push(thread::spawn(move || {
            barrier.wait();
            while running.load(Ordering::Relaxed) {
                let start = Instant::now();
                let _ = black_box(store.set(|s| *s += 1));
                let latency = start.elapsed().as_nanos() as u64;
                total_latency_ns.fetch_add(latency, Ordering::Relaxed);
                write_count.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    barrier.wait();
    let start = Instant::now();
    thread::sleep(TEST_DURATION);
    running.store(false, Ordering::Relaxed);

    for t in threads {
        t.join().unwrap();
    }

    let elapsed = start.elapsed().as_secs_f64();
    let writes = write_count.load(Ordering::Relaxed);
    let avg_latency = if writes > 0 {
        total_latency_ns.load(Ordering::Relaxed) as f64 / writes as f64
    } else {
        0.0
    };

    println!("{}:", name);
    println!("  Writes/sec:   {:.2}", writes as f64 / elapsed);
    println!("  Avg Latency:  {:.2} ns", avg_latency);
}
