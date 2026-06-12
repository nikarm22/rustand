# Performance Comparison: rustand vs zustand

## Throughput (Ops/sec)

| Scenario | ST Read | ST Write | ST-No-Reentry Read | ST-No-Reentry Write | Zustand Read | Zustand Write |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **1. Read-Only** | 1,516,601,248 | 0 | 1,661,085,143 | 0 | 278,833,923 | 0 |
| **2. Read-Heavy** | 629,524,018 | 89,932,002 | 877,828,263 | 125,404,037 | 61,599,572 | 8,799,938 |
| **3. Contested** | 151,485,774 | 151,485,774 | 234,201,718 | 234,201,718 | 9,867,733 | 9,867,733 |
| **4. Write-Heavy** | 23,098,133 | 161,686,935 | 36,352,472 | 254,467,308 | 1,472,028 | 10,304,201 |
| **5. Write-Only** | 0 | 158,878,478 | 0 | 258,174,095 | 0 | 9,995,353 |

*Note: ST-No-Reentry refers to the `st-no-reentry` feature (no re-entrancy protection).*

## Latency (ns)

| Subscribers | rustand (ST) | rustand (ST-No-Reentry) | zustand (Node.js) |
| :--- | :--- | :--- | :--- |
| **1 Subscriber** | 106.11 ns | 46.08 ns | 188.84 ns |
| **50 Subscribers** | 232.17 ns | 126.98 ns | 601.17 ns |
| **1000 Subscribers** | 3,832.00 ns | 1,710.03 ns | 8,764.15 ns |
