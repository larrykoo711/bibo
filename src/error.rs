//! Error types for Bibo TTS
//!
//! YC/Silicon Valley Standard: Clear, actionable error messages

use colored::Colorize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BiboError {
    #[error("Voice '{0}' not found")]
    VoiceNotFound(String),

    #[error("Voice '{0}' not installed")]
    VoiceNotInstalled(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("Empty file: {0}")]
    EmptyFile(String),

    #[error("No text provided")]
    NoTextProvided,

    #[error("Invalid speed: {0}")]
    InvalidSpeed(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("TTS synthesis failed: {0}")]
    SynthesisFailed(String),

    #[error("Audio playback failed: {0}")]
    PlaybackFailed(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("{0}")]
    Other(String),
}

impl BiboError {
    /// Display formatted error with tips
    pub fn show(&self) {
        eprintln!("\n{} {}", "❌".red(), self.to_string().red().bold());

        let tips = self.tips();
        if !tips.is_empty() {
            eprintln!("\n{}", "💡 How to fix:".yellow());
            for tip in tips {
                eprintln!("   {} {}", "→".cyan(), tip);
            }
        }
        eprintln!();
    }

    /// Get actionable tips for each error type
    fn tips(&self) -> Vec<&str> {
        match self {
            BiboError::VoiceNotFound(_) => vec![
                "bibo -l          # List installed voices",
                "bibo -d list     # Show downloadable voices",
            ],
            BiboError::VoiceNotInstalled(voice) => vec![
                Box::leak(format!("bibo -d {}  # Download this voice", voice).into_boxed_str()),
                "bibo -d list     # Show all downloadable voices",
            ],
            BiboError::FileNotFound(_) => vec![
                "Check the file path for typos",
                "Use absolute path: bibo -i /full/path/to/file.md",
            ],
            BiboError::UnsupportedFileType(_) => vec![
                "bibo -i file.md   # Markdown files",
                "bibo -i file.txt  # Text files",
                "bibo \"text\"       # Or just pass text directly",
            ],
            BiboError::EmptyFile(_) => vec![
                "Check if the file contains text content",
                "For Markdown: ensure text outside code blocks",
            ],
            BiboError::NoTextProvided => vec![
                "bibo \"Hello world\"     # Direct text",
                "bibo -i README.md      # From file",
            ],
            BiboError::InvalidSpeed(_) => vec![
                "bibo \"text\" -s slow   # Slow speed",
                "bibo \"text\" -s normal # Normal speed",
                "bibo \"text\" -s fast   # Fast speed",
                "bibo \"text\" -f        # Fast mode shortcut",
            ],
            BiboError::DownloadFailed(_) => vec![
                "Check your internet connection",
                "Try again later",
                "Use VPN if HuggingFace is blocked",
            ],
            BiboError::SynthesisFailed(_) | BiboError::PlaybackFailed(_) => vec![
                "Check if voice model is valid",
                "bibo -d <voice>  # Re-download the voice",
            ],
            BiboError::ConfigError(_) | BiboError::Other(_) => vec!["bibo --help  # Show usage"],
        }
    }
}

pub type Result<T> = std::result::Result<T, BiboError>;
