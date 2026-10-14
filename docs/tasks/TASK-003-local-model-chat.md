# TASK-003: Local Model Discovery, Loading, and Chat Interface

- **Task ID**: TASK-003
- **Title**: Local Model Discovery, llama.cpp Loading, and Chat Integration
- **Priority**: High
- **Phase**: Phase 1 / Phase 3 Bridge
- **Status**: VERIFIED

## Objective
Implement end-to-end local model discovery, llama.cpp GGUF model loading, lifecycle state tracking, and chat generation UI in `RustAI`.

## Related Requirements
- `FR-006`, `FR-007`, `NFR-002`, `NFR-003`

## Prerequisites
- `TASK-001` and `TASK-002` complete and verified.

## Acceptance Criteria
- [x] Scan configurable `models/` directory for `.gguf` files via `ModelDiscovery`.
- [x] Integrate `llama-cpp-2` bindings in `crates/rust-ai-providers` for GGUF model loading and inference.
- [x] Manage model loading states (`Unloaded`, `Loading`, `Loaded`, `Error`).
- [x] Display local model selection side panel with clear empty state guidance when no models exist.
- [x] Provide chat prompt interface with model response generation.
- [x] All unit and documentation tests pass cleanly.
