use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a model file discovered in the models directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveredModel {
    pub name: String,
    pub file_name: String,
    pub path: PathBuf,
    pub size_bytes: u64,
}

/// Handles scanning the local model directory for supported GGUF model files.
#[derive(Debug, Clone, Default)]
pub struct ModelDiscovery;

impl ModelDiscovery {
    /// Scans the given directory for `.gguf` model files.
    pub fn scan_directory(dir_path: &Path) -> Result<Vec<DiscoveredModel>, String> {
        if !dir_path.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(dir_path).map_err(|e| {
            format!(
                "Failed to read models directory '{}': {}",
                dir_path.display(),
                e
            )
        })?;

        let mut models = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "gguf" {
                        let file_name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| file_name.clone());

                        let metadata = fs::metadata(&path).ok();
                        let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);

                        models.push(DiscoveredModel {
                            name,
                            file_name,
                            path,
                            size_bytes,
                        });
                    }
                }
            }
        }

        models.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_scan_directory_empty() {
        let dir = tempdir().expect("temp dir failed");
        let models = ModelDiscovery::scan_directory(dir.path()).expect("scan failed");
        assert!(models.is_empty());
    }

    #[test]
    fn test_scan_directory_with_gguf_files() {
        let dir = tempdir().expect("temp dir failed");
        let model1 = dir.path().join("llama-3-8b.gguf");
        let model2 = dir.path().join("mistral-7b.gguf");
        let non_model = dir.path().join("README.txt");

        fs::write(&model1, b"GGUF_HEADER_1").unwrap();
        fs::write(&model2, b"GGUF_HEADER_2").unwrap();
        fs::write(&non_model, b"Text").unwrap();

        let models = ModelDiscovery::scan_directory(dir.path()).expect("scan failed");
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].name, "llama-3-8b");
        assert_eq!(models[1].name, "mistral-7b");
    }
}
