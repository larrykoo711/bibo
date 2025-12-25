//! TTS module - Piper neural text-to-speech

pub mod engine;
pub mod voice;

pub use engine::TtsEngine;
pub use voice::{Voice, VoiceCatalog, VOICE_CATALOG};
