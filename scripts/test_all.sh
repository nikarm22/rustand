#!/usr/bin/env bash
set -e

RUNTIMES=("single-threaded" "multi-threaded" "tokio" "async-std")
FUZZ_TARGETS=("single_threaded" "multi_threaded" "tokio" "async_std")

echo "Running tests for all runtimes..."

for runtime in "${RUNTIMES[@]}"; do
    echo -e "\n>>> Testing runtime: $runtime"
    cargo test --features "$runtime" --no-default-features
done

echo -e "\nChecking for fuzzing tools..."
if command -v cargo-fuzz &> /dev/null && rustup toolchain list | grep -q "nightly"; then
    echo "Running fuzzers (smoke test)..."
    for target in "${FUZZ_TARGETS[@]}"; do
        runtime="${target//_/-}"
        echo -e "\n>>> Fuzzing target: $target (runtime: $runtime)"
        cargo +nightly fuzz run "$target" --features "$runtime" -- -runs=1000
    done
else
    echo -e "\nSkipping fuzzers: cargo-fuzz or nightly toolchain not found."
fi

echo -e "\nAll tests completed!"
