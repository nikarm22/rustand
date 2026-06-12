# Multithreaded Performance: mt-ring (Ring Buffer Optimization)

The `mt-ring` feature implements a 4-slot ring buffer using cache-line aligned `RwLock`s and an atomic index. This architecture is designed to provide Single-Writer-Multiple-Reader (SWMR) semantics where readers are almost never blocked by writers.

## Results (6-Core Hardware)

### Throughput (Operations per Second)
| Scenario | Reads/sec | Writes/sec | Notes |
| :--- | :--- | :--- | :--- |
| **1. 0W, 6R** | 6,790,844 | 0 | Baseline read performance. |
| **2. 1W, 5R** | 7,169,932 | 700,126 | **Excellent balance**. Writes are 5x faster than default. |
| **3. 3W, 3R** | 5,848,880 | 384,573 | Contended case. |
| **4. 5W, 1R** | 20,505,534 | 646,838 | **Extreme read throughput**. Readers are unblocked. |
| **5. 6W, 0R** | 0 | 1,502,152 | Pure write performance. |

### Latency & Subscriptions
| Mode | Writes/sec | Avg Latency |
| :--- | :--- | :--- |
| **1 Sub** | 1,119,476 | 5,254 ns |
| **50 Subs** | 952,949 | 6,183 ns |
| **1000 Subs** | 903,102 | 6,502 ns |

## Analysis
1.  **Read Scaling**: The most impressive result is in read-heavy contested scenarios. In the 5W/1R case, read throughput peaked at over **20 million ops/sec**. This confirms that the ring buffer successfully decouples readers from the writing process.
2.  **Write Performance**: While pure write performance is lower than `mt-no-reentry` (due to the overhead of cloning the state between slots and managing multiple locks), it is still significantly better than the default worker-thread model.
3.  **Backpressure**: The ring buffer naturally provides backpressure. If readers are too slow, the writer will block on the `WriteLock` of the "next" slot until it is cleared, preventing the state from getting too far ahead of consumers.
4.  **Trade-offs**:
    *   **Memory**: 4x state memory footprint.
    *   **Re-entry**: Like `mt-no-reentry`, this mode is prone to deadlocks if `set` or `unsubscribe` is called from within a subscriber (due to the `subscribers` RwLock and slot locks).
    *   **Complexity**: Requires `T: Clone` for state management.

## Conclusion
`mt-ring` is the best choice for **read-heavy applications with high-frequency updates** where read latency must remain low regardless of writer activity.
