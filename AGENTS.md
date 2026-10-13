# AGENTS.md — AI Agent Guidance and Entry Point

Welcome, AI coding agent. This repository (`RustAI`) is designed to be developed iteratively across independent AI agent sessions. The Git repository is the single authoritative source of truth.

---

## 1. MANDATORY SESSION-START PROTOCOL

Before modifying any files or planning code changes, you **MUST** complete the following reading sequence:

1. **Read `AGENTS.md`** (this file).
2. **Read `docs/README.md`** and **`docs/CURRENT_STATE.md`** to understand the verified state of the codebase.
3. **Read `docs/ARCHITECTURE.md`** and relevant module specifications in `docs/modules/` or `docs/architecture/`.
4. **Read `docs/TASKS.md`** and **`docs/NEXT_STEPS.md`** to identify the active task and its acceptance criteria.
5. **Inspect the repository state and Git changes** (`git status`, `git diff`) to confirm starting context.

---

## 2. CORE NON-NEGOTIABLE DIRECTIVES

* **Verified State vs Planned State**:
  * Never claim a feature is complete or verified unless builds (`cargo build`), tests (`cargo test`), linter (`cargo clippy`), formatter (`cargo fmt --check`), and doc validation (`python3 scripts/validate_docs.py`) have actually been executed and succeeded.
  * Status levels: `PLANNED` -> `IMPLEMENTED` -> `VERIFIED`.

* **Architectural Integrity**:
  * Do NOT silently alter established architectural decisions or dependency direction.
  * Significant architectural changes require an Architecture Decision Record (ADR) in `docs/decisions/`.
  * Do NOT remove requirements or weaken tests to force a build to pass.

* **Controlled Changes**:
  * Keep changes strictly within assigned scope. Avoid unrelated refactoring or modifying files outside the active task.
  * Do NOT introduce unvetted third-party dependencies without reviewing license, maintenance, and security considerations (see `docs/DEPENDENCY_POLICY.md`).

---

## 3. MANDATORY SESSION-END PROTOCOL

At the conclusion of your task, you **MUST**:

1. Run the quality gate pipeline:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   cargo doc --workspace --no-deps
   python3 scripts/validate_docs.py
   ```
2. Inspect `git status` and `git diff` to ensure no unintended file modifications or artifacts exist.
3. Update repository tracking state:
   * `docs/CURRENT_STATE.md`: Update to reflect newly verified functionality.
   * `docs/NEXT_STEPS.md`: Set the next recommended task and acceptance criteria.
   * `docs/TASKS.md`: Mark completed task status (`DONE` / `VERIFIED`).
4. Generate a session handoff record in `docs/sessions/` using `docs/templates/SESSION_HANDOFF_TEMPLATE.md`.

---

## 4. DOCUMENTATION INDEX & NAVIGATION

* **Product Vision & Goals**: `docs/PROJECT_VISION.md`
* **Requirements Registry**: `docs/REQUIREMENTS.md`
* **Roadmap & Phases**: `docs/SCOPE_AND_ROADMAP.md`
* **System Architecture**: `docs/ARCHITECTURE.md`
* **Module Catalog**: `docs/MODULE_CATALOG.md`
* **Development Workflow**: `docs/DEVELOPMENT_WORKFLOW.md`
* **Coding Standards**: `docs/CODING_STANDARDS.md`
* **Testing Strategy**: `docs/TESTING_STRATEGY.md`
* **Dependency & Reuse Policy**: `docs/DEPENDENCY_POLICY.md` & `docs/DEPENDENCY_EVALUATION.md`
* **Security Model**: `docs/SECURITY_MODEL.md`
* **Current State**: `docs/CURRENT_STATE.md`
* **Next Steps**: `docs/NEXT_STEPS.md`
* **Task Queue**: `docs/TASKS.md`
