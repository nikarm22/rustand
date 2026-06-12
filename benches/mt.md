# Multithreaded Benchmark Results

Generated on 2026-06-12.
Each test ran for 5 seconds. Uses `black_box` to prevent compiler optimizations.

## Performance Tests

| Scenario | Writer Threads | Reader Threads | Reads/sec | Writes/sec |
| :--- | :---: | :---: | :---: | :---: |
| **1. Read-Only** | 0 | 8 | 6,439,188 | 0 |
| **2. Read-Heavy** | 1 | 7 | 6,447,618 | 177,481 |
| **3. Contested** | 4 | 4 | 3,393,809 | 930,400 |
| **4. Write-Heavy** | 7 | 1 | 1,209,604 | 1,480,828 |
| **5. Write-Only** | 8 | 0 | 0 | 1,418,828 |

## Latency & Subscription Tests

| Scenario | Writer Threads | Subscribers | Writes/sec | Avg Latency (Dispatch) |
| :--- | :---: | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 8 | 1 | 1,493,555 | 5,212 ns |
| **2. Moderate Subscriptions** | 8 | 50 | 1,561,870 | 4,982 ns |
| **3. High Subscriptions** | 8 | 1000 | 1,243,191 | 6,288 ns |

---
*Note: Latency measures the dispatch time from the producer thread to the background worker.*
