# System Overview Architecture Deep-Dive

This document provides a detailed breakdown of the high-level architecture outlined in `docs/ARCHITECTURE.md`.

## Core Subsystems

1. **UI Shell (`rust-ai-app`)**:
   - Manages OS window events, user inputs, and visual rendering.
   - Communicates with `rust-ai-core` asynchronously via MPSC event channels.

2. **Domain Core (`rust-ai-core`)**:
   - Manages active conversation sessions, message histories, and agent state machines.
   - Holds core traits for model providers and tools.

3. **Inference & Providers (`rust-ai-providers`)**:
   - Implements async adapters for local model execution engines (llama.cpp / Ollama) and remote APIs (OpenAI / Anthropic).

4. **Agent Sandbox & Tools (`rust-ai-tools`)**:
   - Sandboxed tool invocation engine supporting path canonicalization, command execution approval gates, and file operations.
