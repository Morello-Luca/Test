use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Application theme options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

/// Named system prompt version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemPromptVersion {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

/// Inference hardware acceleration choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InferenceDevice {
    #[default]
    Cpu,
    Gpu,
}

impl std::fmt::Display for InferenceDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InferenceDevice::Cpu => write!(f, "CPU"),
            InferenceDevice::Gpu => write!(f, "GPU"),
        }
    }
}

/// User settings and application configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: ThemeMode,
    pub models_dir: PathBuf,
    pub default_model_provider: String,
    pub default_model_name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub system_prompts: Vec<SystemPromptVersion>,
    pub active_system_prompt_id: Option<String>,
    pub inference_device: InferenceDevice,
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_prompts = vec![
            SystemPromptVersion {
                id: "default-assistant".to_string(),
                name: "Default Assistant".to_string(),
                prompt: "You are a helpful, respectful, and honest AI assistant.".to_string(),
            },
            SystemPromptVersion {
                id: "coding-assistant".to_string(),
                name: "Code Expert".to_string(),
                prompt: "You are an expert software engineer. Provide concise, accurate code and explanations.".to_string(),
            },
        ];

        Self {
            theme: ThemeMode::System,
            models_dir: PathBuf::from("models"),
            default_model_provider: "llama.cpp".to_string(),
            default_model_name: String::new(),
            window_width: 1280,
            window_height: 800,
            system_prompts: default_prompts,
            active_system_prompt_id: Some("default-assistant".to_string()),
            inference_device: InferenceDevice::Cpu,
        }
    }
}

impl AppConfig {
    /// Returns the active system prompt content if set, or default fallback.
    pub fn active_system_prompt(&self) -> String {
        if let Some(ref active_id) = self.active_system_prompt_id {
            if let Some(found) = self.system_prompts.iter().find(|p| p.id == *active_id) {
                return found.prompt.clone();
            }
        }
        self.system_prompts
            .first()
            .map(|p| p.prompt.clone())
            .unwrap_or_else(|| "You are a helpful AI assistant.".to_string())
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
        assert_eq!(config.models_dir, PathBuf::from("models"));
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 800);
    }

    #[test]
    fn test_config_save_and_load() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join("config.json");

        let config = AppConfig {
            theme: ThemeMode::Dark,
            models_dir: PathBuf::from("/custom/models"),
            default_model_name: "llama-3.gguf".to_string(),
            system_prompts: vec![SystemPromptVersion {
                id: "custom".to_string(),
                name: "Custom".to_string(),
                prompt: "Be precise.".to_string(),
            }],
            active_system_prompt_id: Some("custom".to_string()),
            inference_device: InferenceDevice::Gpu,
            ..AppConfig::default()
        };

        config.save_to_file(&file_path).expect("Save config failed");

        let loaded = AppConfig::load_from_file(&file_path).expect("Load config failed");
        assert_eq!(config, loaded);
        assert_eq!(loaded.theme, ThemeMode::Dark);
        assert_eq!(loaded.models_dir, PathBuf::from("/custom/models"));
        assert_eq!(loaded.default_model_name, "llama-3.gguf");
        assert_eq!(loaded.active_system_prompt(), "Be precise.");
        assert_eq!(loaded.inference_device, InferenceDevice::Gpu);
    }

    #[test]
    fn test_system_prompt_selection_fallback() {
        let mut config = AppConfig {
            active_system_prompt_id: Some("non-existent".to_string()),
            ..Default::default()
        };
        assert_eq!(
            config.active_system_prompt(),
            "You are a helpful, respectful, and honest AI assistant."
        );

        config.system_prompts.clear();
        assert_eq!(
            config.active_system_prompt(),
            "You are a helpful AI assistant."
        );
    }
}
