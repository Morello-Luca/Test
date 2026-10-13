# Error Handling Architecture

RustAI enforces structured error handling across crate boundaries.

## Crate-Level Error Boundaries

* **`rust-ai-core`**: Defines `DomainError` using `thiserror`.
* **`rust-ai-providers`**: Defines `ProviderError` mapping network errors, rate limits, and parsing failures.
* **`rust-ai-tools`**: Defines `ToolError` capturing permission denied, missing files, and execution timeouts.
* **`rust-ai-app`**: Consolidates subsystem errors at the top-level main function.

## Rules
* Errors must provide clear human-readable messages and preserve root causes using source attributes (`#[from]`).
* Panics are strictly forbidden in non-test library code.
