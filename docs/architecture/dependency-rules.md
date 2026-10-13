# Dependency Rules and Module Boundaries

To ensure clean architecture and modular maintainability, RustAI enforces strict dependency rules:

```
┌───────────────────────────────────────────────────────────┐
│                    rust-ai-app (UI Shell)                  │
└─────────────────────────────┬─────────────────────────────┘
                              │ Depends on core
                              ▼
┌───────────────────────────────────────────────────────────┐
│                  rust-ai-core (Domain Core)                │
└──────────────┬──────────────────────────────┬─────────────┘
               │                              │
               │ Abstract Traits              │ Abstract Traits
               ▼                              ▼
┌─────────────────────────────┐ ┌───────────────────────────┐
│     rust-ai-providers       │ │       rust-ai-tools       │
│     (Provider Adapters)     │ │   (Tool Sandbox Engine)   │
└─────────────────────────────┘ └───────────────────────────┘
```

## Non-Negotiable Boundary Rules
1. `rust-ai-core` must **NEVER** depend on `rust-ai-app` or specific UI frameworks.
2. Provider adapters and tool crates depend on traits exposed in `rust-ai-core`.
3. Circular dependencies between workspace crates are strictly prohibited.
