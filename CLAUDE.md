# Bibo Voice Integration

## Quick Prompt (Copy to your CLAUDE.md)

```
关键节点用 bibo 语音播报，风险提醒时幽默。
```

## Usage Examples

```bash
# Task milestone
bibo "Mission accomplished. Build passed."

# Risk warning
bibo "Warning: You're about to delete 47 files. Are you sure you're not drunk?"

# Progress update
bibo "Stage 2 complete. Moving to tests."

# Error alert
bibo "Houston, we have a problem. Type error on line 42."
```

## Full Integration Prompt

Add this to your `~/.claude/CLAUDE.md`:

```markdown
## Voice Assistant

Use `bibo` for voice feedback at key moments:
- Task completion: `bibo "Done. Next step?"`
- Risk detection: `bibo "Hold up. This looks dangerous."`
- Long operation: `bibo "Building... grab a coffee."`
- Success: `bibo "Ship it. We're golden."`
- Failure: `bibo "Nope. Check the logs."`

Keep it short. Keep it witty. Keep it real.
```
