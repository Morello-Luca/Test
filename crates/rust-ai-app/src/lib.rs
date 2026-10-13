pub mod config;

pub use config::{AppConfig, ThemeMode};

/// Returns the initial status string for the application shell.
pub fn app_status() -> &'static str {
    "RustAI Application Shell - Phase 1 Native Desktop Shell Initialized"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_status() {
        assert_eq!(
            app_status(),
            "RustAI Application Shell - Phase 1 Native Desktop Shell Initialized"
        );
    }
}
