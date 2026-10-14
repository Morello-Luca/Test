use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use rust_ai_core::traits::{ModelLoadState, ModelProvider};
use std::path::Path;

/// llama.cpp local model provider and genuine inference backend.
pub struct LlamaCppBackend {
    backend: Option<LlamaBackend>,
    model: Option<LlamaModel>,
    model_name: Option<String>,
    load_state: ModelLoadState,
}

impl Default for LlamaCppBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LlamaCppBackend {
    pub fn new() -> Self {
        Self {
            backend: None,
            model: None,
            model_name: None,
            load_state: ModelLoadState::Unloaded,
        }
    }
}

impl ModelProvider for LlamaCppBackend {
    fn provider_name(&self) -> &str {
        "llama.cpp Local Backend"
    }

    fn load_state(&self) -> ModelLoadState {
        self.load_state.clone()
    }

    fn load_model(&mut self, model_path: &Path) -> Result<(), String> {
        if !model_path.exists() {
            let err = format!("Model file not found: {}", model_path.display());
            self.load_state = ModelLoadState::Error(err.clone());
            return Err(err);
        }

        self.load_state = ModelLoadState::Loading;

        let file_stem = model_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "gguf_model".to_string());

        let backend = LlamaBackend::init().map_err(|e| {
            let err = format!("Failed to initialize llama.cpp backend: {}", e);
            self.load_state = ModelLoadState::Error(err.clone());
            err
        })?;

        let params = LlamaModelParams::default();

        match LlamaModel::load_from_file(&backend, model_path, &params) {
            Ok(loaded_model) => {
                self.backend = Some(backend);
                self.model = Some(loaded_model);
                self.model_name = Some(file_stem.clone());
                self.load_state = ModelLoadState::Loaded(file_stem);
                Ok(())
            }
            Err(e) => {
                let err = format!(
                    "Failed to load GGUF model from '{}': {}",
                    model_path.display(),
                    e
                );
                self.load_state = ModelLoadState::Error(err.clone());
                Err(err)
            }
        }
    }

    fn unload_model(&mut self) {
        self.model = None;
        self.backend = None;
        self.model_name = None;
        self.load_state = ModelLoadState::Unloaded;
    }

    fn generate_response(&mut self, prompt: &str) -> Result<String, String> {
        match &self.load_state {
            ModelLoadState::Loaded(_name) => {
                if prompt.trim().is_empty() {
                    return Err("Prompt cannot be empty.".to_string());
                }

                let model = self
                    .model
                    .as_ref()
                    .ok_or_else(|| "Model handle missing.".to_string())?;

                let vocab = model.vocab();

                // Create a context for inference
                let ctx_params = LlamaContextParams::default();
                let mut ctx = model
                    .new_context(self.backend.as_ref().unwrap(), ctx_params)
                    .map_err(|e| format!("Failed to create context: {}", e))?;

                // Tokenize prompt using the model's vocabulary
                let tokens = vocab.tokenize(prompt.as_bytes(), true, true);

                if tokens.is_empty() {
                    return Err("Tokenization yielded no tokens.".to_string());
                }

                // Batch evaluation
                let mut batch = LlamaBatch::new(2048, 1);
                let last_index = tokens.len() as i32 - 1;
                for (i, &token) in tokens.iter().enumerate() {
                    let is_last = i as i32 == last_index;
                    batch
                        .add(token, i as i32, &[0], is_last)
                        .map_err(|e| format!("Batch add error: {}", e))?;
                }

                ctx.decode(&mut batch)
                    .map_err(|e| format!("Failed to decode initial batch: {}", e))?;

                let mut sampler = LlamaSampler::greedy();
                let mut output_bytes = Vec::new();
                let mut decoder = encoding_rs::UTF_8.new_decoder();
                let mut output_string = String::new();

                let max_tokens = 256;
                let mut n_cur = tokens.len() as i32;

                for _ in 0..max_tokens {
                    let token = sampler.sample(&ctx, batch.n_tokens() - 1);
                    if vocab.is_eog(token) {
                        break;
                    }

                    let token_bytes = vocab.token_to_piece(token, true, None);

                    output_bytes.extend_from_slice(&token_bytes);
                    let mut temp_str = String::new();
                    let _ = decoder.decode_to_string(&token_bytes, &mut temp_str, false);
                    output_string.push_str(&temp_str);

                    batch.clear();
                    batch
                        .add(token, n_cur, &[0], true)
                        .map_err(|e| format!("Batch add error: {}", e))?;

                    n_cur += 1;
                    ctx.decode(&mut batch)
                        .map_err(|e| format!("Failed to decode token: {}", e))?;
                }

                if output_string.trim().is_empty() {
                    Ok("[Model returned empty response]".to_string())
                } else {
                    Ok(output_string)
                }
            }
            ModelLoadState::Unloaded => {
                Err("No model loaded. Please select and load a local model first.".to_string())
            }
            ModelLoadState::Loading => Err("Model is currently loading. Please wait.".to_string()),
            ModelLoadState::Error(err) => Err(format!(
                "Cannot generate response: Model in error state ({})",
                err
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_initial_state() {
        let backend = LlamaCppBackend::new();
        assert_eq!(backend.provider_name(), "llama.cpp Local Backend");
        assert_eq!(backend.load_state(), ModelLoadState::Unloaded);
    }

    #[test]
    fn test_backend_unloaded_generation_error() {
        let mut backend = LlamaCppBackend::new();
        let res = backend.generate_response("Hello");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "No model loaded. Please select and load a local model first."
        );
    }

    #[test]
    fn test_backend_missing_file_error() {
        let mut backend = LlamaCppBackend::new();
        let res = backend.load_model(Path::new("/nonexistent/model.gguf"));
        assert!(res.is_err());
        match backend.load_state() {
            ModelLoadState::Error(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected error state"),
        }
    }
}
