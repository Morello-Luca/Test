# Session Handoff Template

- **Session Date**: YYYY-MM-DD
- **Task ID**: TASK-XXX
- **Objective**: Session objective summary

## Changes Made
- List of files modified/created.

## Modules Affected
- `rust-ai-xxx`

## Architectural Decisions
- Any ADRs referenced or created.

## Executed Commands & Verifications
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
python3 scripts/validate_docs.py
```

## Known Problems & Limitations
- Outstanding issues or technical debt noted.

## Outstanding Work & Recommended Next Step
- Exact task and instructions for the next agent session.
