//! CLI argument parsing with clap
//!
//! YC Standard: Single-letter shortcuts, intuitive defaults

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Speed {
    Slow,
    Normal,
    Fast,
}

impl Speed {
    /// Convert to length scale for Piper
    /// Lower = faster speech
    pub fn to_length_scale(&self) -> f32 {
        match self {
            Speed::Slow => 1.2,
            Speed::Normal => 1.0,
            Speed::Fast => 0.8,
        }
    }
}

impl Default for Speed {
    fn default() -> Self {
        Speed::Normal
    }
}

/// Bibo - Fast, local neural text-to-speech
///
/// Built with Silicon Valley standards: simple, fast, powerful
#[derive(Parser, Debug)]
#[command(name = "bibo")]
#[command(version, about, long_about = None)]
#[command(after_help = r#"EXAMPLES:
    bibo "Hello world"              Just works
    bibo "Hello" -s fast            Fast speech
    bibo -i doc.md                  Read from file
    bibo -d list                    Show available voices
    bibo -d amy                     Download voice
    bibo -l                         List installed voices

ENVIRONMENT VARIABLES:
    BIBO_VOICE    Default voice (default: melo)
    BIBO_SPEED    Default speed (default: normal)
"#)]
pub struct Cli {
    /// Text to speak
    #[arg(value_name = "TEXT")]
    pub text: Option<String>,

    /// Voice model to use
    #[arg(short, long, env = "BIBO_VOICE", default_value = "melo")]
    pub voice: String,

    /// Speech speed
    #[arg(short, long, env = "BIBO_SPEED", value_enum, default_value = "normal")]
    pub speed: Speed,

    /// Fast mode (shortcut for -s fast)
    #[arg(short = 'f', long)]
    pub fast: bool,

    /// Input file (.md or .txt)
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<String>,

    /// Output WAV file (plays if not specified)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Quiet mode (no output)
    #[arg(short, long)]
    pub quiet: bool,

    /// List installed voices
    #[arg(short, long)]
    pub list: bool,

    /// Download voice: id, "list", "all", or "1,3,5"
    #[arg(short, long, value_name = "SPEC")]
    pub download: Option<String>,
}

impl Cli {
    /// Get effective speed (considering -f flag)
    pub fn effective_speed(&self) -> Speed {
        if self.fast {
            Speed::Fast
        } else {
            self.speed.clone()
        }
    }
}
