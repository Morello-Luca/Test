# RustAI — Modular Native AI Application Platform

RustAI is a modular, native Rust desktop application designed for interacting with local and remote AI models, operating AI agents, and assisting with repository-aware software development under human supervision.

---

## Architectural Principles

* **Fast & Native**: Built in Rust for maximum responsiveness, minimal footprint, and zero runtime overhead.
* **Modular Architecture**: Strict boundaries between UI shell, domain logic, model provider adapters, local inference, persistence, and agent execution runtimes.
* **AI-Agent Continuous Maintainability**: Governed by an explicit documentation and tracking system enabling autonomous AI coding agents to resume development safely across independent sessions.
* **Dual Licensed**: Open source under MIT OR Apache-2.0.

---

## How to Work with AI Coding Agents (e.g., Jules)

This repository is designed so that you can prompt an AI coding agent (like Jules) in new, independent sessions without pasting long project specifications into the chat.

### 1. Initializing a Session
When starting a new session with an AI agent, you can simply point the agent to `AGENTS.md` at the root of the repository.

**Example Prompt to Start a Session:**
> *"Please read `AGENTS.md`, reconstruct the current project state from the repository documentation, inspect `docs/TASKS.md` and `docs/NEXT_STEPS.md`, and begin work on the next ready task. Follow all documented protocols and verify your changes before finishing."*

### 2. Directing Specific Tasks
To have an agent work on a specific task:
1. Reference the task ID from [`docs/TASKS.md`](./docs/TASKS.md) (e.g., `TASK-002`).
2. Point the agent to its specification in `docs/tasks/` (e.g., `docs/tasks/TASK-002-desktop-shell.md`).

**Example Task Prompt:**
> *"Please execute `TASK-002` as specified in `docs/tasks/TASK-002-desktop-shell.md`. Read `AGENTS.md` first to ensure compliance with session protocols and architectural constraints."*

### 3. Creating New Tasks or Proposing Architectural Changes
If you have a new feature request:
- Ask the agent to create a task specification in `docs/tasks/TASK-XXX-<name>.md` and register it in `docs/TASKS.md`.
- For significant architectural changes, ask the agent to create a new ADR proposal in `docs/decisions/` using `docs/templates/ADR_TEMPLATE.md`.

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

## Documentation Navigation

For AI agents and human contributors, start by reading [`AGENTS.md`](./AGENTS.md) and [`docs/README.md`](./docs/README.md).

---

## License

This project is licensed under either of:

* Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE))
* MIT License ([`LICENSE-MIT`](./LICENSE-MIT))

at your option.
