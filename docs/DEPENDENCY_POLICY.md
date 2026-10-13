# DEPENDENCY_POLICY.md — Dependency and Licensing Policy

## 1. Core Licensing Directives

* **Original Code Licensing**: All original code created for RustAI is dual-licensed under **MIT OR Apache-2.0**.
* **Third-Party Dependency Audit**:
  * Every introduced dependency must be evaluated for license compatibility (e.g. MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, MPL-2.0).
  * Strict Copyleft licenses (such as GPL v2/v3, AGPL) are prohibited in production dependencies unless explicitly isolated or approved via ADR.

---

## 2. Criteria for Introducing New Dependencies

Before adding a third-party crate or external library, evaluate:
1. **Maturity & Maintenance**: Active repository maintenance, clear releases, responsive security patching.
2. **Native Rust Preference**: Prefer native Rust crates over C/C++ FFI bindings unless performance or backend requirements dictate otherwise (e.g., llama.cpp bindings).
3. **Scope & Dependency Bloat**: Avoid monolithic frameworks when a focused library suffices.
4. **Security & Safety**: Inspect for excessive `unsafe` code usage or unmaintained transient dependencies.
