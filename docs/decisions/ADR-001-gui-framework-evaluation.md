# ADR-001: Desktop GUI Framework Selection (egui / eframe)

- **Status**: ACCEPTED
- **Date**: 2026-03-05
- **Author**: Principal Software Architect / Jules

## Context
RustAI requires a native desktop graphical interface capable of rendering chat conversations, rich Markdown, model selection controls, settings dialogs, and agent tool status monitors across Windows, Linux, and macOS.

We evaluated three primary Rust-centric GUI frameworks:
1. **egui / eframe**: Immediate-mode Rust GUI library. Pure Rust, fast iteration, self-contained stack, zero external compilation DSL dependencies.
2. **Slint**: Declarative markup language compiled to Rust code. Lightweight and native-feeling.
3. **Iced**: Elm-inspired declarative GUI framework. Retained-mode state management.

---

## Technical Comparison Matrix

| Evaluation Criteria | egui / eframe | Slint | Iced |
|---|---|---|---|
| **Programming Paradigm** | Immediate-mode Rust | Declarative (`.slint` markup) | Declarative / Elm Architecture |
| **Cross-Platform Support** | Windows, Linux, macOS, Web | Windows, Linux, macOS, Web | Windows, Linux, macOS, Web |
| **Markdown / Code Rendering** | Strong (Built-in rich text & widgets) | Moderate (Custom widgets) | Strong (Custom widgets) |
| **Async Tokio Integration** | Good (Context repainting) | Excellent (Native async callbacks) | Excellent (Subscriptions / Commands) |
| **Custom Styling / UI Customization** | Functional / Built-in painter | Exceptional (CSS-like markup) | Moderate / Widget styling |
| **Memory & Performance** | Very low (~60-80MB) | Extremely low (<50MB) | Moderate (~80-100MB) |
| **Ecosystem Maturity** | Production-ready (Widely used in Rust tools) | Production-ready | Rapidly maturing (COSMIC Desktop) |

---

## Decision
We select **egui / eframe** as the native GUI framework for RustAI in Phase 1 (`TASK-002`).

### Rationale
- Pure Rust implementation with no external domain-specific languages (DSLs) or pre-compilers required.
- Rapid prototyping capabilities for chat streams, side panels, and model provider configurations.
- Cross-platform support across Linux, macOS, and Windows with low memory overhead.
