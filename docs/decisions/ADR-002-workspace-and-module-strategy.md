# ADR-002: Cargo Workspace and Module Partitioning Strategy

- **Status**: ACCEPTED
- **Date**: 2026-03-05
- **Author**: Principal Software Architect / Jules

## Context
We needed to decide whether RustAI should start as a single monolithic crate or a Cargo workspace with partitioned crates.

## Decision
We adopt a **minimal Cargo workspace** starting with a single application binary crate (`crates/rust-ai-app`).

Future workspace expansion will occur incrementally as domain boundaries solidify:
* `crates/rust-ai-app`: UI host and shell binary.
* `crates/rust-ai-core`: Core conversation and agent domain logic.
* `crates/rust-ai-providers`: Inference backend adapters.
* `crates/rust-ai-tools`: Security sandbox and agent tool execution.

## Consequences
- Prevents premature abstraction while preserving clear future crate boundaries.
- Keeps compilation time minimal during initial foundation setup.
