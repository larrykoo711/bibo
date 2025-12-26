//! TTS module - Sherpa-onnx neural text-to-speech
//!
//! Zero Python dependency - uses native sherpa-onnx binary
//! Universal binary support for arm64 and x86_64

pub mod engine;
pub mod sherpa;
pub mod voice;

pub use engine::TtsEngine;
pub use sherpa::{find_sherpa_tts, sherpa_available, sherpa_download_url, SHERPA_VERSION};
pub use voice::{Voice, VoiceCatalog, VOICE_CATALOG};
