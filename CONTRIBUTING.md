# Contributing to rustand

First off, thank you for considering contributing to `rustand`! It's people like you that make the Rust ecosystem such a great community.

`rustand` aims to be the standard foundational primitive for state management in Rust. Whether you're fixing a bug, improving documentation, or building an ecosystem integration, your help is appreciated.

## 🗺️ The Ecosystem Vision

We are actively looking for contributors to help build and maintain our integration crates:
- **`rustand-slint`**: Bring reactive state to Slint.
- **`rustand-tauri`**: Bridge the gap between Rust and JS state.
- **`rustand-leptos`**: Integrate with the Leptos signal system.
- **`rustand-egui`**: Smooth state management for egui.
- **`rustand-redox`**: State primitives for the Redox ecosystem.

If you are interested in leading or contributing to these, please open an [issue](https://github.com/nikarm22/rustand/issues)!

## 🛠️ How to Contribute

1.  **Search for existing issues:** Before opening a new one, check if it's already being discussed.
2.  **Fork and Branch:** Create a feature branch for your work.
3.  **Tests are Mandatory:** Every bug fix or new feature must include a test case.
4.  **Format and Lint:** We strictly enforce `rustfmt` and `clippy`.
5.  **Documentation:** All public APIs must be documented.

## 📜 Development Guidelines

- **Zero Unsafe:** The use of `unsafe` is strictly prohibited.
- **Zero Dependencies:** The core crate must remain dependency-free (standard library only). Optional dependencies for runtimes (Tokio/async-std) are allowed only via feature flags.
- **Atomic Parity:** Ensure changes to `multi-threaded` patterns are reflected (or intentionally omitted) in the `single-threaded` implementation.

## 🚀 Getting Started

To set up your development environment:

```bash
# Install git hooks for auto-formatting
git config core.hooksPath .githooks

# Run the test suite across all features
cargo test --all-features
```

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the MIT License.
