# Multithreaded Performance Comparison

This document compares the different multithreaded architectures available in `rustand`.

## Modes
1.  **Default**: Worker thread + `mpsc` channel. Handles recursive updates safely.
2.  **No-Reentry**: Direct notification on the writer thread. Faster but deadlocks on recursive updates.
3.  **MT-Ring**: 4-slot ring buffer with cache-line alignment. Optimized for low-latency reads.
4.  **MT-Ring-Unsafe**: Raw atomic ring buffer. Maximum possible performance using `unsafe`.

## Throughput Comparison (6 Threads)
*Measured in Operations per Second. Higher is better.*

| Scenario | Read (Default) | Write (Default) | Read (No-Reentry) | Write (No-Reentry) | Read (MT-Ring) | Write (MT-Ring) | Read (MT-Ring-Unsafe) | Write (MT-Ring-Unsafe) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Balanced (1W, 1R)** | 14,044,428 | 1,942,924 | **25,451,551** | 1,065,164 | 6,497,835 | 1,814,122 | 14,840,681 | **3,004,537** |
| **1. 0W, 6R** | 6,146,669 | 0 | 5,456,738 | 0 | 5,504,798 | 0 | **12,005,946** | 0 |
| **2. 1W, 5R** | 5,840,247 | 131,928 | 5,338,805 | 143,230 | 5,484,729 | 518,570 | **10,838,548** | **1,411,344** |
| **3. 3W, 3R** | 3,719,943 | 974,311 | 3,343,055 | 1,355,618 | 4,839,910 | 325,528 | **9,606,003** | **516,679** |
| **4. 5W, 1R** | 2,413,284 | 1,607,807 | 1,257,854 | 2,435,448 | 17,923,926 | 548,926 | **27,653,815** | **601,723** |
| **5. 6W, 0R** | 0 | 1,709,924 | 0 | **2,717,740** | 0 | 1,292,243 | 0 | 1,064,322 |

## Latency Comparison (6 Writers)
*Measured in nanoseconds (ns). Lower is better.*

| Mode | 1 Subscriber | 50 Subscribers | 1000 Subscribers |
| :--- | :--- | :--- | :--- |
| **Default** | 3,584 ns | 3,326 ns | 3,307 ns |
| **No-Reentry** | **2,433 ns** | **2,652 ns** | **3,580 ns** |
| **MT-Ring** | 6,042 ns | 6,631 ns | 7,615 ns |
| **MT-Ring-Unsafe** | 8,895 ns | 11,562 ns | 30,065 ns |

## Analysis
1.  **Balanced Baseline**: In the 1W/1R case, `mt-no-reentry` achieves the highest read throughput because the lack of lock contention allows the single reader to run almost unhindered. However, `mt-ring-unsafe` delivers the highest write throughput in this scenario.
2.  **Read Dominance**: `mt-ring-unsafe` is the undisputed king of reads under pressure, hitting **27M+ ops/sec** even with 5 active writers.
3.  **Latency Trade-off**: 
    *   **Default** latency remains the most stable because it only measures dispatch.
    *   **MT-Ring-Unsafe** has the highest latency due to backpressure; writers spin-wait for readers to clear slots.
4.  **Selection Guide**:
    *   **Default**: Safest, handles any recursive logic.
    *   **No-Reentry**: Best for simple low-contention scenarios and pure write throughput.
    *   **MT-Ring**: Best balance of safety and read resilience under pressure.
    *   **MT-Ring-Unsafe**: Maximum possible read performance for specialized high-throughput applications.

---
*Benchmarks conducted on 6-core hardware on 2026-06-12.*
