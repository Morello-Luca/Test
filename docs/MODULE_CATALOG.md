# MODULE_CATALOG.md — Workspace Module & Crate Catalog

This catalog documents implemented and planned crates and modules in `RustAI`.

---

## Workspace Crates Overview

| Crate Name | Path | Description | Status |
|---|---|---|---|
| `rust-ai-app` | `crates/rust-ai-app` | Entry point binary and UI shell host. | `VERIFIED` |
| `rust-ai-core` | `crates/rust-ai-core` | Core domain models, state management, traits. | `PLANNED` |
| `rust-ai-providers` | `crates/rust-ai-providers` | Adapters for local and remote AI models. | `PLANNED` |
| `rust-ai-tools` | `crates/rust-ai-tools` | Sandboxed tool execution engine for agents. | `PLANNED` |

---

## Internal Module Catalog

### `rust-ai-app`
* **Purpose**: Primary executable and shell interface.
* **Public Interface**:
  * `app_status() -> &'static str`: Returns current status string for application verification.
* **Dependencies**: None (workspace internal root).
* **Status**: `VERIFIED`.
