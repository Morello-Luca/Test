use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Application theme options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

/// User settings and application configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: ThemeMode,
    pub default_model_provider: String,
    pub default_model_name: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeMode::System,
            default_model_provider: "Ollama".to_string(),
            default_model_name: "llama3".to_string(),
            window_width: 1280,
            window_height: 800,
        }
    }
}

impl AppConfig {
    /// Loads configuration from a JSON file, or returns default if not found.
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config JSON: {}", e))
    }

    /// Saves the configuration to a JSON file.
    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.theme, ThemeMode::System);
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 800);
    }

    #[test]
    fn test_config_save_and_load() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("config.json");

        let config = AppConfig {
            theme: ThemeMode::Dark,
            default_model_name: "mistral".to_string(),
            ..AppConfig::default()
        };

        config.save_to_file(&file_path).expect("Save config failed");

        let loaded = AppConfig::load_from_file(&file_path).expect("Load config failed");
        assert_eq!(config, loaded);
        assert_eq!(loaded.theme, ThemeMode::Dark);
        assert_eq!(loaded.default_model_name, "mistral");
    }
}
