RUNTIMES = single-threaded multi-threaded tokio async-std
FUZZ_TARGETS = single_threaded multi_threaded tokio async_std

.PHONY: test-all test-runtimes fuzz-all $(RUNTIMES) $(FUZZ_TARGETS)

test-all: test-runtimes fuzz-all

test-runtimes: $(RUNTIMES)

$(RUNTIMES):
	cargo test --features $@ --no-default-features

fuzz-all:
	@if command -v cargo-fuzz >/dev/null 2>&1 && rustup toolchain list | grep -q "nightly"; then \
		$(MAKE) $(FUZZ_TARGETS); \
	else \
		echo "Skipping fuzzers: cargo-fuzz or nightly toolchain not found."; \
	fi

$(FUZZ_TARGETS):
	cargo +nightly fuzz run $@ --features $(subst _,-,$@) -- -runs=1000
