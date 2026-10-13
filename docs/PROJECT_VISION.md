# PROJECT_VISION.md — Product Vision and Long-Term Goals

## 1. Executive Summary

RustAI is a native, fast, highly modular desktop application built in Rust. It serves as a unified AI workbench allowing users to interact with local and remote AI models, operate personal AI agents, and execute repository-aware coding workflows with safety, transparency, and high performance.

## 2. Intended Users

* **Software Engineers & Technical Creators**: Developers who require local code assistance, repository understanding, and local model inference without sacrificing privacy or performance.
* **AI Power Users**: Users who manage multiple model providers (Ollama, llama.cpp, OpenAI, Anthropic) and need a responsive native chat interface.
* **Privacy-Conscious Users**: Users who prefer local inference and controlled, explicit permissions for AI tool access.

## 3. Long-Term Objectives

1. **Polished Native Desktop Interface**: High-frequency, responsive chat rendering with full Markdown, math, and code syntax highlighting.
2. **Flexible Inference Layer**: Unified interface supporting both local backends (e.g., llama.cpp / candle / ollama) and remote API providers.
3. **Repository-Aware Coding Agent**: Ability to index local codebases, construct semantic context, propose edits, run tests, and manage Git workflows under human supervision.
4. **Controlled Tool Execution**: Secure sandbox for file operations, terminal execution, and external integrations with fine-grained human approval gates.
5. **AI Continuity Architecture**: Built from the ground up so that AI agents (such as Jules) can continuously develop, maintain, and audit the application over time using structured repository governance.

## 4. Core Guiding Principles

* **Performance & Native Feel**: Zero bloated web-wrapper runtimes. Rust delivers instant startup, low memory usage, and seamless async I/O.
* **Explicit Modular Boundaries**: Strict separation between UI shell, core domain, provider integration, persistence, and agent execution.
* **Verified Reality**: Documentation and tracking reflect only executed, verified code—never hypothetical assumptions.
* **Human Supervision**: High-risk tool execution and code changes always require explicit user opt-in or review.

## 5. Explicit Non-Goals (Out of Scope for Early Phases)

* **Not an All-in-One Cloud SaaS**: RustAI is primarily a client-side desktop application, not a multi-tenant cloud service.
* **Not Unrestricted Autonomous Self-Modification**: The agent may propose and evaluate improvements to its own codebase, but live hot-patching without human review is strictly prohibited.
* **Not a Custom Inference Engine Rewrite**: We leverage mature open-source inference backends and bindings rather than writing custom tensor execution engines from scratch.
