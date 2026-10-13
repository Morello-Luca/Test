# ADR-001: Desktop GUI Framework Evaluation (Slint vs egui vs Iced)

- **Status**: PENDING (Recommendation Formulated; Implementation Deferred to Phase 1)
- **Date**: 2026-03-05
- **Author**: Principal Software Architect / Jules

## Context
RustAI requires a native desktop graphical interface capable of rendering chat conversations, rich Markdown with syntax highlighting, model selection controls, settings dialogs, and agent tool status monitors across Windows, Linux, and macOS.

We evaluated three primary Rust-centric GUI frameworks:
1. **Slint**: Declarative markup language compiled to Rust code. Lightweight and native-feeling.
2. **egui**: Immediate-mode Rust GUI library. Fast iteration, self-contained stack.
3. **Iced**: Elm-inspired declarative GUI framework. Retained-mode state management.

---

## Technical Comparison Matrix

| Evaluation Criteria | Slint | egui | Iced |
|---|---|---|---|
| **Programming Paradigm** | Declarative (`.slint` markup) | Immediate-mode Rust | Declarative / Elm Architecture |
| **Cross-Platform Support** | Windows, Linux, macOS, Web | Windows, Linux, macOS, Web | Windows, Linux, macOS, Web |
| **Markdown / Code Rendering** | Moderate (Custom widgets) | Strong (Built-in rich text) | Strong (Custom widgets) |
| **Async Tokio Integration** | Excellent (Native async callbacks) | Good (Event loop repainting) | Excellent (Subscriptions / Commands) |
| **Custom Styling / UI Customization** | Exceptional (CSS-like markup) | Functional / Custom painter | Moderate / Widget styling |
| **Memory & Performance** | Extremely low (<50MB baseline) | Very low (~60-80MB) | Moderate (~80-100MB) |
| **Ecosystem Maturity** | Production-ready (Commercial & Open-Source) | Mature (Widely used in Rust tools) | Rapidly maturing (COSMIC Desktop) |

---

## Evaluation Findings & Trade-offs

1. **Slint**:
   - *Pros*: Excellent separation of presentation (`.slint`) from Rust logic, outstanding performance, commercial backing, crisp vector rendering.
   - *Cons*: DSL requires learning `.slint` syntax; rich text/Markdown parsing requires custom Rust widget adapters.

2. **egui**:
   - *Pros*: Pure Rust, zero external DSLs, rapid prototyping, built-in text styling and code block rendering.
   - *Cons*: Immediate mode re-renders on every frame/event which can lead to higher CPU usage if repainting isn't strictly controlled during streaming chat responses.

3. **Iced**:
   - *Pros*: Elm architecture provides clean unidirectional state flow, used at scale by System76 COSMIC desktop.
   - *Cons*: API evolution between versions can require refactoring; widget layout customization can be verbose.

---

## Recommendation & Decision
* **Recommendation**: Proceed with **Slint** or **Iced** as primary candidates for Phase 1 proof-of-concept testing.
* **Phase 0 Decision**: Defer adding GUI crate dependencies until Phase 1 (`TASK-002`), avoiding unneeded compilation overhead in Phase 0.
