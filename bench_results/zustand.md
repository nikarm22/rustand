# Zustand (Node.js) Benchmark Results

Generated on 2026-06-12.

## Performance Tests
Iterations per test: 10,000,000

| Scenario | Total Time | Reads/sec | Writes/sec |
| :--- | :--- | :--- | :--- |
| **1. Read-Only** | 65.43 ms | 152,844,306 | - |
| **2. Read-Heavy (7R:1W)** | 1,148.93 ms | 60,926,508 | 8,703,786 |
| **3. Contested (1W:1R)** | 1,071.80 ms | 9,330,093 | 9,330,093 |
| **4. Write-Heavy (7W:1R)** | 6,596.55 ms | 1,515,945 | 10,611,615 |
| **5. Write-Only** | 998.49 ms | - | 10,015,108 |

## Latency & Subscription Tests
Iterations: 100,000

| Scenario | Total Time | Writes/sec | Avg Latency |
| :--- | :--- | :--- | :--- |
| **1. Minimal (1 Sub)** | 29.00 ms | 3,447,694 | 196.50 ns |
| **2. Moderate (50 Subs)** | 63.93 ms | 1,564,241 | 559.32 ns |
| **3. High (1000 Subs)** | 1,059.35 ms | 94,397 | 9,897.20 ns |
