use rustand::Store;
use std::hint::black_box;
use std::time::{Duration, Instant};

const ITERATIONS: u64 = 10_000_000;

fn main() {
    println!("Starting Single-Threaded Benchmarks...");
    println!("Iterations per test: {}\n", ITERATIONS);

    println!("--- Performance Tests ---");
    run_read_only();
    run_read_heavy();
    run_contested();
    run_write_heavy();
    run_write_only();

    println!("\n--- Latency & Subscription Tests ---");
    run_latency_test("1. Minimal Subscriptions (1 Sub)", 1);
    run_latency_test("2. Moderate Subscriptions (50 Subs)", 50);
    run_latency_test("3. High Subscriptions (1000 Subs)", 1000);
}

fn run_read_only() {
    let store = Store::new(0);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = black_box(store.get());
    }
    let elapsed = start.elapsed();
    print_results("1. Read-Only", ITERATIONS, 0, elapsed);
}

fn run_read_heavy() {
    let store = Store::new(0);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for _ in 0..7 {
            let _ = black_box(store.get());
        }
        let _ = black_box(store.set(|s| *s += 1));
    }
    let elapsed = start.elapsed();
    print_results("2. Read-Heavy (7R:1W)", ITERATIONS * 7, ITERATIONS, elapsed);
}

fn run_contested() {
    let store = Store::new(0);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = black_box(store.set(|s| *s += 1));
        let _ = black_box(store.get());
    }
    let elapsed = start.elapsed();
    print_results("3. Contested (1W:1R)", ITERATIONS, ITERATIONS, elapsed);
}

fn run_write_heavy() {
    let store = Store::new(0);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for _ in 0..7 {
            let _ = black_box(store.set(|s| *s += 1));
        }
        let _ = black_box(store.get());
    }
    let elapsed = start.elapsed();
    print_results(
        "4. Write-Heavy (7W:1R)",
        ITERATIONS,
        ITERATIONS * 7,
        elapsed,
    );
}

fn run_write_only() {
    let store = Store::new(0);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = black_box(store.set(|s| *s += 1));
    }
    let elapsed = start.elapsed();
    print_results("5. Write-Only", 0, ITERATIONS, elapsed);
}

fn run_latency_test(name: &str, num_subs: usize) {
    let store = Store::new(0);
    let mut _subs = vec![];
    for _ in 0..num_subs {
        _subs.push(store.subscribe(|_| {}).unwrap());
    }

    // Keep iterations consistent to avoid measurement jitter
    let iters = 100_000;

    let mut total_latency = Duration::default();
    let start = Instant::now();
    for _ in 0..iters {
        let write_start = Instant::now();
        let _ = black_box(store.set(|s| *s += 1));
        total_latency += write_start.elapsed();
    }
    let elapsed = start.elapsed();

    println!("{}:", name);
    println!("  Total Time:   {:?}", elapsed);
    println!(
        "  Writes/sec:   {:.2}",
        iters as f64 / elapsed.as_secs_f64()
    );
    println!(
        "  Avg Latency:  {:.2} ns",
        total_latency.as_nanos() as f64 / iters as f64
    );
}

fn print_results(name: &str, reads: u64, writes: u64, elapsed: Duration) {
    let elapsed_secs = elapsed.as_secs_f64();
    println!("{}:", name);
    println!("  Total Time: {:?}", elapsed);
    if reads > 0 {
        println!("  Reads/sec:  {:.2}", reads as f64 / elapsed_secs);
    }
    if writes > 0 {
        println!("  Writes/sec: {:.2}", writes as f64 / elapsed_secs);
    }
}
