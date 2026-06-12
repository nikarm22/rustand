# 6-Core Multithreaded Benchmark Results

Generated on 2026-06-12.
Optimized for 6-core hardware to minimize context switching overhead.

## Performance Tests (6 Threads)

| Scenario | Writer Threads | Reader Threads | Reads/sec | Writes/sec |
| :--- | :---: | :---: | :---: | :---: |
| **1. 0W, 6R** | 0 | 6 | 6,795,124 | 0 |
| **2. 1W, 5R** | 1 | 5 | 6,034,822 | 142,419 |
| **3. 3W, 3R** | 3 | 3 | 4,010,070 | 959,121 |
| **4. 5W, 1R** | 5 | 1 | 2,345,930 | 1,582,586 |
| **5. 6W, 0R** | 6 | 0 | 0 | 1,718,263 |

## Latency & Subscription Tests (6 Writers)

| Scenario | Writer Threads | Subscribers | Writes/sec | Avg Latency |
| :--- | :---: | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 6 | 1 | 1,620,763 | 3,565 ns |
| **2. Moderate Subscriptions** | 6 | 50 | 1,717,829 | 3,357 ns |
| **3. High Subscriptions** | 6 | 1000 | 1,746,568 | 3,295 ns |

## Analysis of the Performance Gap
The drop from **~1.5B reads/sec (ST)** to **~6.8M reads/sec (MT)** is primarily due to three factors:

1.  **Atomic Overhead & Memory Barriers**: In `multi-threaded` mode, every `get` and `set` must acquire an `RwLock`. Unlike `RefCell` (which uses simple counter checks), `RwLock` uses atomic operations and memory barriers to ensure visibility across CPU caches. This is inherently orders of magnitude slower than non-atomic operations.
2.  **Notification Pipeline**: Every `set` in MT mode performs an `mpsc::send` to a background worker thread. This cross-thread communication involves synchronization and potential thread wake-ups.
3.  **Context Switching**: When running 8 threads on a 6-core machine (previous MT results), performance dropped significantly (e.g., Write-Only fell from **1.7M** to **0.8M**). The 6-core tests above show better scaling by aligning with the hardware.

---
*Results are based on a 5-second sampling per scenario on 6-core hardware.*
