# NEXT_STEPS.md — Recommended Next Action

## Active Task Overview

* **Current Active Task**: `TASK-002` (Phase 1 — Native Desktop Shell Scaffolding & State Management)
* **Status**: Verification / Finalization Step

---

## Recommended Immediate Task: `TASK-003` (Configuration Persistence & Storage Engine) or `TASK-004` (Chat Architecture)

Once `TASK-002` is finalized and submitted, the immediate next recommended task is:

* **Task Identifier**: `TASK-003`
* **Objective**: Create dedicated settings storage and model configuration management.
* **Prerequisites**: Phase 1 desktop shell scaffolding complete and verified (`TASK-002`).
* **Scope**:
  1. Expand `AppConfig` into a dedicated persistence module or crate (`crates/rust-ai-core`).
  2. Implement secure credential handling helpers for model API keys.
* **Acceptance Criteria**:
  * Credential masking and configuration load/save verified via unit tests.
