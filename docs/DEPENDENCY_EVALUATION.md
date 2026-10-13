# DEPENDENCY_EVALUATION.md — Dependency & Reference Project Evaluation

This document records findings from inspecting reference projects (such as `DesignCraft`) and establishes reusable evaluation criteria.

---

## 1. Reference Project Findings: `DesignCraft`

* **Repository**: `https://github.com/storytold/designcraft`
* **Findings & Architectural Analysis**:
  * `DesignCraft` explores UI layout, design tooling, and declarative component state.
  * *Takeaway*: UI layout and state isolation principles can inform RustAI's UI shell structure in Phase 1, but its codebase should not be directly copied.
  * *Licensing*: Must verify component-level license compatibility before incorporating any design ideas or algorithmic patterns.

---

## 2. Dependency Evaluation Matrix Criteria

When evaluating crates for future phases (e.g., GUI frameworks, async runtimes, HTTP clients), score across:
1. License compatibility (MIT/Apache-2.0 preferred).
2. Cross-platform support (Linux, macOS, Windows).
3. Maintenance activity & community support.
4. Binary size and compilation overhead.
5. Async integration compatibility with Tokio.
