const { createStore } = require('zustand/vanilla');
const { performance } = require('perf_hooks');

const ITERATIONS = 10_000_000;

function main() {
    console.log("Starting Zustand (Node.js) Benchmarks...");
    console.log(`Iterations per test: ${ITERATIONS}\n`);

    console.log("--- Performance Tests ---");
    runReadOnly();
    runReadHeavy();
    runContested();
    runWriteHeavy();
    runWriteOnly();

    console.log("\n--- Latency & Subscription Tests ---");
    runLatencyTest("1. Minimal Subscriptions (1 Sub)", 1);
    runLatencyTest("2. Moderate Subscriptions (50 Subs)", 50);
    runLatencyTest("3. High Subscriptions (1000 Subs)", 1000);
}

function runReadOnly() {
    const store = createStore(() => ({ count: 0 }));
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        const _ = store.getState().count;
    }
    const end = performance.now();
    printResults("1. Read-Only", ITERATIONS, 0, end - start);
}

function runReadHeavy() {
    const store = createStore(() => ({ count: 0 }));
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        for (let j = 0; j < 7; j++) {
            const _ = store.getState().count;
        }
        store.setState((s) => ({ count: s.count + 1 }));
    }
    const end = performance.now();
    printResults("2. Read-Heavy (7R:1W)", ITERATIONS * 7, ITERATIONS, end - start);
}

function runContested() {
    const store = createStore(() => ({ count: 0 }));
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        store.setState((s) => ({ count: s.count + 1 }));
        const _ = store.getState().count;
    }
    const end = performance.now();
    printResults("3. Contested (1W:1R)", ITERATIONS, ITERATIONS, end - start);
}

function runWriteHeavy() {
    const store = createStore(() => ({ count: 0 }));
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        for (let j = 0; j < 7; j++) {
            store.setState((s) => ({ count: s.count + 1 }));
        }
        const _ = store.getState().count;
    }
    const end = performance.now();
    printResults("4. Write-Heavy (7W:1R)", ITERATIONS, ITERATIONS * 7, end - start);
}

function runWriteOnly() {
    const store = createStore(() => ({ count: 0 }));
    const start = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
        store.setState((s) => ({ count: s.count + 1 }));
    }
    const end = performance.now();
    printResults("5. Write-Only", 0, ITERATIONS, end - start);
}

function runLatencyTest(name, numSubs) {
    const store = createStore(() => ({ count: 0 }));
    for (let i = 0; i < numSubs; i++) {
        store.subscribe(() => {});
    }

    const iters = 100_000;
    let totalLatencyMs = 0;
    
    const start = performance.now();
    for (let i = 0; i < iters; i++) {
        const writeStart = performance.now();
        store.setState((s) => ({ count: s.count + 1 }));
        totalLatencyMs += (performance.now() - writeStart);
    }
    const elapsedMs = performance.now() - start;

    console.log(`${name}:`);
    console.log(`  Total Time:   ${elapsedMs.toFixed(2)}ms`);
    console.log(`  Writes/sec:   ${((iters / elapsedMs) * 1000).toFixed(2)}`);
    console.log(`  Avg Latency:  ${((totalLatencyMs / iters) * 1_000_000).toFixed(2)} ns`);
}

function printResults(name, reads, writes, elapsedMs) {
    const elapsedSecs = elapsedMs / 1000;
    console.log(`${name}:`);
    console.log(`  Total Time: ${elapsedMs.toFixed(2)}ms`);
    if (reads > 0) {
        console.log(`  Reads/sec:  ${(reads / elapsedSecs).toFixed(2)}`);
    }
    if (writes > 0) {
        console.log(`  Writes/sec: ${(writes / elapsedSecs).toFixed(2)}`);
    }
}

main();
