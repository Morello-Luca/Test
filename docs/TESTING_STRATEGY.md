# TESTING_STRATEGY.md — Testing & Quality Strategy

## 1. Testing Pyramid

RustAI employs a multi-tiered testing strategy to ensure reliability across modules and AI agent workflows:

```
          ┌───────────────────────────┐
          │  System & Integration     │  (End-to-End, Provider Fakes)
          ├───────────────────────────┤
          │  Module & Unit Tests      │  (Domain logic, State, Parsing)
          ├───────────────────────────┤
          │  Automated Quality Gates  │  (fmt, clippy, doc validation)
          └───────────────────────────┘
```

---

## 2. Test Organization

* **Unit Tests**: Placed within the same file as the code under test inside a `#[cfg(test)] mod tests` block.
* **Integration Tests**: Placed in `tests/` directory under crate roots for testing public crate interfaces.
* **Documentation Validation**: Automated Python validator script `scripts/validate_docs.py` executed in CI to ensure repository link integrity and requirement traceability.

---

## 3. Mandatory Requirements for Task Verification

A task is considered `VERIFIED` only when:
1. `cargo test --workspace` passes with zero failures.
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings` executes without warnings.
3. `cargo fmt --all -- --check` passes cleanly.
4. `python3 scripts/validate_docs.py` succeeds.
