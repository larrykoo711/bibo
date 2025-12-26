# Bibo

> Fast, local neural text-to-speech CLI. Zero dependencies. Just speak.

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

Bibo is a blazing-fast, privacy-first TTS tool that runs entirely on your machine. Built with Rust and powered by [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) neural TTS engine.

## Features

- **Zero Dependencies** - No Python, no setup. TTS engine auto-downloads on first run
- **Lightning Fast** - Rust CLI with instant startup, 15x faster than real-time synthesis
- **100% Local** - All processing on your machine, text never leaves your device
- **17 Neural Voices** - Chinese, English, Korean, Russian, Spanish, German, and more
- **Multi-language** - MeloTTS supports Chinese + English bilingual in single voice

## Installation

### Homebrew (macOS / Linux)

```bash
brew install larrykoo711/tap/bibo
```

### From Releases

Download the latest binary from [Releases](https://github.com/larrykoo711/bibo/releases):

```bash
# macOS ARM64
curl -L https://github.com/larrykoo711/bibo/releases/latest/download/bibo-darwin-arm64.tar.gz | tar xz
sudo mv bibo /usr/local/bin/

# macOS x64
curl -L https://github.com/larrykoo711/bibo/releases/latest/download/bibo-darwin-x64.tar.gz | tar xz
sudo mv bibo /usr/local/bin/

# Linux x64
curl -L https://github.com/larrykoo711/bibo/releases/latest/download/bibo-linux-x64.tar.gz | tar xz
sudo mv bibo /usr/local/bin/
```

### From Source

```bash
git clone https://github.com/larrykoo711/bibo.git
cd bibo
cargo build --release
sudo cp target/release/bibo /usr/local/bin/
```

## Quick Start

```bash
# Speak text (auto-downloads default voice on first run)
bibo "Hello, world!"

# Chinese + English bilingual with MeloTTS
bibo "Hello world. 你好世界。" -v melo

# Korean
bibo "안녕하세요" -v kss

# List installed voices
bibo -l

# Show downloadable voices
bibo -d list

# Download a voice
bibo -d amy

# Save to file
bibo "Welcome" -v amy -o welcome.wav

# Fast mode (1.2x speed)
bibo -f "Quick announcement"
```

## Available Voices

| Voice    | Language        | Gender | Notes           |
|----------|-----------------|--------|-----------------|
| melo     | Chinese+English | Female | Bilingual (default) |
| kss      | Korean          | Female | High quality    |
| amy      | English US      | Female |                 |
| ryan     | English US      | Male   |                 |
| huayan   | Chinese         | Female |                 |
| irina    | Russian         | Female |                 |
| eva      | Spanish         | Female |                 |
| thorsten | German          | Male   |                 |
| ...      | +9 more         |        |                 |

Run `bibo -d list` to see all 17 available voices.

## Usage

```
bibo [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to synthesize (or use -i for file, or stdin)

Options:
  -v, --voice <VOICE>    Voice to use [default: melo] [env: BIBO_VOICE]
  -s, --speed <SPEED>    Speed preset: slow, normal, fast [default: normal]
  -f, --fast             Shortcut for --speed fast
  -i, --input <FILE>     Read text from file
  -o, --output <FILE>    Save audio to WAV file
  -q, --quiet            Suppress progress output
  -l, --list             List installed voices
  -d, --download <VOICE> Download a voice model (or "list" to show all)
  -h, --help             Print help
  -V, --version          Print version
```

## Claude Code Integration

Add to `~/.claude/CLAUDE.md` to let Claude speak at key moments:

```
Use bibo CLI for voice feedback: task done, build status, risk warnings. Keep it short.
```

Then Claude will say things like:
- `bibo "Ship it. Tests green."`
- `bibo "Hold up. This looks risky."`

## Requirements

- macOS 12+ (ARM64 / x64) or Linux (x64 / ARM64)
- ~50MB per voice model
- sherpa-onnx TTS engine (auto-downloaded on first run)

## How It Works

Bibo uses [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) as the TTS engine:

1. **Rust CLI** - Handles arguments, file I/O, audio playback
2. **sherpa-onnx** - Native binary for neural TTS (VITS/MeloTTS models)

Zero Python. Zero dependencies. Everything auto-downloads on first run.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

- [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) - Next-gen Kaldi TTS engine
- [MeloTTS](https://github.com/myshell-ai/MeloTTS) - Bilingual TTS models

## Links

- **Website**: [larrykoo711.github.io/bibo](https://larrykoo711.github.io/bibo)
- **Releases**: [github.com/larrykoo711/bibo/releases](https://github.com/larrykoo711/bibo/releases)
- **Homebrew**: `brew install larrykoo711/tap/bibo`

---

<p align="center">
  <strong>Give voice to your AI. Ship faster.</strong>
</p>
