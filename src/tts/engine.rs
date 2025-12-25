//! TTS synthesis engine using Python piper-tts as backend
//!
//! Rust CLI wrapper + Python TTS engine hybrid approach

use crate::error::{BiboError, Result};
use crate::tts::voice::VoiceCatalog;
use std::path::PathBuf;
use std::process::Command;

/// TTS Engine wrapper (calls Python piper-tts)
pub struct TtsEngine {
    model_path: PathBuf,
    config_path: PathBuf,
}

impl TtsEngine {
    /// Create a new TTS engine for the given voice
    pub fn new(voice_id: &str) -> Result<Self> {
        let voice = VoiceCatalog::find(voice_id)
            .ok_or_else(|| BiboError::VoiceNotFound(voice_id.to_string()))?;

        let models_dir = VoiceCatalog::models_dir();
        let model_path = models_dir.join(voice.model_filename());
        let config_path = models_dir.join(voice.config_filename());

        if !model_path.exists() {
            return Err(BiboError::VoiceNotInstalled(voice_id.to_string()));
        }

        if !config_path.exists() {
            return Err(BiboError::ConfigError(format!(
                "Config file missing for voice: {}",
                voice_id
            )));
        }

        Ok(Self {
            model_path,
            config_path,
        })
    }

    /// Get Python command (uv run or system python)
    fn python_cmd() -> Vec<String> {
        // Try uv first
        if Command::new("uv").arg("--version").output().is_ok() {
            vec!["uv".to_string(), "run".to_string(), "python".to_string()]
        } else {
            vec!["python3".to_string()]
        }
    }

    /// Synthesize text to WAV file using Python piper-tts
    pub fn synthesize_to_file(
        &self,
        text: &str,
        length_scale: f32,
        output_path: &str,
    ) -> Result<()> {
        let python_cmd = Self::python_cmd();

        // Python script to synthesize
        let script = format!(
            r#"
import sys
import wave
from piper.voice import PiperVoice
from piper.config import SynthesisConfig

voice = PiperVoice.load("{model}", "{config}")
syn_config = SynthesisConfig(length_scale={length_scale})

with wave.open("{output}", "wb") as wav_file:
    voice.synthesize_wav('''{text}''', wav_file, syn_config=syn_config)
"#,
            model = self.model_path.display(),
            config = self.config_path.display(),
            length_scale = length_scale,
            output = output_path,
            text = text.replace("'''", r"\'\'\'"),
        );

        let mut cmd = Command::new(&python_cmd[0]);
        for arg in &python_cmd[1..] {
            cmd.arg(arg);
        }
        cmd.arg("-c").arg(&script);

        let output = cmd
            .output()
            .map_err(|e| BiboError::SynthesisFailed(format!("Failed to run Python: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BiboError::SynthesisFailed(format!(
                "Python error: {}",
                stderr
            )));
        }

        Ok(())
    }

    /// Synthesize text and return audio samples (reads from temp file)
    pub fn synthesize(&self, text: &str, length_scale: f32) -> Result<Vec<i16>> {
        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| BiboError::Other(format!("Failed to create temp file: {}", e)))?;

        let temp_path = temp_file.path().to_str().unwrap();

        // Use file extension for WAV
        let wav_path = format!("{}.wav", temp_path);

        self.synthesize_to_file(text, length_scale, &wav_path)?;

        // Read WAV samples
        let reader = hound::WavReader::open(&wav_path)
            .map_err(|e| BiboError::Other(format!("Failed to read WAV: {}", e)))?;

        let samples: Vec<i16> = reader
            .into_samples::<i16>()
            .filter_map(|s| s.ok())
            .collect();

        // Clean up
        let _ = std::fs::remove_file(&wav_path);

        Ok(samples)
    }
}
