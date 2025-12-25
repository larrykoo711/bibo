//! Voice download module with fallback mirrors

use crate::error::{BiboError, Result};
use crate::tts::voice::{Voice, VoiceCatalog, VOICE_CATALOG};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Download sources (primary + fallback)
const DOWNLOAD_SOURCES: &[(&str, &str)] = &[
    (
        "huggingface",
        "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0",
    ),
    (
        "hf_mirror",
        "https://hf-mirror.com/rhasspy/piper-voices/resolve/v1.0.0",
    ),
];

/// Voice downloader
pub struct VoiceDownloader;

impl VoiceDownloader {
    /// Show available voices for download
    pub fn show_catalog() {
        let installed = VoiceCatalog::installed();

        println!("\n{}", "📦 Available voices for download:".cyan().bold());
        println!();
        println!(
            "{:<3} {:<10} {:<10} {:<6} {:<3} {:<7} {:<6} {}",
            "#", "ID", "Name", "Lang", "G", "Quality", "Size", "Status"
        );
        println!("{}", "─".repeat(70));

        for (idx, voice) in VOICE_CATALOG.iter().enumerate() {
            let is_installed = installed
                .iter()
                .any(|v| v.to_lowercase().contains(voice.id));
            let status = if is_installed {
                "✅ installed".green().to_string()
            } else {
                String::new()
            };

            println!(
                "{:<3} {:<10} {:<10} {:<6} {:<3} {:<7} {}MB  {}",
                idx + 1,
                voice.id,
                voice.name,
                voice.lang,
                voice.gender,
                voice.quality,
                voice.size_mb,
                status
            );
        }

        println!();
        println!("{}", "💡 Usage:".yellow());
        println!("   bibo -d <id>        Download single voice");
        println!("   bibo -d all         Download all voices");
        println!("   bibo -d 1,3,5       Download by numbers");
    }

    /// Download a voice by ID
    pub async fn download_voice(voice_id: &str, quiet: bool) -> Result<bool> {
        let voice = VoiceCatalog::find(voice_id)
            .ok_or_else(|| BiboError::VoiceNotFound(voice_id.to_string()))?;

        let models_dir = VoiceCatalog::models_dir();
        tokio::fs::create_dir_all(&models_dir)
            .await
            .map_err(|e| BiboError::Other(format!("Failed to create models dir: {}", e)))?;

        // Check if already installed
        let model_file = models_dir.join(voice.model_filename());
        if model_file.exists() {
            if !quiet {
                println!(
                    "{} {} ({}) already installed",
                    "✅".green(),
                    voice.name,
                    voice.lang
                );
            }
            return Ok(true);
        }

        if !quiet {
            println!(
                "\n{} Downloading: {} ({}, {}, {}, ~{}MB)",
                "📥".cyan(),
                voice.name,
                voice.lang,
                voice.gender,
                voice.quality,
                voice.size_mb
            );
        }

        // Try each source
        for (source_name, base_url) in DOWNLOAD_SOURCES {
            if !quiet {
                println!("   Source: {}", source_name);
            }

            let success = Self::download_from_source(voice, base_url, &models_dir, quiet).await;

            if success {
                if !quiet {
                    println!("{} {} installed successfully!", "✅".green(), voice.name);
                }
                return Ok(true);
            } else if !quiet {
                println!(
                    "   {} {} failed, trying next...",
                    "⚠️".yellow(),
                    source_name
                );
            }
        }

        Err(BiboError::DownloadFailed(voice.name.to_string()))
    }

    /// Download from a specific source
    async fn download_from_source(
        voice: &Voice,
        base_url: &str,
        models_dir: &PathBuf,
        quiet: bool,
    ) -> bool {
        for ext in [".onnx", ".onnx.json"] {
            let url = format!("{}/{}{}", base_url, voice.hf_path, ext);
            let filename = format!("{}{}", voice.hf_path.split('/').last().unwrap(), ext);
            let dest = models_dir.join(&filename);

            if !quiet {
                let file_type = if ext == ".onnx" { "model" } else { "config" };
                print!("   📄 {}...", file_type);
            }

            match Self::download_file(&url, &dest, quiet).await {
                Ok(_) => {
                    if !quiet {
                        println!(" ✓");
                    }
                }
                Err(_) => {
                    if !quiet {
                        println!(" ❌");
                    }
                    // Clean up partial download
                    let _ = tokio::fs::remove_file(&dest).await;
                    return false;
                }
            }
        }

        true
    }

    /// Download a single file with progress
    async fn download_file(url: &str, dest: &PathBuf, quiet: bool) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "Bibo-TTS/1.0")
            .send()
            .await
            .map_err(|e| BiboError::DownloadFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(BiboError::DownloadFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let total_size = response.content_length().unwrap_or(0);

        // Create progress bar
        let pb = if !quiet && total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("   [{bar:20.cyan/blue}] {percent}%")
                    .unwrap()
                    .progress_chars("█░"),
            );
            Some(pb)
        } else {
            None
        };

        // Download with streaming
        let mut file = File::create(dest)
            .await
            .map_err(|e| BiboError::DownloadFailed(format!("Failed to create file: {}", e)))?;

        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;

        while let Some(chunk) = stream.next().await {
            let chunk =
                chunk.map_err(|e| BiboError::DownloadFailed(format!("Stream error: {}", e)))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| BiboError::DownloadFailed(format!("Write error: {}", e)))?;

            downloaded += chunk.len() as u64;
            if let Some(ref pb) = pb {
                pb.set_position(downloaded);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        Ok(())
    }

    /// Download voices by specification
    pub async fn download_by_spec(spec: &str, quiet: bool) -> Result<usize> {
        let spec = spec.to_lowercase();

        // Show catalog
        if spec == "list" {
            Self::show_catalog();
            return Ok(0);
        }

        // Download all
        if spec == "all" {
            if !quiet {
                println!("{}", "📦 Downloading all voices...".cyan());
            }
            let mut success = 0;
            for voice in VOICE_CATALOG {
                if Self::download_voice(voice.id, quiet).await.is_ok() {
                    success += 1;
                }
            }
            if !quiet {
                println!(
                    "\n{} Downloaded {}/{} voices",
                    "✅".green(),
                    success,
                    VOICE_CATALOG.len()
                );
            }
            return Ok(success);
        }

        // Download by numbers (e.g., "1,3,5")
        if spec.contains(',') || spec.chars().all(|c| c.is_ascii_digit()) {
            let indices: Vec<usize> = spec
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect();

            let mut success = 0;
            for idx in indices {
                if idx >= 1 && idx <= VOICE_CATALOG.len() {
                    let voice = &VOICE_CATALOG[idx - 1];
                    if Self::download_voice(voice.id, quiet).await.is_ok() {
                        success += 1;
                    }
                } else if !quiet {
                    println!(
                        "{} Invalid number: {} (valid: 1-{})",
                        "⚠️".yellow(),
                        idx,
                        VOICE_CATALOG.len()
                    );
                }
            }
            return Ok(success);
        }

        // Download by ID
        Self::download_voice(&spec, quiet).await.map(|_| 1)
    }
}
