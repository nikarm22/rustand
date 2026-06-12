# Multithreaded Benchmarking Criteria

This document outlines the performance and latency benchmarking criteria for the `rustand` library in multithreaded environments.

## Performance Tests
These tests focus on throughput, measuring the number of operations per second under different read/write ratios.

| Scenario | Writer Threads | Reader Threads | Metrics |
| :--- | :---: | :---: | :--- |
| **1. Read-Only (No Contention)** | 0 | 8 | reads/sec |
| **2. Read-Heavy** | 1 | 7 | reads/sec, writes/sec |
| **3. Write-Heavy** | 7 | 1 | reads/sec, writes/sec |
| **4. Write-Only** | 8 | 0 | writes/sec |

## Latency & Subscription Tests
These tests measure the impact of subscription overhead on write latency and overall throughput.

| Scenario | Writer Threads | Subscribers | Metrics |
| :--- | :---: | :---: | :--- |
| **1. Minimal Subscriptions** | 8 | 1 | latency, writes/sec |
| **2. Moderate Subscriptions** | 8 | 50 | latency, writes/sec |
| **3. High Subscriptions** | 8 | 1000 | latency, writes/sec |

---
*Note: All tests should be conducted on consistent hardware to ensure comparability of results.*
