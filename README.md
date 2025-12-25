# Bibo 🎙️

> Fast, local neural text-to-speech CLI. No cloud. No latency. Just speak.

<p align="center">
  <a href="https://larrykoo711.github.io/bibo">Website</a> •
  <a href="https://github.com/larrykoo711/bibo/releases">Releases</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a>
</p>

<p align="center">
  <a href="https://github.com/larrykoo711/bibo/actions/workflows/ci.yml"><img src="https://github.com/larrykoo711/bibo/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/larrykoo711/bibo/releases"><img src="https://img.shields.io/github/v/release/larrykoo711/bibo?color=green" alt="Release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux-lightgrey" alt="Platform">
</p>

---

Bibo is a blazing-fast, privacy-first TTS tool that runs entirely on your machine. Built with Rust for instant startup and powered by [Piper TTS](https://github.com/rhasspy/piper) neural voices.

## Features

- ⚡ **Lightning Fast** - Rust CLI with instant startup, synthesize in milliseconds
- 🔒 **100% Local** - All processing on your machine, text never leaves your device
- 🎭 **12 Neural Voices** - High-quality voices in English, Spanish, French, German, Italian, Russian
- 🛠️ **Unix Philosophy** - Pipe-friendly, scriptable, composable

## Installation

### Homebrew (macOS ARM64)

```bash
brew install larrykoo711/tap/bibo
```

### From Releases

Download the latest binary from [Releases](https://github.com/larrykoo711/bibo/releases):

```bash
# macOS ARM64
curl -L https://github.com/larrykoo711/bibo/releases/latest/download/bibo-darwin-arm64.tar.gz | tar xz
sudo mv bibo /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/larrykoo711/bibo.git
cd bibo
cargo build --release
sudo cp target/release/bibo /usr/local/bin/
```

### Python Dependency

Bibo requires Python 3.10+ with `piper-tts`:

```bash
pip3 install piper-tts
```

## Quick Start

```bash
# Speak text
bibo "Hello, world!"

# List available voices
bibo -l

# Download a voice
bibo -d amy

# Save to file
bibo "Welcome" -v amy -o welcome.wav

# Read from stdin
cat article.txt | bibo

# Fast mode (1.2x speed)
bibo -f "Quick announcement"
```

## Available Voices

| Voice    | Language   | Gender | Quality |
|----------|------------|--------|---------|
| amy      | English US | Female | Medium  |
| danny    | English US | Male   | Medium  |
| joe      | English US | Male   | Medium  |
| kathleen | English US | Female | Medium  |
| alan     | English GB | Male   | Medium  |
| alba     | English GB | Female | Medium  |
| eva      | Spanish ES | Female | Medium  |
| camille  | French FR  | Female | Medium  |
| thorsten | German DE  | Male   | Medium  |
| riccardo | Italian IT | Male   | Medium  |
| irina    | Russian RU | Female | Medium  |
| dmitri   | Russian RU | Male   | Medium  |

## Usage

```
bibo [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to synthesize (or use -i for file, or stdin)

Options:
  -v, --voice <VOICE>    Voice to use [default: amy] [env: BIBO_VOICE]
  -s, --speed <SPEED>    Speed preset: slow, normal, fast [default: normal]
  -f, --fast             Shortcut for --speed fast
  -i, --input <FILE>     Read text from file
  -o, --output <FILE>    Save audio to WAV file
  -q, --quiet            Suppress progress output
  -l, --list             List available voices
  -d, --download <VOICE> Download a voice model
  -h, --help             Print help
  -V, --version          Print version
```

## Environment Variables

```bash
export BIBO_VOICE="amy"    # Default voice
export BIBO_SPEED="normal" # Default speed
```

## Requirements

- macOS 12+ (ARM64) or Linux (coming soon)
- Python 3.10+
- ~50MB per voice model

## How It Works

Bibo uses a hybrid architecture:

1. **Rust CLI** - Handles argument parsing, file I/O, audio playback
2. **Python TTS** - Piper neural TTS for high-quality synthesis

This gives you the best of both worlds: instant CLI startup and proven neural TTS quality.

## Roadmap

- [x] macOS ARM64 support
- [ ] macOS x64 support
- [ ] Linux x64 support
- [ ] More voice models
- [ ] Streaming synthesis

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- [Piper TTS](https://github.com/rhasspy/piper) - Neural TTS engine
- Voice models from [Piper Voices](https://huggingface.co/rhasspy/piper-voices)

## Links

- 🌐 **Website**: [larrykoo711.github.io/bibo](https://larrykoo711.github.io/bibo)
- 📦 **Releases**: [github.com/larrykoo711/bibo/releases](https://github.com/larrykoo711/bibo/releases)
- 🍺 **Homebrew**: `brew install larrykoo711/tap/bibo`

---

<p align="center">
  <strong>Move fast. Ship it. Bibo.</strong> 🚀
</p>
