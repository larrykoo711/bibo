# Bibo 🎙️

> Fast, local neural text-to-speech CLI. No cloud. No latency. Just speak.

[![CI](https://github.com/larrykoo711/bibo/actions/workflows/ci.yml/badge.svg)](https://github.com/larrykoo711/bibo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Bibo is a blazing-fast, privacy-first TTS tool that runs entirely on your machine. Built with Rust for instant startup and powered by [Piper TTS](https://github.com/rhasspy/piper) neural voices.

## Features

- ⚡ **Lightning Fast** - Rust CLI with instant startup, synthesize in milliseconds
- 🔒 **100% Local** - All processing on your machine, text never leaves your device
- 🎭 **12 Neural Voices** - High-quality voices in English, Spanish, French, German, Italian, Russian
- 🛠️ **Unix Philosophy** - Pipe-friendly, scriptable, composable

## Installation

### Homebrew (macOS/Linux)

```bash
brew install larrykoo711/tap/bibo
```

### From Source

```bash
git clone https://github.com/larrykoo711/bibo.git
cd bibo
cargo build --release
```

Requires Python 3.10+ with `piper-tts`:

```bash
pip install piper-tts
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

| Voice    | Language   | Gender |
|----------|------------|--------|
| amy      | English US | Female |
| danny    | English US | Male   |
| joe      | English US | Male   |
| kathleen | English US | Female |
| alan     | English GB | Male   |
| alba     | English GB | Female |
| eva      | Spanish ES | Female |
| camille  | French FR  | Female |
| thorsten | German DE  | Male   |
| riccardo | Italian IT | Male   |
| irina    | Russian RU | Female |
| dmitri   | Russian RU | Male   |

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

## Requirements

- macOS 12+ or Linux (x86_64, arm64)
- Python 3.10+
- ~50MB per voice model

## How It Works

Bibo uses a hybrid architecture:

1. **Rust CLI** - Handles argument parsing, file I/O, audio playback
2. **Python TTS** - Piper neural TTS for high-quality synthesis

This gives you the best of both worlds: instant CLI startup and proven neural TTS quality.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- [Piper TTS](https://github.com/rhasspy/piper) - Neural TTS engine
- Voice models from [Piper Voices](https://huggingface.co/rhasspy/piper-voices)

---

**Move fast. Ship it. Bibo.** 🚀
