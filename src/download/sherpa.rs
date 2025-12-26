//! Sherpa-onnx binary download module
//!
//! Auto-download sherpa-onnx TTS engine on first run

use crate::error::{BiboError, Result};
use crate::tts::sherpa::{sherpa_bin_dir, sherpa_download_url, sherpa_tts_path};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Sherpa-onnx downloader
pub struct SherpaDownloader;

impl SherpaDownloader {
    /// Download and install sherpa-onnx binary
    pub async fn download(quiet: bool) -> Result<()> {
        let sherpa_path = sherpa_tts_path();

        // Check if already installed
        if sherpa_path.exists() {
            if !quiet {
                println!("{} Sherpa-onnx already installed", "✅".green());
            }
            return Ok(());
        }

        let url = sherpa_download_url();
        let bin_dir = sherpa_bin_dir();

        if !quiet {
            println!("{} Downloading sherpa-onnx TTS engine...", "📦".cyan());
            println!("   From: {}", url);
        }

        // Create bin directory
        tokio::fs::create_dir_all(&bin_dir)
            .await
            .map_err(|e| BiboError::Other(format!("Failed to create bin dir: {}", e)))?;

        // Download to temp file
        let temp_tar = bin_dir.join("sherpa_temp.tar.bz2");
        Self::download_file(url, &temp_tar, quiet).await?;

        // Extract tar.bz2
        if !quiet {
            println!("   {} Extracting...", "📂".cyan());
        }

        Self::extract_tar_bz2(&temp_tar, &bin_dir).await?;

        // Clean up temp file
        let _ = tokio::fs::remove_file(&temp_tar).await;

        // Make binaries executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let bin_subdir = bin_dir.join("bin");
            if let Ok(entries) = std::fs::read_dir(&bin_subdir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if entry.path().is_file() {
                        if let Ok(mut perms) =
                            std::fs::metadata(entry.path()).map(|m| m.permissions())
                        {
                            perms.set_mode(0o755);
                            let _ = std::fs::set_permissions(entry.path(), perms);
                        }
                    }
                }
            }
        }

        if !quiet {
            println!("{} Sherpa-onnx installed successfully!", "✅".green());
        }

        Ok(())
    }

    /// Download a file with progress
    async fn download_file(url: &str, dest: &Path, quiet: bool) -> Result<()> {
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

    /// Extract tar.bz2 file
    /// Sherpa-onnx tar extracts to sherpa-onnx-v{version}-{platform}/
    async fn extract_tar_bz2(tar_path: &Path, dest_dir: &Path) -> Result<()> {
        use std::process::Command;

        // Use system tar command (available on macOS and Linux)
        let output = Command::new("tar")
            .arg("-xjf")
            .arg(tar_path)
            .arg("-C")
            .arg(dest_dir)
            .arg("--strip-components=1") // Remove top-level directory
            .output()
            .map_err(|e| BiboError::Other(format!("Failed to run tar: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BiboError::Other(format!(
                "tar extraction failed: {}",
                stderr
            )));
        }

        // Verify extraction
        let sherpa_binary = dest_dir.join("bin").join("sherpa-onnx-offline-tts");
        if !sherpa_binary.exists() {
            return Err(BiboError::Other(
                "sherpa-onnx-offline-tts binary not found in extracted archive".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if sherpa-onnx needs to be downloaded
    pub fn needs_download() -> bool {
        !sherpa_tts_path().exists()
    }
}
