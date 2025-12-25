//! Voice catalog and management
//!
//! Curated selection of high-quality Piper voices

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Voice metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub id: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub gender: char,
    pub quality: &'static str,
    pub size_mb: u32,
    pub hf_path: &'static str,
}

impl Voice {
    /// Get the model filename
    pub fn model_filename(&self) -> String {
        format!("{}.onnx", self.hf_path.split('/').last().unwrap_or(self.id))
    }

    /// Get the config filename
    pub fn config_filename(&self) -> String {
        format!(
            "{}.onnx.json",
            self.hf_path.split('/').last().unwrap_or(self.id)
        )
    }
}

/// Curated voice catalog - Top voices from Piper
pub static VOICE_CATALOG: &[Voice] = &[
    // English - US
    Voice {
        id: "amy",
        name: "Amy",
        lang: "en_US",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        hf_path: "en/en_US/amy/medium/en_US-amy-medium",
    },
    Voice {
        id: "lessac",
        name: "Lessac",
        lang: "en_US",
        gender: 'F',
        quality: "high",
        size_mb: 120,
        hf_path: "en/en_US/lessac/high/en_US-lessac-high",
    },
    Voice {
        id: "ryan",
        name: "Ryan",
        lang: "en_US",
        gender: 'M',
        quality: "high",
        size_mb: 120,
        hf_path: "en/en_US/ryan/high/en_US-ryan-high",
    },
    Voice {
        id: "joe",
        name: "Joe",
        lang: "en_US",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        hf_path: "en/en_US/joe/medium/en_US-joe-medium",
    },
    // English - GB
    Voice {
        id: "alan",
        name: "Alan",
        lang: "en_GB",
        gender: 'M',
        quality: "medium",
        size_mb: 45,
        hf_path: "en/en_GB/alan/medium/en_GB-alan-medium",
    },
    Voice {
        id: "alba",
        name: "Alba",
        lang: "en_GB",
        gender: 'F',
        quality: "medium",
        size_mb: 45,
        hf_path: "en/en_GB/alba/medium/en_GB-alba-medium",
    },
    // German
    Voice {
        id: "thorsten",
        name: "Thorsten",
        lang: "de_DE",
        gender: 'M',
        quality: "high",
        size_mb: 120,
        hf_path: "de/de_DE/thorsten/high/de_DE-thorsten-high",
    },
    // French
    Voice {
        id: "siwis",
        name: "Siwis",
        lang: "fr_FR",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        hf_path: "fr/fr_FR/siwis/medium/fr_FR-siwis-medium",
    },
    // Chinese
    Voice {
        id: "huayan",
        name: "Huayan",
        lang: "zh_CN",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        hf_path: "zh/zh_CN/huayan/medium/zh_CN-huayan-medium",
    },
    // Spanish
    Voice {
        id: "davefx",
        name: "DaveFX",
        lang: "es_ES",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        hf_path: "es/es_ES/davefx/medium/es_ES-davefx-medium",
    },
    // Russian
    Voice {
        id: "irina",
        name: "Irina",
        lang: "ru_RU",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        hf_path: "ru/ru_RU/irina/medium/ru_RU-irina-medium",
    },
    Voice {
        id: "ruslan",
        name: "Ruslan",
        lang: "ru_RU",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        hf_path: "ru/ru_RU/ruslan/medium/ru_RU-ruslan-medium",
    },
];

/// Voice catalog operations
pub struct VoiceCatalog;

impl VoiceCatalog {
    /// Get models directory
    pub fn models_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("bibo")
            .join("models")
    }

    /// Find voice by ID in catalog
    pub fn find(id: &str) -> Option<&'static Voice> {
        VOICE_CATALOG.iter().find(|v| v.id.eq_ignore_ascii_case(id))
    }

    /// List installed voices
    pub fn installed() -> Vec<String> {
        let models_dir = Self::models_dir();
        if !models_dir.exists() {
            return vec![];
        }

        std::fs::read_dir(&models_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .map(|ext| ext == "onnx")
                            .unwrap_or(false)
                    })
                    .filter_map(|e| {
                        e.path()
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if voice is installed
    pub fn is_installed(id: &str) -> bool {
        let installed = Self::installed();
        installed
            .iter()
            .any(|v| v.to_lowercase().contains(&id.to_lowercase()))
    }

    /// Get model path for a voice ID
    pub fn model_path(id: &str) -> Option<PathBuf> {
        let voice = Self::find(id)?;
        let models_dir = Self::models_dir();
        let model_file = models_dir.join(voice.model_filename());

        if model_file.exists() {
            Some(model_file)
        } else {
            None
        }
    }
}
