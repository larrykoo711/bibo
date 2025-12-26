//! Download module - voices and sherpa-onnx binary

pub mod sherpa;

use crate::error::{BiboError, Result};
use crate::tts::voice::{Voice, VoiceCatalog, VOICE_CATALOG};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
pub use sherpa::SherpaDownloader;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Voice downloader for sherpa-onnx models
pub struct VoiceDownloader;

impl VoiceDownloader {
    /// Show available voices for download
    pub fn show_catalog() {
        let installed = VoiceCatalog::installed();

        println!("\n{}", "📦 Available voices for download:".cyan().bold());
        println!();
        println!(
            "{:<3} {:<12} {:<12} {:<8} {:<3} {:<7} {:<6} {}",
            "#", "ID", "Name", "Lang", "G", "Quality", "Size", "Status"
        );
        println!("{}", "─".repeat(75));

        for (idx, voice) in VOICE_CATALOG.iter().enumerate() {
            let is_installed = installed
                .iter()
                .any(|v| v.to_lowercase().contains(&voice.model_dir.to_lowercase()));
            let status = if is_installed {
                "✅ installed".green().to_string()
            } else {
                String::new()
            };

            println!(
                "{:<3} {:<12} {:<12} {:<8} {:<3} {:<7} {}MB  {}",
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
        println!();
        println!("{}", "🌍 Languages:".yellow());
        println!("   melo    - Chinese + English bilingual (recommended)");
        println!("   kss     - Korean");
        println!("   amy     - English (US)");
        println!("   huayan  - Chinese");
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
        let model_path = voice.model_path(&models_dir);
        if model_path.exists() {
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

        // Download tar.bz2 from sherpa-onnx releases
        let temp_tar = models_dir.join(format!("{}.tar.bz2", voice.model_dir));

        if !quiet {
            println!("   Source: sherpa-onnx");
        }

        Self::download_file(voice.download_url, &temp_tar, quiet).await?;

        // Extract tar.bz2
        if !quiet {
            println!("   {} Extracting...", "📂".cyan());
        }

        Self::extract_tar_bz2(&temp_tar, &models_dir).await?;

        // Clean up temp file
        let _ = tokio::fs::remove_file(&temp_tar).await;

        // Verify extraction
        if !model_path.exists() {
            return Err(BiboError::DownloadFailed(format!(
                "Model file not found after extraction: {}",
                voice.name
            )));
        }

        if !quiet {
            println!("{} {} installed successfully!", "✅".green(), voice.name);
        }

        Ok(true)
    }

    /// Extract tar.bz2 file
    async fn extract_tar_bz2(tar_path: &PathBuf, dest_dir: &PathBuf) -> Result<()> {
        use std::process::Command;

        // Use system tar command
        let output = Command::new("tar")
            .arg("-xjf")
            .arg(tar_path)
            .arg("-C")
            .arg(dest_dir)
            .output()
            .map_err(|e| BiboError::Other(format!("Failed to run tar: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BiboError::Other(format!(
                "tar extraction failed: {}",
                stderr
            )));
        }

        Ok(())
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
                    .template("   [{bar:30.cyan/blue}] {bytes}/{total_bytes} ({percent}%)")
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
