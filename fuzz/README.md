# Fuzz testing

Uses `cargo-fuzz` (`libFuzzer` wrapper). As of June '26 requires nightly toolchain.

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

## Running Fuzz tests

### Single-threaded store
```bash
cargo +nightly fuzz run single_threaded --features single-threaded
```

### Multi-threaded store
Multi-threaded and async runtime support is WIP in the core crate.