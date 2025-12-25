//! Cross-platform audio playback

use crate::error::{BiboError, Result};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Audio player for WAV files
pub struct AudioPlayer;

impl AudioPlayer {
    /// Play a WAV file
    pub fn play_file(path: &Path) -> Result<()> {
        // Get output stream
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| BiboError::PlaybackFailed(format!("Failed to get audio output: {}", e)))?;

        // Create sink for playback
        let sink = Sink::try_new(&stream_handle).map_err(|e| {
            BiboError::PlaybackFailed(format!("Failed to create audio sink: {}", e))
        })?;

        // Open and decode file
        let file = File::open(path)
            .map_err(|e| BiboError::PlaybackFailed(format!("Failed to open audio file: {}", e)))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| BiboError::PlaybackFailed(format!("Failed to decode audio: {}", e)))?;

        // Play
        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }

    /// Play raw audio samples
    pub fn play_samples(samples: Vec<i16>, sample_rate: u32) -> Result<()> {
        use rodio::buffer::SamplesBuffer;

        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| BiboError::PlaybackFailed(format!("Failed to get audio output: {}", e)))?;

        let sink = Sink::try_new(&stream_handle).map_err(|e| {
            BiboError::PlaybackFailed(format!("Failed to create audio sink: {}", e))
        })?;

        // Convert i16 to f32 for rodio
        let samples_f32: Vec<f32> = samples.iter().map(|&s| s as f32 / 32768.0).collect();
        let source = SamplesBuffer::new(1, sample_rate, samples_f32);

        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }
}
