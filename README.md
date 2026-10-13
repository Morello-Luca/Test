# RustAI — Modular Native AI Application Platform

RustAI is a modular, native Rust desktop application designed for interacting with local and remote AI models, operating AI agents, and assisting with repository-aware software development under human supervision.

---

## Architectural Principles

* **Fast & Native**: Built in Rust for maximum responsiveness, minimal footprint, and zero runtime overhead.
* **Modular Architecture**: Strict boundaries between UI shell, domain logic, model provider adapters, local inference, persistence, and agent execution runtimes.
* **AI-Agent Continuous Maintainability**: Governed by an explicit documentation and tracking system enabling autonomous AI coding agents to resume development safely across independent sessions.
* **Dual Licensed**: Open source under MIT OR Apache-2.0.

---

## Quick Start & Verification

### Prerequisites
* Rust 1.80+ (Toolchain 1.94.0 recommended via `rust-toolchain.toml`)
* Python 3.8+ (for documentation validation tooling)

### Build & Test
```bash
# Build workspace
cargo build

# Run unit and integration tests
cargo test

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Validate documentation integrity
python3 scripts/validate_docs.py
```

---

## Documentation Entry Point

For AI agents and human contributors, start by reading [`AGENTS.md`](./AGENTS.md) and [`docs/README.md`](./docs/README.md).

---

## License

This project is licensed under either of:

* Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE))
* MIT License ([`LICENSE-MIT`](./LICENSE-MIT))

at your option.
