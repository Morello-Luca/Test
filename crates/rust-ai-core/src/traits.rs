use serde::{Deserialize, Serialize};
use std::fmt;

/// Loading lifecycle state of a model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModelLoadState {
    #[default]
    Unloaded,
    Loading,
    Loaded(String),
    Error(String),
}

impl fmt::Display for ModelLoadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelLoadState::Unloaded => write!(f, "Unloaded"),
            ModelLoadState::Loading => write!(f, "Loading..."),
            ModelLoadState::Loaded(name) => write!(f, "Loaded: {}", name),
            ModelLoadState::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

/// A single message in a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String,
    pub content: String,
}

impl ConversationMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }
}

/// Common trait for model providers and inference engines.
pub trait ModelProvider {
    /// Returns the provider name (e.g., "llama.cpp Local Engine").
    fn provider_name(&self) -> &str;

    /// Returns the current loading state.
    fn load_state(&self) -> ModelLoadState;

    /// Loads a model from a local file path.
    fn load_model(&mut self, model_path: &std::path::Path) -> Result<(), String>;

    /// Unloads the active model and releases resources.
    fn unload_model(&mut self);

    /// Generates a response for the user prompt.
    fn generate_response(&mut self, prompt: &str) -> Result<String, String>;
}
