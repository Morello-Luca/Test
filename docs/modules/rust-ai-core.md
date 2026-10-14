# Module Specification: `rust-ai-core`

- **Crate**: `rust-ai-core`
- **Path**: `crates/rust-ai-core`
- **Status**: VERIFIED

## Purpose
Defines primary domain models, model discovery services (`ModelDiscovery`), conversation structs (`ConversationMessage`), and core provider traits (`ModelProvider`, `ModelLoadState`).

## Public Interface
- `ModelDiscovery::scan_directory(path: &Path) -> Result<Vec<DiscoveredModel>, String>`
- `pub trait ModelProvider`
- `pub enum ModelLoadState`

## Verification & Tests
Unit tests in `crates/rust-ai-core/src/discovery.rs` verify `.gguf` file discovery in temporary directories.
