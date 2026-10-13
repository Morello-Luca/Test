# REQUIREMENTS.md — System Requirements Registry

All requirements in RustAI are given stable identifiers for traceability.

Requirement Status definitions:
* `PLANNED`: Defined and assigned to roadmap phase.
* `IMPLEMENTED`: Code written.
* `VERIFIED`: Verified via automated tests/checks in CI.

---

## Functional Requirements (FR)

| ID | Description | Priority | Phase | Status |
|---|---|---|---|---|
| `FR-001` | Workspace & Crate Scaffolding: Basic Rust Cargo workspace with initial application shell executable. | High | Phase 0 | `VERIFIED` |
| `FR-002` | Repository Documentation Governance: Complete set of canonical documentation files and AI session protocols. | High | Phase 0 | `VERIFIED` |
| `FR-003` | Automated Documentation Validation: Script verifying required files, Markdown links, and ID formats. | High | Phase 0 | `VERIFIED` |
| `FR-004` | GUI Application Shell: Native desktop window with main navigation layout, theme support, and responsive window handling. | High | Phase 1 | `PLANNED` |
| `FR-005` | Application State & Config Management: Persistent settings, model profiles, and user preferences stored securely. | Medium | Phase 1 | `PLANNED` |
| `FR-006` | Multi-Provider Chat Interface: Streaming chat UI supporting concurrent message streams, Markdown, and syntax highlighting. | High | Phase 2 | `PLANNED` |
| `FR-007` | Local Inference Integration: Model loading, hardware detection, and local backend orchestration (e.g. llama.cpp / Ollama). | High | Phase 3 | `PLANNED` |
| `FR-008` | Tool-Using Agent Runtime: Secure tool execution framework (filesystem, process execution) with permission prompts. | High | Phase 4 | `PLANNED` |
| `FR-009` | Repository Intelligence Engine: Codebase indexing, context search, patch creation, and Git workspace operations. | High | Phase 5 | `PLANNED` |

---

## Non-Functional Requirements (NFR)

| ID | Description | Priority | Phase | Status |
|---|---|---|---|---|
| `NFR-001` | Build & Verification Speed: Workspace builds and tests execute in clean time; clean CI runs under 5 minutes. | High | Phase 0 | `VERIFIED` |
| `NFR-002` | UI Responsiveness: UI thread must never freeze or block during long-running async inference or disk I/O. | High | Phase 1 | `PLANNED` |
| `NFR-003` | Memory Efficiency: Idle application baseline memory footprint should remain under 150MB. | Medium | Phase 1 | `PLANNED` |
| `NFR-004` | AI Session Continuity: Fresh AI agent session must reconstruct project state in <2 minutes reading docs. | High | Phase 0 | `VERIFIED` |

---

## Security Requirements (SEC)

| ID | Description | Priority | Phase | Status |
|---|---|---|---|---|
| `SEC-001` | Dual License Compliance & Dependency Audit: Zero incompatible third-party licenses in production tree. | High | Phase 0 | `VERIFIED` |
| `SEC-002` | Sandboxed Tool Execution: Terminal commands and file edits executed by agents must be scoped to explicitly approved working directories. | High | Phase 4 | `PLANNED` |
| `SEC-003` | Secrets Management: API keys and sensitive credentials stored encrypted or in native OS keystores, never plaintext in logs. | High | Phase 2 | `PLANNED` |
