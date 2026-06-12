# Single-Threaded Benchmark Results

Generated on 2026-06-12.
Using the `single-threaded` feature (optimized for non-thread-safe environments using `Rc` and `RefCell`).

## Performance Tests
*Iterations per test: 10,000,000*

| Scenario | Reads/sec | Writes/sec | Total Time |
| :--- | :---: | :---: | :---: |
| **1. Read-Only** | 1,516,601,248 | 0 | 6.59 ms |
| **2. Read-Heavy (7R:1W)** | 629,524,018 | 89,932,002 | 111.20 ms |
| **3. Contested (1W:1R)** | 151,485,774 | 151,485,774 | 66.01 ms |
| **4. Write-Heavy (7W:1R)** | 23,098,133 | 161,686,935 | 432.94 ms |
| **5. Write-Only** | 0 | 158,878,478 | 62.94 ms |

## Latency & Subscription Tests
*Subscribers are executed synchronously on the same thread.*

| Scenario | Subscribers | Writes/sec | Avg Latency |
| :--- | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 1 | 6,836,057 | 106.11 ns |
| **2. Moderate Subscriptions** | 50 | 3,662,062 | 232.17 ns |
| **3. High Subscriptions** | 1000 | 258,201 | 3,832.00 ns |

---
*Note: These benchmarks measure the raw performance of the single-threaded store implementation with re-entrancy protection (subscriber snapshotting).*
