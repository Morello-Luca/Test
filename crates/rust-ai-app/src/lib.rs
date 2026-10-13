pub mod config;
pub mod gui;

pub use config::{AppConfig, ThemeMode};
pub use gui::RustAiApp;

/// Returns the initial status string for the application shell.
pub fn app_status() -> &'static str {
    "RustAI Application Shell - Phase 1 egui Native Desktop Shell Initialized"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_status() {
        assert_eq!(
            app_status(),
            "RustAI Application Shell - Phase 1 egui Native Desktop Shell Initialized"
        );
    }
}
