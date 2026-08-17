# agyline
`agyline` is a fast, customizable statusline generator and interactive TUI theme editor for the Google Antigravity CLI (and compatible AI coding assistants).

## Features
- **Keyboard-driven TUI editor** built with `ratatui`
- **Official Google Antigravity support**: Built-in support for `agent_state`, `model`, `vcs` (git), `context_window`, `quota` (weekly bucket), `task_count`, `execution_mode`, `vim`, `artifact_count`, `sandbox`, `plan_tier`, `email`, and more.
- **Smart model tier icons**: Gemini model tiers (`Flash`, `Pro`, `Ultra`, `Flash Lite`) with customizable icons, preserving Claude model tiers (`Opus`, `Sonnet`, `Haiku`).
- **Fast native VCS handling**: Utilizes precomputed `vcs` payload from Antigravity to avoid slow subprocess calls.
- **Rich preset collection**: 10 built-in color schemes (Default, Cometix, Gruvbox, Late, Minimal, Nord, Powerline Dark, Powerline Light, Rose Pine, Tokyo Night) and 5 icon sets (Emoji, Late, Minimal, Nerd Font, Powerline).
- **Multiple rendering styles**: Plain, Nerd Font, Emoji, and Powerline glyph modes.

## Build and Install
```bash
# Build from source:
cargo build --release

# Install locally:
cargo install --path .
```

## Usage
Run `agyline` in a terminal to launch the interactive theme editor:
```bash
agyline
```

### Automatic Setup with Google Antigravity CLI
Quickly configure Antigravity CLI to use `agyline`:
```bash
# Configure statusLine in ~/.gemini/antigravity-cli/settings.json
agyline --agy-setup

# Overwrite if a different statusLine command is already configured
agyline --agy-setup-force

# Remove agyline from settings.json
agyline --agy-unsetup
```

### Manual Configuration
You can also manually edit `~/.gemini/antigravity-cli/settings.json`:
```json
{
  "statusLine": {
    "type": "command",
    "command": "agyline"
  }
}
```

## Config Directory
By default, `agyline` stores its configuration in `~/.gemini/antigravity-cli/agyline` (with fallback to `XLINE_CONFIG_DIR`). Override the config directory with `--config-dir`:
```bash
agyline --config-dir /tmp/agyline-test
```

Or via environment variable:
```bash
AGYLINE_CONFIG_DIR=/tmp/agyline-test agyline
```

## Inspiration
This project drew inspiration from [CCometixLine](https://github.com/Haleclipse/CCometixLine).

