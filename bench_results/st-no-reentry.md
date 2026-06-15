# Single-Threaded (No Re-entry) Benchmark Results

Generated on 2026-06-12.
Feature: `st-no-reentry`

## Performance Tests
Iterations per test: 10,000,000

| Scenario | Total Time | Reads/sec | Writes/sec |
| :--- | :--- | :--- | :--- |
| **1. Read-Only** | 5.83 ms | 1,715,737,914 | - |
| **2. Read-Heavy (7R:1W)** | 79.03 ms | 885,741,890 | 126,534,555 |
| **3. Contested (1W:1R)** | 42.86 ms | 233,323,549 | 233,323,549 |
| **4. Write-Heavy (7W:1R)** | 270.50 ms | 36,968,344 | 258,778,408 |
| **5. Write-Only** | 47.41 ms | - | 210,911,435 |

## Latency & Subscription Tests
Iterations: 100,000

| Scenario | Total Time | Writes/sec | Avg Latency |
| :--- | :--- | :--- | :--- |
| **1. Minimal (1 Sub)** | 8.37 ms | 11,940,372 | 42.16 ns |
| **2. Moderate (50 Subs)** | 15.82 ms | 6,320,369 | 118.81 ns |
| **3. High (1000 Subs)** | 170.01 ms | 588,208 | 1,658.72 ns |
