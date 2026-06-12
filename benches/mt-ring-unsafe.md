# Multithreaded Performance: mt-ring-unsafe (YOLO Mode)

The `mt-ring-unsafe` feature is the ultimate optimization of the `rustand` multithreaded store. It implements a 4-slot ring buffer using `UnsafeCell` and raw atomic counters, completely bypassing the overhead of `RwLock`.

## Results (6-Core Hardware)

### Throughput (Operations per Second)
| Scenario | Reads/sec | Writes/sec | Notes |
| :--- | :--- | :--- | :--- |
| **1. 0W, 6R** | 13,614,516 | 0 | 2x faster than `mt-ring`. |
| **2. 1W, 5R** | 12,296,025 | 1,472,801 | **Mind-boggling**. 2x faster reads and 2x faster writes than `mt-ring`. |
| **3. 3W, 3R** | 10,157,960 | 543,619 | High contention, still maintaining 10M+ reads. |
| **4. 5W, 1R** | 31,719,681 | 713,163 | **The Peak**. Reads hit over 31 million ops/sec. |
| **5. 6W, 0R** | 0 | 1,223,237 | Pure write performance (limited by writer mutex). |

## Why it's so fast:
1.  **No Lock Metadata Contention**: By replacing `RwLock` with raw atomic increments on per-slot counters, we eliminate the "cache line bouncing" that happens when multiple threads fight for the same lock object.
2.  **Zero-Overhead Read Path**: A read is now just two atomic operations (increment/decrement) and a clone. There are no syscalls or complex lock state machines.
3.  **Maximum Isolation**: Readers and writers are completely decoupled. A writer only ever touches the reader count of the *next* slot, leaving the *current* slot's metadata entirely to the readers.

## Trade-offs (The YOLO Factor):
1.  **Unsafe Code**: Uses `UnsafeCell` and raw atomic synchronization. While architecturally sound, it bypasses Rust's safety checks for state access.
2.  **No-Reentry**: Same as `mt-ring` and `mt-no-reentry`. Recursive updates will deadlock.
3.  **Memory**: 4x state memory footprint.

## Conclusion:
`mt-ring-unsafe` is for scenarios where every nanosecond counts. It provides the highest possible read throughput while maintaining solid write performance, making it the definitive choice for high-frequency trading, real-time telemetry, or extreme-performance UI engines.
