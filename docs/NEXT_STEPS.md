# NEXT_STEPS.md — Recommended Next Action

## Active Task Overview

* **Current Active Task**: `TASK-003` (Local Model Discovery, Loading & Chat Interface)
* **Status**: Complete & Verified

---

## Recommended Immediate Task: `TASK-004` (Multi-Provider Architecture & Remote APIs)

Once `TASK-003` is submitted, the immediate next recommended task is:

* **Task Identifier**: `TASK-004`
* **Objective**: Expand `ModelProvider` to support remote API services (OpenAI / Anthropic) alongside the local `LlamaCppBackend`.
* **Prerequisites**: `TASK-003` complete and verified.
* **Scope**:
  1. Add async HTTP model provider implementations in `crates/rust-ai-providers`.
  2. Implement secure API key credentials handling.
* **Acceptance Criteria**:
  * Seamless provider switching between local llama.cpp GGUF models and remote API endpoints in UI.
