# SCOPE_AND_ROADMAP.md — Project Roadmap and Development Phases

Development of RustAI is partitioned into distinct phases. Each phase builds upon the verified deliverables of previous phases.

---

## Roadmap Overview

```
Phase 0: Engineering Foundation & Governance (CURRENT)
    │
    ▼
Phase 1: Native Desktop Shell & Core Workspace
    │
    ▼
Phase 2: Chat Architecture & Provider Interface
    │
    ▼
Phase 3: Local Inference & Remote Model Integration
    │
    ▼
Phase 4: Tool-Using AI Agent Runtime
    │
    ▼
Phase 5: Repository Intelligence & Coding Agent
    │
    ▼
Phase 6: Evaluated Self-Improvement & Benchmarking
```

---

## Detailed Phase Breakdown

### Phase 0 — Engineering Foundation & Governance (Current Phase)
* **Goal**: Establish repository structure, dual licensing, Cargo workspace scaffolding, canonical documentation hierarchy, doc validation script, GitHub Actions CI, and AI agent session protocols.
* **Deliverables**: Cargo workspace, `AGENTS.md`, full `docs/` hierarchy, `scripts/validate_docs.py`, `.github/workflows/ci.yml`.
* **Status**: In Progress / Verification.

### Phase 1 — Native Desktop Shell & Core Workspace
* **Goal**: Implement the native GUI shell, navigation layout, theme management, configuration persistence, and logging system.
* **Prerequisites**: Phase 0 complete.
* **Key Tasks**: GUI framework selection (ADR-001 evaluation finalization), application window lifecycle, settings crate/module.

### Phase 2 — Chat Architecture & Provider Interface
* **Goal**: Implement conversation domain model, message persistence, Markdown code block rendering, streaming response handling, and unified model provider trait (`ModelProvider`).
* **Prerequisites**: Phase 1 complete.

### Phase 3 — Local Inference & Remote Model Integration
* **Goal**: Integrate local backend orchestration (e.g. llama.cpp bindings / Ollama API adapter) and remote APIs (OpenAI / Anthropic / Groq).

### Phase 4 — Tool-Using AI Agent Runtime
* **Goal**: Build agent runtime execution engine with tool invocation traits, sandboxed filesystem access, command execution, and human approval UI gates.

### Phase 5 — Repository Intelligence & Coding Agent
* **Goal**: Add code parsing/indexing, context search, automated diff creation, test suite execution, and Git patch management.

### Phase 6 — Evaluated Self-Improvement & Benchmarking
* **Goal**: Benchmarking suite, regression testing, isolated experiment harnesses for proposing repository refactoring under human evaluation.
