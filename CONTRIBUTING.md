# Contributing to RustAI

Thank you for your interest in contributing to RustAI!

## Development Guidelines

1. **Read `AGENTS.md` and `docs/DEVELOPMENT_WORKFLOW.md`**:
   Whether you are a human or an AI coding agent, the workflows and conventions documented in `docs/` are mandatory.

2. **Licensing**:
   All contributions submitted to RustAI will be dual-licensed under MIT OR Apache-2.0. Do not submit third-party code without verifying license compatibility.

3. **Code Quality & Testing**:
   Before submitting changes, run:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   cargo doc --workspace --no-deps
   python3 scripts/validate_docs.py
   ```

4. **Task Traceability**:
   Every code change must correspond to a task listed in `docs/TASKS.md` and fulfill documented requirements in `docs/REQUIREMENTS.md`.
