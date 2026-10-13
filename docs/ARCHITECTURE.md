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

* **`rust-ai-app`** (Shell / Binary): Entry point, application initialization, event loop handling.
* **`rust-ai-core`** (Domain Logic): Conversation models, agent loop state machines, task orchestrators (planned Phase 1/2).
* **`rust-ai-providers`** (Inference & APIs): `ModelProvider` trait definition, OpenAI/Anthropic/llama.cpp client implementations (planned Phase 2/3).
* **`rust-ai-tools`** (Agent Capabilities): File operations, command runner, security sandbox (planned Phase 4).

---

## 4. Architectural Rules

1. Domain crates must never import UI crates.
2. Infrastructure implementations (e.g. specific REST clients or local bindings) depend on core traits, not vice-versa.
3. Errors must use structured types with `thiserror` for crates and `anyhow` for top-level application binary handling.
