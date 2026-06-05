# Contributing to rustand

Thank you for your interest in improving `rustand`!

## How to Contribute

1. **Report Bugs:** Open an issue describing the bug and how to reproduce it.
2. **Suggest Features:** Open an issue to discuss your idea.
3. **Submit Pull Requests:**
   - Fork the repository.
   - Create a new branch for your changes.
   - Ensure your code follows the existing style.
   - Add tests for any new functionality.
   - Ensure all tests pass with `cargo test`.
   - Submit a pull request.

## Development Guidelines

- **Git Hooks:** We use a pre-commit hook to automatically format code. To enable it, run:
  ```bash
  git config core.hooksPath .githooks
  ```
- **No Unsafe:** The use of `unsafe` code is strictly prohibited. We prioritize memory safety and simplicity over micro-optimizations.
- **No Dependencies:** This is a zero-dependency crate. Contributions must rely exclusively on the Rust Standard Library (`std`).
- **Documentation:** Every public API must be documented with doc comments.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.
