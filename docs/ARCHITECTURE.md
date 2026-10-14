# ARCHITECTURE.md — System Architecture Overview

## 1. Architectural Philosophy

RustAI follows clean, layered architectural principles adapted for native Rust application development:

1. **Strict Dependency Direction**: Core domain logic has zero dependencies on UI frameworks or external network protocols.
2. **Interface Abstraction**: Model providers, local backends, storage engines, and system tools implement explicit Rust traits (`Async` traits where applicable).
3. **Non-Blocking UI Async Execution**: The UI shell operates on the main thread and communicates with asynchronous background tasks (tokio channels / MPSC).
4. **Explicit State Ownership**: Application state is held in single-ownership domain structures or state stores, avoiding unmanaged global mutable state.

---

## 2. High-Level Subsystem Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     UI Shell Crate                      │
│            (Desktop Window, Components, Layout)         │
└────────────────────────────┬────────────────────────────┘
                             │ Async Messages / Events
                             ▼
┌─────────────────────────────────────────────────────────┐
│                 Application Core Crate                  │
│       (Orchestration, Session Mgr, Agent Runtime)       │
└──────────────┬────────────────────────────┬─────────────┘
               │                            │
               ▼                            ▼
┌──────────────────────────────┐ ┌────────────────────────┐
│    Model Provider Adapters   │ │ Persist & Config Crate │
│ (Local Backends, Remote API) │ │ (SQLite / Local Files) │
└──────────────────────────────┘ └────────────────────────┘
```

---

## 3. Crate and Module Boundaries

* **`rust-ai-app`** (Shell / Binary): GUI entry point, application initialization, settings management (system prompt versions & CPU/GPU device selection), non-blocking background thread worker, and egui interface layout.
* **`rust-ai-core`** (Domain Logic): Conversation models (`ConversationMessage`), model discovery (`ModelDiscovery`), model provider traits (`ModelProvider`), and loading lifecycle states (`ModelLoadState`).
* **`rust-ai-providers`** (Inference & APIs): `LlamaCppBackend` adapter wrapping `llama-cpp-2` for local GGUF model loading, CPU/GPU hardware offload parameters (`load_model_with_device`), batch evaluation, greedy/sampled decoding, and error propagation.
* **`rust-ai-tools`** (Agent Capabilities): File operations, command runner, security sandbox (planned Phase 4).

---

## 5. System Prompt & Inference Pipeline

1. **System Prompt Management**: System prompt versions (`SystemPromptVersion`) are stored and persisted in `AppConfig`. The active prompt is bound to new conversations when created (`start_new_conversation`), ensuring existing conversations retain their initial prompt context.
2. **CPU/GPU Device Selection**: Hardware acceleration is configurable via `InferenceDevice`. `LlamaCppBackend::load_model_with_device` sets `n_gpu_layers` on `LlamaModelParams` (99 for GPU offloading, 0 for CPU).
3. **Inference Execution & Error Propagation**: Prompt tokenization and decoding use batch evaluation with token-by-token greedy sampling. If token generation yields no tokens or encounters an initialization error, standard error strings are propagated rather than masked.
4. **GGUF Smoke Testing**: Live model inference testing is opt-in via the `RUST_AI_TEST_GGUF` environment variable in debug builds, gracefully skipping when no `.gguf` file is present.

---

## 4. Architectural Rules

1. Domain crates must never import UI crates.
2. Infrastructure implementations (e.g. specific REST clients or local bindings) depend on core traits, not vice-versa.
3. Errors must use structured types with `thiserror` for crates and `anyhow` for top-level application binary handling.
