//! TTS synthesis engine using sherpa-onnx binary
//!
//! Zero Python dependency - uses native sherpa-onnx binary
//! Supports arm64 and x86_64 via universal binary

use crate::error::{BiboError, Result};
use crate::tts::sherpa::{find_sherpa_tts, sherpa_env};
use crate::tts::voice::VoiceCatalog;
use std::path::PathBuf;
use std::process::Command;

/// TTS Engine wrapper (calls sherpa-onnx binary)
pub struct TtsEngine {
    model_dir: PathBuf,
    onnx_file: String,
    voice_id: String,
}

impl TtsEngine {
    /// Create a new TTS engine for the given voice
    pub fn new(voice_id: &str) -> Result<Self> {
        let voice = VoiceCatalog::find(voice_id)
            .ok_or_else(|| BiboError::VoiceNotFound(voice_id.to_string()))?;

        let models_dir = VoiceCatalog::models_dir();
        let model_dir = voice.model_dir_path(&models_dir);

        if !model_dir.exists() {
            return Err(BiboError::VoiceNotInstalled(voice_id.to_string()));
        }

        // Check model onnx file exists
        let model_path = voice.model_path(&models_dir);
        if !model_path.exists() {
            return Err(BiboError::ConfigError(format!(
                "Model file missing for voice: {}",
                voice_id
            )));
        }

        Ok(Self {
            model_dir,
            onnx_file: voice.onnx_file.to_string(),
            voice_id: voice_id.to_string(),
        })
    }

    /// Build sherpa-onnx command with model arguments
    fn build_command(&self, sherpa_path: &PathBuf) -> Result<Command> {
        let mut cmd = Command::new(sherpa_path);

        // Set library path for dynamic libraries
        for (key, value) in sherpa_env() {
            cmd.env(key, value);
        }

        let model_path = self.model_dir.join(&self.onnx_file);
        let tokens_path = self.model_dir.join("tokens.txt");
        let lexicon_path = self.model_dir.join("lexicon.txt");
        let dict_dir = self.model_dir.join("dict");
        let data_dir = self.model_dir.join("espeak-ng-data");

        // Required: model and tokens
        cmd.arg(format!("--vits-model={}", model_path.display()));

        if tokens_path.exists() {
            cmd.arg(format!("--vits-tokens={}", tokens_path.display()));
        }

        // Optional: lexicon
        if lexicon_path.exists() {
            cmd.arg(format!("--vits-lexicon={}", lexicon_path.display()));
        }

        // Optional: dict directory (for MeloTTS Chinese)
        if dict_dir.exists() {
            cmd.arg(format!("--vits-dict-dir={}", dict_dir.display()));
        }

        // Optional: espeak-ng data (for piper models)
        if data_dir.exists() {
            cmd.arg(format!("--vits-data-dir={}", data_dir.display()));
        }

        Ok(cmd)
    }

    /// Synthesize text to WAV file using sherpa-onnx binary
    pub fn synthesize_to_file(
        &self,
        text: &str,
        length_scale: f32,
        output_path: &str,
    ) -> Result<()> {
        let sherpa_path = find_sherpa_tts()?;

        let mut cmd = self.build_command(&sherpa_path)?;

        // Output file and speed (length_scale: larger = slower)
        cmd.arg(format!("--output-filename={}", output_path));
        cmd.arg(format!("--vits-length-scale={:.2}", length_scale));

        // Text as positional argument
        cmd.arg(text);

        let output = cmd
            .output()
            .map_err(|e| BiboError::SynthesisFailed(format!("Failed to run sherpa-onnx: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BiboError::SynthesisFailed(format!(
                "sherpa-onnx error: {}",
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

    /// Get sample rate for audio playback
    pub fn sample_rate(&self) -> u32 {
        // Most sherpa-onnx VITS models use 22050 Hz
        // MeloTTS uses 44100 Hz
        if self.voice_id.to_lowercase().contains("melo") {
            44100
        } else {
            22050
        }
    }
}
