# NEXT_STEPS.md — Recommended Next Action

## Active Task Overview

* **Current Active Task**: `TASK-001` (Phase 0 — Engineering Foundation and Governance)
* **Status**: In Progress / Verification Step

---

## Recommended Immediate Task: `TASK-002` (Phase 1 Setup)

Once `TASK-001` is finalized and submitted, the immediate next recommended task is:

* **Task Identifier**: `TASK-002`
* **Objective**: Initialize Phase 1 Native Desktop GUI Shell & Application State Infrastructure.
* **Prerequisites**: Phase 0 complete and verified (`TASK-001`).
* **Scope**:
  1. Finalize GUI framework selection ADR (`ADR-001`).
  2. Implement application window lifecycle in `crates/rust-ai-app`.
  3. Establish application configuration persistence crate/module.
* **Acceptance Criteria**:
  * Desktop window compiles and launches cleanly across target platforms.
  * Basic layout and state initialization verified with tests.
