//! Bibo - Fast, local neural text-to-speech
//!
//! Built with Silicon Valley standards: simple, fast, powerful

mod audio;
mod cli;
mod download;
mod error;
mod tts;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
use download::VoiceDownloader;
use error::BiboError;
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

/// Clean markdown formatting for TTS
fn clean_markdown(text: &str) -> String {
    let mut text = text.to_string();

    // Remove code blocks
    text = regex_lite::Regex::new(r"```[\s\S]*?```")
        .unwrap()
        .replace_all(&text, "")
        .to_string();

    // Remove inline code
    text = regex_lite::Regex::new(r"`([^`]+)`")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();

    // Remove images
    text = regex_lite::Regex::new(r"!\[([^\]]*)\]\([^)]+\)")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();

    // Convert links to text
    text = regex_lite::Regex::new(r"\[([^\]]+)\]\([^)]+\)")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();

    // Remove headers
    text = regex_lite::Regex::new(r"^#{1,6}\s+")
        .unwrap()
        .replace_all(&text, "")
        .to_string();

    // Remove bold/italic
    text = regex_lite::Regex::new(r"\*\*\*([^*]+)\*\*\*")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();
    text = regex_lite::Regex::new(r"\*\*([^*]+)\*\*")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();
    text = regex_lite::Regex::new(r"\*([^*]+)\*")
        .unwrap()
        .replace_all(&text, "$1")
        .to_string();

    // Remove list markers
    text = regex_lite::Regex::new(r"^\s*[-*+]\s+")
        .unwrap()
        .replace_all(&text, "")
        .to_string();
    text = regex_lite::Regex::new(r"^\s*\d+\.\s+")
        .unwrap()
        .replace_all(&text, "")
        .to_string();

    // Clean up whitespace
    text = regex_lite::Regex::new(r"\n{3,}")
        .unwrap()
        .replace_all(&text, "\n\n")
        .to_string();

    text.trim().to_string()
}

/// Read content from file
fn read_file_content(path: &str, quiet: bool) -> Result<String, BiboError> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(BiboError::FileNotFound(path.display().to_string()));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !["md", "txt", "markdown"].contains(&ext.as_str()) {
        return Err(BiboError::UnsupportedFileType(ext));
    }

    let content = fs::read_to_string(path)
        .map_err(|e| BiboError::FileNotFound(format!("{}: {}", path.display(), e)))?;

    if !quiet {
        println!(
            "{} Reading: {} ({} chars)",
            "📄".cyan(),
            path.file_name().unwrap_or_default().to_string_lossy(),
            content.len()
        );
    }

    // Clean markdown
    let content = if ext == "md" || ext == "markdown" {
        let cleaned = clean_markdown(&content);
        if !quiet {
            println!("{} Cleaned: {} chars", "📝".cyan(), cleaned.len());
        }
        cleaned
    } else {
        content
    };

    if content.trim().is_empty() {
        return Err(BiboError::EmptyFile(path.display().to_string()));
    }

    Ok(content)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Download mode
    if let Some(spec) = &cli.download {
        match VoiceDownloader::download_by_spec(spec, cli.quiet).await {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                e.show();
                std::process::exit(1);
            }
        }
    }

    // List voices mode
    if cli.list {
        let voices = tts::voice::VoiceCatalog::installed();
        if voices.is_empty() {
            println!("{} No voices installed", "⚠️".yellow());
            println!("{} Download: bibo -d list", "📥".cyan());
        } else {
            println!("{}", "📢 Installed voices:".cyan().bold());
            for v in &voices {
                let prefix = if v.to_lowercase().contains(&cli.voice.to_lowercase()) {
                    "→"
                } else {
                    " "
                };
                println!("  {} {}", prefix, v);
            }
            println!("\n{} Download more: bibo -d list", "💡".yellow());
        }
        std::process::exit(0);
    }

    // Get text input
    let text = if let Some(input_file) = &cli.input {
        match read_file_content(input_file, cli.quiet) {
            Ok(content) => content,
            Err(e) => {
                e.show();
                std::process::exit(1);
            }
        }
    } else if let Some(text) = &cli.text {
        text.clone()
    } else {
        BiboError::NoTextProvided.show();
        std::process::exit(1);
    };

    // Get speed
    let speed = cli.effective_speed();
    let length_scale = speed.to_length_scale();

    // Create TTS engine
    let engine = match tts::TtsEngine::new(&cli.voice) {
        Ok(e) => e,
        Err(e) => {
            e.show();
            std::process::exit(1);
        }
    };

    if !cli.quiet {
        let speed_name = format!("{:?}", speed).to_lowercase();
        println!("{} {} @ {}", "🎤".cyan(), cli.voice, speed_name);
    }

    // Synthesize
    match engine.synthesize(&text, length_scale) {
        Ok(samples) => {
            // Output to file or play
            if let Some(output_path) = &cli.output {
                // Save to file
                if let Err(e) = engine.synthesize_to_file(&text, length_scale, output_path) {
                    e.show();
                    std::process::exit(1);
                }
                if !cli.quiet {
                    println!("{} Saved: {}", "✅".green(), output_path);
                }
            } else {
                // Play audio
                if !cli.quiet {
                    println!("{} Playing...", "▶️".cyan());
                }
                if let Err(e) = audio::AudioPlayer::play_samples(samples, 22050) {
                    e.show();
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            e.show();
            std::process::exit(1);
        }
    }
}
