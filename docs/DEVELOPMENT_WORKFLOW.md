# DEVELOPMENT_WORKFLOW.md — AI Agent & Developer Workflow

This document outlines the required workflow for human contributors and AI coding agents.

---

## 1. AI Session Continuity Lifecycle

```
[ Session Start ]
       │
       ▼
Read AGENTS.md -> Read docs/CURRENT_STATE.md -> Inspect Task in docs/TASKS.md
       │
       ▼
Formulate / Confirm Plan (Execute in incremental steps)
       │
       ▼
Modify Code -> Run Local Quality Checks (cargo test, clippy, doc validation)
       │
       ▼
Update Docs (docs/CURRENT_STATE.md, docs/NEXT_STEPS.md, Session Handoff)
       │
       ▼
[ Session End ]
```

---

## 2. Mandatory Quality Gate Commands

Every session must execute and pass the following quality gate suite prior to completing work:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Linter check
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Unit & Integration test suite
cargo test --workspace

# 4. Documentation compilation
cargo doc --workspace --no-deps

# 5. Documentation structure & link validation
python3 scripts/validate_docs.py
```

---

## 3. State Management Rules

* Do not update a task status to `DONE` or `VERIFIED` in `docs/TASKS.md` unless all quality gate checks have passed cleanly.
* Document newly discovered technical debt or risks in `docs/CURRENT_STATE.md` and create corresponding tasks in `docs/TASKS.md`.
