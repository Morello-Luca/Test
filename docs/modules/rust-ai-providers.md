# Module Specification: `rust-ai-providers`

- **Crate**: `rust-ai-providers`
- **Path**: `crates/rust-ai-providers`
- **Status**: VERIFIED

## Purpose
Provides adapters for model execution engines. Currently implements `LlamaCppBackend` for local GGUF model loading and inference using `llama-cpp-2`.

## Public Interface
- `LlamaCppBackend::new()`
- Implements `ModelProvider` for loading GGUF model files and generating responses.

## Verification & Tests
Unit tests in `crates/rust-ai-providers/src/llama.rs` verify initial states, unloaded error handling, and file checking.
