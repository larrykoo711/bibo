//! Sherpa-onnx TTS engine management
//!
//! Zero-dependency, native arm64 support via sherpa-onnx

use crate::error::{BiboError, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Sherpa-onnx version
pub const SHERPA_VERSION: &str = "1.12.20";

/// Sherpa-onnx download URL (Universal binary: arm64 + x86_64)
#[cfg(target_os = "macos")]
pub fn sherpa_download_url() -> &'static str {
    "https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.20/sherpa-onnx-v1.12.20-osx-universal2-shared.tar.bz2"
}

#[cfg(target_os = "linux")]
pub fn sherpa_download_url() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        "https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.20/sherpa-onnx-v1.12.20-linux-x64-shared.tar.bz2"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.20/sherpa-onnx-v1.12.20-linux-aarch64-shared.tar.bz2"
    }
}

/// Get sherpa bin directory
pub fn sherpa_bin_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("bibo")
        .join("sherpa")
}

/// Get sherpa TTS binary path (with playback)
pub fn sherpa_tts_play_path() -> PathBuf {
    sherpa_bin_dir()
        .join("bin")
        .join("sherpa-onnx-offline-tts-play")
}

/// Get sherpa TTS binary path (generate only)
pub fn sherpa_tts_path() -> PathBuf {
    sherpa_bin_dir().join("bin").join("sherpa-onnx-offline-tts")
}

/// Get sherpa lib directory (for DYLD_LIBRARY_PATH)
pub fn sherpa_lib_dir() -> PathBuf {
    sherpa_bin_dir().join("lib")
}

/// Find sherpa-onnx TTS binary with priority resolution
pub fn find_sherpa_tts() -> Result<PathBuf> {
    // 1. Environment variable
    if let Ok(path) = env::var("BIBO_SHERPA_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 2. Homebrew bundled (macOS)
    #[cfg(target_os = "macos")]
    {
        let homebrew_paths = [
            PathBuf::from("/opt/homebrew/opt/bibo/libexec/sherpa/bin/sherpa-onnx-offline-tts"),
            PathBuf::from("/usr/local/opt/bibo/libexec/sherpa/bin/sherpa-onnx-offline-tts"),
        ];
        for p in homebrew_paths {
            if p.exists() {
                return Ok(p);
            }
        }
    }

    // 3. User directory
    let user_sherpa = sherpa_tts_path();
    if user_sherpa.exists() {
        return Ok(user_sherpa);
    }

    // 4. System PATH
    if let Ok(output) = Command::new("which")
        .arg("sherpa-onnx-offline-tts")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    Err(BiboError::SherpaNotFound)
}

/// Find sherpa-onnx TTS-play binary (with audio playback)
pub fn find_sherpa_tts_play() -> Result<PathBuf> {
    // 1. Environment variable
    if let Ok(path) = env::var("BIBO_SHERPA_PLAY_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 2. Homebrew bundled (macOS)
    #[cfg(target_os = "macos")]
    {
        let homebrew_paths = [
            PathBuf::from("/opt/homebrew/opt/bibo/libexec/sherpa/bin/sherpa-onnx-offline-tts-play"),
            PathBuf::from("/usr/local/opt/bibo/libexec/sherpa/bin/sherpa-onnx-offline-tts-play"),
        ];
        for p in homebrew_paths {
            if p.exists() {
                return Ok(p);
            }
        }
    }

    // 3. User directory
    let user_sherpa = sherpa_tts_play_path();
    if user_sherpa.exists() {
        return Ok(user_sherpa);
    }

    // 4. System PATH
    if let Ok(output) = Command::new("which")
        .arg("sherpa-onnx-offline-tts-play")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    Err(BiboError::SherpaNotFound)
}

/// Check if sherpa-onnx is available
pub fn sherpa_available() -> bool {
    find_sherpa_tts().is_ok()
}

/// Get environment for running sherpa binaries
pub fn sherpa_env() -> Vec<(String, String)> {
    let lib_dir = sherpa_lib_dir();
    let mut env = vec![];

    #[cfg(target_os = "macos")]
    {
        // Set DYLD_LIBRARY_PATH for dynamic libraries
        let current = std::env::var("DYLD_LIBRARY_PATH").unwrap_or_default();
        let new_path = if current.is_empty() {
            lib_dir.to_string_lossy().to_string()
        } else {
            format!("{}:{}", lib_dir.to_string_lossy(), current)
        };
        env.push(("DYLD_LIBRARY_PATH".to_string(), new_path));
    }

    #[cfg(target_os = "linux")]
    {
        // Set LD_LIBRARY_PATH for dynamic libraries
        let current = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
        let new_path = if current.is_empty() {
            lib_dir.to_string_lossy().to_string()
        } else {
            format!("{}:{}", lib_dir.to_string_lossy(), current)
        };
        env.push(("LD_LIBRARY_PATH".to_string(), new_path));
    }

    env
}
