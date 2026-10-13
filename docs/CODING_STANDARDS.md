# CODING_STANDARDS.md — Rust Coding Guidelines & Conventions

## 1. Code Style and Formatting

* **Formatting**: Standard `rustfmt` rules enforced via `rustfmt.toml`. Run `cargo fmt --all` prior to committing.
* **Linting**: Standard `clippy` checks enforced with `-D warnings`. Deny all clippy warnings in CI.
* **Naming Conventions**:
  * Modules, files, functions, variables: `snake_case`.
  * Types, Traits, Enums, Structs: `PascalCase`.
  * Constants, Statics: `SCREAMING_SNAKE_CASE`.

---

## 2. Error Handling Rules

* **Library Crates**: Use structured, explicit error enums with `thiserror` (or standard `std::fmt::Display`). Avoid `anyhow` or unhandled panic traps in library crates.
* **Application Binary (`rust-ai-app`)**: May use `anyhow` or `eyre` at the top-level main function.
* **No Silent Ignored Errors**: Do not use `let _ = ...` on error-returning expressions unless explicitly documented with a comment explaining why failure is harmless.
* **No `unwrap()` or `expect()` in Production Code**:
  * `unwrap()` and `expect()` are permitted only in test code (`#[cfg(test)]`).
  * Production code must propagate errors using `?` or handle them explicitly.

---

## 3. Async Guidelines

* Primary async runtime: `tokio`.
* Avoid blocking operations inside async task contexts. Use `tokio::task::spawn_blocking` for CPU-heavy or synchronous file I/O operations.
