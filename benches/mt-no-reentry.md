# Multithreaded (No Re-entry) Benchmark Results

Generated on 2026-06-12.
Using the `mt-no-reentry` feature (direct notification, no background worker).
Optimized for 6-core hardware.

## Performance Tests (6 Threads)

| Scenario | Writer Threads | Reader Threads | Reads/sec | Writes/sec |
| :--- | :---: | :---: | :---: | :---: |
| **1. 0W, 6R** | 0 | 6 | 7,382,313 | 0 |
| **2. 1W, 5R** | 1 | 5 | 7,266,054 | 126,351 |
| **3. 3W, 3R** | 3 | 3 | 4,563,459 | 1,580,247 |
| **4. 5W, 1R** | 5 | 1 | 891,808 | 3,256,756 |
| **5. 6W, 0R** | 6 | 0 | 0 | 3,201,591 |

## Latency & Subscription Tests (6 Writers)

| Scenario | Writer Threads | Subscribers | Writes/sec | Avg Latency |
| :--- | :---: | :---: | :---: | :---: |
| **1. Minimal Subscriptions** | 6 | 1 | 3,856,891 | 1,443 ns |
| **2. Moderate Subscriptions** | 6 | 50 | 2,793,118 | 2,015 ns |
| **3. High Subscriptions** | 6 | 1000 | 1,974,760 | 2,912 ns |

---
*Results show a significant throughput increase for writes by eliminating channel synchronization.*
