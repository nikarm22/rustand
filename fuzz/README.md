# rustand Fuzz Testing

This directory contains fuzz tests for the `rustand` crate, targeting all 4 runtime implementations.

## Setup

Fuzzing requires the `nightly` toolchain and `cargo-fuzz`.

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

## Running Fuzzers

Each runtime has a dedicated fuzz target. You MUST enable the corresponding feature flag when running.

### 1. Multi-threaded (Default)
```bash
cargo +nightly fuzz run multi_threaded --features multi_threaded
```

### 2. Single-threaded (WASM optimized)
```bash
cargo +nightly fuzz run single_threaded --features single_threaded
```

### 3. Tokio Runtime
```bash
cargo +nightly fuzz run tokio --features tokio
```

### 4. Async-std Runtime
```bash
cargo +nightly fuzz run async_std --features async_std
```

## Fuzzing Logic

The fuzzers use the `arbitrary` crate to generate a sequence of `Operation`s:
- `Get`: Calls `store.get().await`.
- `Set(u32)`: Calls `store.set(|s| *s = val).await`.
- `Subscribe`: Adds a new subscriber.
- `Unsubscribe(idx)`: Removes an existing subscriber by index.
- `Wait`: Yields the executor to allow background tasks to process.

This sequence is executed against a `Store<u32>` to ensure no panics, deadlocks, or undefined behavior occur under arbitrary operation sequences.
