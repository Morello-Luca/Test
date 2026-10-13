# Configuration and Settings Management Architecture

Configuration in RustAI is handled hierarchically:

1. **Defaults**: Hardcoded safe defaults in `rust-ai-core`.
2. **Configuration File**: User-configurable settings stored in TOML/JSON in standard OS config directories (`XDG_CONFIG_HOME`, `AppData`, `Library/Preferences`).
3. **Environment Variables**: Overrides for development and API keys (e.g. `OPENAI_API_KEY`).

## Security Note
API keys provided in configuration or environment variables are masked in memory and logs.
