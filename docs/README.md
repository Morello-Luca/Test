# RustAI Documentation Directory

Welcome to the documentation system for `RustAI`. The documentation is organized as a single, verified source of truth for humans and AI coding agents.

## Navigation & Canonical Documents

| Document | Purpose |
|---|---|
| [`PROJECT_VISION.md`](./PROJECT_VISION.md) | High-level vision, target users, guiding principles, non-goals |
| [`REQUIREMENTS.md`](./REQUIREMENTS.md) | Stable requirement identifiers (FR, NFR, SEC) and verification status |
| [`SCOPE_AND_ROADMAP.md`](./SCOPE_AND_ROADMAP.md) | Phased development plan (Phase 0 to Phase 6) and scope boundaries |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | System overview, module boundaries, async execution, data flow |
| [`MODULE_CATALOG.md`](./MODULE_CATALOG.md) | Catalog of implemented and planned modules and crate responsibilities |
| [`DEVELOPMENT_WORKFLOW.md`](./DEVELOPMENT_WORKFLOW.md) | Agent session protocol, quality gates, and task workflow |
| [`CODING_STANDARDS.md`](./CODING_STANDARDS.md) | Rust style guidelines, error handling, async, public API rules |
| [`TESTING_STRATEGY.md`](./TESTING_STRATEGY.md) | Testing strategy, coverage, unit/integration/architectural checks |
| [`DEPENDENCY_POLICY.md`](./DEPENDENCY_POLICY.md) | Dependency rules, licensing criteria, third-party audit guidelines |
| [`DEPENDENCY_EVALUATION.md`](./DEPENDENCY_EVALUATION.md) | Reference repository evaluation (DesignCraft) and dependency analysis |
| [`SECURITY_MODEL.md`](./SECURITY_MODEL.md) | Threat model, sandbox rules, agent tool permissions, secret handling |
| [`CURRENT_STATE.md`](./CURRENT_STATE.md) | Verified state of repository functionality at latest checkpoint |
| [`NEXT_STEPS.md`](./NEXT_STEPS.md) | Recommended immediate task, acceptance criteria, and prerequisites |
| [`TASKS.md`](./TASKS.md) | Central task index with identifiers (TASK-xxx) and status |

## Documentation Subdirectories

* `architecture/`: Deep dive documents (`system-overview.md`, `dependency-rules.md`, `data-flow.md`, `error-handling.md`, `configuration.md`).
* `decisions/`: Architecture Decision Records (ADRs).
* `modules/`: Detailed specifications for individual crates and modules.
* `tasks/`: Individual task specifications.
* `sessions/`: AI session handoff logs.
* `templates/`: Templates for tasks, ADRs, module specs, and session handoffs.
