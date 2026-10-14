# TASK-003: Local Model Discovery, Loading, and Chat Interface

- **Task ID**: TASK-003
- **Title**: Local Model Discovery, llama.cpp Loading, and Chat Integration
- **Priority**: High
- **Phase**: Phase 1 / Phase 3 Bridge
- **Status**: IN_PROGRESS

## Objective
Implement end-to-end local model discovery, llama.cpp GGUF model loading, lifecycle state tracking, system prompt management, CPU/GPU inference settings, and chat generation UI in `RustAI`.

## Related Requirements
- `FR-006`, `FR-007`, `NFR-002`, `NFR-003`

## Prerequisites
- `TASK-001` and `TASK-002` complete and verified.

## Acceptance Criteria
- [x] Scan configurable `models/` directory for `.gguf` files via `ModelDiscovery`.
- [x] Integrate `llama-cpp-2` bindings in `crates/rust-ai-providers` for GGUF model loading and inference.
- [x] Manage model loading states (`Unloaded`, `Loading`, `Loaded`, `Error`).
- [x] Display local model selection side panel with clear empty state guidance when no models exist.
- [x] Provide chat prompt interface with genuine model response generation and correct token batch/sampler evaluation.
- [x] Fix empty-response bug by replacing generic error masking with clear error propagation.
- [x] System prompt versioning (creation, selection, editing, deletion, persistence) and conversation scoping.
- [x] CPU and GPU inference settings in `AppConfig` and `LlamaCppBackend` (`load_model_with_device`).
- [x] Opt-in live GGUF smoke test via `RUST_AI_TEST_GGUF` environment variable in debug builds.
- [x] All unit and documentation tests pass cleanly.
