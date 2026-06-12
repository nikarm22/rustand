# Single-Threaded (No Re-entry) Benchmark Results

Generated on 2026-06-12.
Using the `st-no-reentry` feature (high performance, no subscriber snapshotting, no re-entrancy protection).

## Performance Tests
*Iterations per test: 10,000,000*

| Scenario | Reads/sec | Writes/sec | Total Time |
| :--- | :---: | :---: | :---: |
| **1. Read-Only** | 1,661,085,143 | 0 | 6.02 ms |
| **2. Read-Heavy (7R:1W)** | 877,828,263 | 125,404,037 | 79.74 ms |
| **3. Contested (1W:1R)** | 234,201,718 | 234,201,718 | 42.70 ms |
| **4. Write-Heavy (7W:1R)** | 36,352,472 | 254,467,308 | 275.08 ms |
| **5. Write-Only** | 0 | 258,174,095 | 38.73 ms |

## Latency & Subscription Tests
*Subscribers are executed synchronously on the same thread without snapshotting.*

| Scenario | Subscribers | Writes/sec | Avg Latency |
| :--- | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 1 | 10,945,079 | 46.08 ns |
| **2. Moderate Subscriptions** | 50 | 5,898,673 | 126.98 ns |
| **3. High Subscriptions** | 1000 | 570,222 | 1,710.03 ns |

---
*Note: These benchmarks measure the raw performance of the single-threaded store implementation when re-entrancy protection is disabled.*
