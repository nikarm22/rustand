# Single-Threaded Benchmark Results

Generated on 2026-06-12.

## Performance Tests
Iterations per test: 10,000,000

| Scenario | Total Time | Reads/sec | Writes/sec |
| :--- | :--- | :--- | :--- |
| **1. Read-Only** | 15.90 ms | 629,032,530 | - |
| **2. Read-Heavy (7R:1W)** | 121.41 ms | 576,539,074 | 82,362,725 |
| **3. Contested (1W:1R)** | 69.82 ms | 143,226,312 | 143,226,312 |
| **4. Write-Heavy (7W:1R)** | 443.98 ms | 22,523,594 | 157,665,164 |
| **5. Write-Only** | 64.01 ms | - | 156,227,237 |

## Latency & Subscription Tests
Iterations: 100,000

| Scenario | Total Time | Writes/sec | Avg Latency |
| :--- | :--- | :--- | :--- |
| **1. Minimal (1 Sub)** | 14.18 ms | 7,052,638 | 102.11 ns |
| **2. Moderate (50 Subs)** | 30.50 ms | 3,279,007 | 259.98 ns |
| **3. High (1000 Subs)** | 453.16 ms | 220,671 | 4,486.53 ns |
