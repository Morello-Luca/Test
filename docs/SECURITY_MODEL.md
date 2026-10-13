# SECURITY_MODEL.md — System Security Model

## 1. Threat Model Overview

RustAI interacts with local files, model API keys, terminal environments, and untrusted local/remote AI models. Security is built into the architecture.

```
┌───────────────────────────────────────────────────────────┐
│                      Human Approval                       │
└─────────────────────────────┬─────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────┐
│              Agent Security Sandbox Gate                  │
│       (Allowed Paths, Approved Terminal Commands)         │
└─────────────────────────────┬─────────────────────────────┘
                              │
                              ▼
┌───────────────────────────────────────────────────────────┐
│                Host Filesystem / Terminal                 │
└───────────────────────────────────────────────────────────┘
```

---

## 2. Tool Execution Rules & Sandboxing

* **Explicit Working Directory Boundaries**: Agent file operations (read/write/delete) are restricted to explicit project root paths.
* **Terminal Execution Prompts**: Any destructive command or terminal execution requires explicit user confirmation by default.
* **Path Traversal Protection**: All filesystem inputs must be canonicalized and checked against allowed root scopes.

---

## 3. Secrets & Credential Management

* API keys for remote model providers (OpenAI, Anthropic, etc.) must never be written to application logs, persistent chat histories, or checked into source control.
* Store credentials in system keystores (e.g., Secret Service, Keychain, Credential Manager) or environment variables.
