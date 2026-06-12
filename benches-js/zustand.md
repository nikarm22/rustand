# Zustand (Node.js) Benchmark Results

Generated on 2026-06-12.
Testing `zustand/vanilla` in Node.js.

## Performance Tests
*Iterations per test: 10,000,000*

| Scenario | Reads/sec | Writes/sec | Total Time |
| :--- | :---: | :---: | :---: |
| **1. Read-Only** | 278,833,923 | 0 | 35.86 ms |
| **2. Read-Heavy (7R:1W)** | 61,599,572 | 8,799,938 | 1.14 s |
| **3. Contested (1W:1R)** | 9,867,733 | 9,867,733 | 1.01 s |
| **4. Write-Heavy (7W:1R)** | 1,472,028 | 10,304,201 | 6.79 s |
| **5. Write-Only** | 0 | 9,995,353 | 1.00 s |

## Latency & Subscription Tests
*Subscribers are executed synchronously on the same thread.*

| Scenario | Subscribers | Writes/sec | Avg Latency |
| :--- | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 1 | 3,623,475 | 188.84 ns |
| **2. Moderate Subscriptions** | 50 | 1,457,914 | 601.17 ns |
| **3. High Subscriptions** | 1000 | 112,945 | 8,764.15 ns |

---
*Note: Benchmarks conducted in Node.js environment using the vanilla Zustand store.*
