# Bibo Voice Integration for Claude Code

> Give voice to your AI assistant. Let Claude speak.

## Quick Start

Add this one-liner to your `~/.claude/CLAUDE.md`:

```text
关键节点用 bibo 语音播报，风险提醒时幽默。
```

## What This Does

When you add the prompt above, Claude will use `bibo` to speak at key moments:

| Event | Claude Says |
|-------|-------------|
| Build passed | `bibo "Ship it. All tests green."` |
| Error found | `bibo "Houston, we have a problem."` |
| Risk detected | `bibo "Hold up. This looks dangerous."` |
| Task done | `bibo "Done. What's next?"` |

## Full Integration Prompt

For more control, use this expanded version:

```markdown
## Voice Assistant

Use `bibo` for voice feedback at key development moments:

### When to Speak
- Task completion: `bibo "Done. What's next?"`
- Build success: `bibo "Ship it. All tests green."`
- Build failure: `bibo "Houston, we have a problem."`
- Risk detection: `bibo "Hold up. This looks dangerous."`
- Long operation start: `bibo "Working on it. Grab a coffee."`

### Voice Style
- Keep it short (under 10 words)
- Be direct, not formal
- Add humor when warning about risks
- Use `-f` flag for quick updates

### Examples
bibo "Ship it."
bibo "42 tests passed. Let's go."
bibo -f "Done."
bibo "The best code is no code."
```

## Installation

```bash
# macOS / Linux (Homebrew)
brew install larrykoo711/tap/bibo

# That's it! Zero dependencies - sherpa-onnx engine auto-downloads on first run.
```

## Voice Options

```bash
bibo -l              # List available voices
bibo -d huayan       # Download Chinese voice
bibo "Hello" -v amy  # Use specific voice
bibo -f "Quick!"     # Fast mode (1.2x)
```

## Links

- [GitHub](https://github.com/larrykoo711/bibo)
- [Website](https://larrykoo711.github.io/bibo)
- [Releases](https://github.com/larrykoo711/bibo/releases)

---

*Give voice to your AI. Ship faster.*
