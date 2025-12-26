//! Voice catalog and management
//!
//! Curated selection of high-quality sherpa-onnx voices

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Voice metadata for sherpa-onnx models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub id: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub gender: char,
    pub quality: &'static str,
    pub size_mb: u32,
    /// Model directory name in sherpa-onnx releases
    pub model_dir: &'static str,
    /// Download URL for the model
    pub download_url: &'static str,
}

impl Voice {
    /// Get the model directory path
    pub fn model_dir_path(&self, base: &PathBuf) -> PathBuf {
        base.join(self.model_dir)
    }

    /// Get the model.onnx path
    pub fn model_path(&self, base: &PathBuf) -> PathBuf {
        self.model_dir_path(base).join("model.onnx")
    }

    /// Get the tokens.txt path
    pub fn tokens_path(&self, base: &PathBuf) -> PathBuf {
        self.model_dir_path(base).join("tokens.txt")
    }

    /// Get the lexicon.txt path (optional, for some models)
    pub fn lexicon_path(&self, base: &PathBuf) -> PathBuf {
        self.model_dir_path(base).join("lexicon.txt")
    }

    /// Get the dict_dir path (for Chinese models)
    pub fn dict_dir(&self, base: &PathBuf) -> PathBuf {
        self.model_dir_path(base).join("dict")
    }

    /// Check if this is a MeloTTS model (Chinese+English)
    pub fn is_melo(&self) -> bool {
        self.model_dir.contains("melo")
    }
}

/// Curated voice catalog - Top voices from sherpa-onnx
///
/// Model sources: https://github.com/k2-fsa/sherpa-onnx/releases/tag/tts-models
pub static VOICE_CATALOG: &[Voice] = &[
    // Chinese + English bilingual (MeloTTS)
    Voice {
        id: "melo",
        name: "MeloTTS",
        lang: "zh_en",
        gender: 'F',
        quality: "high",
        size_mb: 150,
        model_dir: "vits-melo-tts-zh_en",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-melo-tts-zh_en.tar.bz2",
    },
    // Chinese only
    Voice {
        id: "huayan",
        name: "Huayan",
        lang: "zh_CN",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-zh_CN-huayan-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-zh_CN-huayan-medium.tar.bz2",
    },
    Voice {
        id: "aishell3",
        name: "AIShell3",
        lang: "zh_CN",
        gender: 'F',
        quality: "high",
        size_mb: 100,
        model_dir: "vits-zh-aishell3",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-zh-aishell3.tar.bz2",
    },
    // Korean
    Voice {
        id: "kss",
        name: "KSS",
        lang: "ko_KR",
        gender: 'F',
        quality: "low",
        size_mb: 30,
        model_dir: "vits-mimic3-ko_KO-kss_low",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-mimic3-ko_KO-kss_low.tar.bz2",
    },
    // English - US
    Voice {
        id: "amy",
        name: "Amy",
        lang: "en_US",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-en_US-amy-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_US-amy-medium.tar.bz2",
    },
    Voice {
        id: "lessac",
        name: "Lessac",
        lang: "en_US",
        gender: 'F',
        quality: "high",
        size_mb: 120,
        model_dir: "vits-piper-en_US-lessac-high",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_US-lessac-high.tar.bz2",
    },
    Voice {
        id: "ryan",
        name: "Ryan",
        lang: "en_US",
        gender: 'M',
        quality: "high",
        size_mb: 120,
        model_dir: "vits-piper-en_US-ryan-high",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_US-ryan-high.tar.bz2",
    },
    Voice {
        id: "joe",
        name: "Joe",
        lang: "en_US",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-en_US-joe-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_US-joe-medium.tar.bz2",
    },
    Voice {
        id: "ljspeech",
        name: "LJSpeech",
        lang: "en_US",
        gender: 'F',
        quality: "high",
        size_mb: 80,
        model_dir: "vits-ljs",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-ljs.tar.bz2",
    },
    // English - GB
    Voice {
        id: "alan",
        name: "Alan",
        lang: "en_GB",
        gender: 'M',
        quality: "medium",
        size_mb: 45,
        model_dir: "vits-piper-en_GB-alan-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_GB-alan-medium.tar.bz2",
    },
    Voice {
        id: "alba",
        name: "Alba",
        lang: "en_GB",
        gender: 'F',
        quality: "medium",
        size_mb: 45,
        model_dir: "vits-piper-en_GB-alba-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_GB-alba-medium.tar.bz2",
    },
    // German
    Voice {
        id: "thorsten",
        name: "Thorsten",
        lang: "de_DE",
        gender: 'M',
        quality: "high",
        size_mb: 120,
        model_dir: "vits-piper-de_DE-thorsten-high",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-de_DE-thorsten-high.tar.bz2",
    },
    // French
    Voice {
        id: "siwis",
        name: "Siwis",
        lang: "fr_FR",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-fr_FR-siwis-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-fr_FR-siwis-medium.tar.bz2",
    },
    // Spanish
    Voice {
        id: "davefx",
        name: "DaveFX",
        lang: "es_ES",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-es_ES-davefx-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-es_ES-davefx-medium.tar.bz2",
    },
    // Russian
    Voice {
        id: "irina",
        name: "Irina",
        lang: "ru_RU",
        gender: 'F',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-ru_RU-irina-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-ru_RU-irina-medium.tar.bz2",
    },
    Voice {
        id: "ruslan",
        name: "Ruslan",
        lang: "ru_RU",
        gender: 'M',
        quality: "medium",
        size_mb: 60,
        model_dir: "vits-piper-ru_RU-ruslan-medium",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-ru_RU-ruslan-medium.tar.bz2",
    },
    // Vietnamese
    Voice {
        id: "vais",
        name: "VAIS1000",
        lang: "vi_VN",
        gender: 'F',
        quality: "low",
        size_mb: 30,
        model_dir: "vits-mimic3-vi_VN-vais1000_low",
        download_url: "https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-mimic3-vi_VN-vais1000_low.tar.bz2",
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

    /// Get default voice
    pub fn default_voice() -> &'static Voice {
        // Default to MeloTTS (Chinese+English)
        Self::find("melo").unwrap_or(&VOICE_CATALOG[0])
    }

    /// List installed voices
    pub fn installed() -> Vec<String> {
        let models_dir = Self::models_dir();
        if !models_dir.exists() {
            return vec![];
        }

        // Look for model directories that contain model.onnx
        std::fs::read_dir(&models_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .filter(|e| e.path().join("model.onnx").exists())
                    .filter_map(|e| {
                        e.path()
                            .file_name()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if voice is installed
    pub fn is_installed(id: &str) -> bool {
        if let Some(voice) = Self::find(id) {
            let models_dir = Self::models_dir();
            let model_path = voice.model_path(&models_dir);
            model_path.exists()
        } else {
            // Check by directory name match
            let installed = Self::installed();
            installed
                .iter()
                .any(|v| v.to_lowercase().contains(&id.to_lowercase()))
        }
    }

    /// Get model directory path for a voice ID
    pub fn model_dir_path(id: &str) -> Option<PathBuf> {
        let voice = Self::find(id)?;
        let models_dir = Self::models_dir();
        let model_dir = voice.model_dir_path(&models_dir);

        if model_dir.exists() {
            Some(model_dir)
        } else {
            None
        }
    }
}
