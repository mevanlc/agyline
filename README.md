# xline
`xline` is a small Rust Claude Code Statusline.

## Features
- keyboard-driven TUI editor built with `ratatui`
- starter themes written automatically on first run
- plain, Nerd Font, emoji, and powerline-style rendering
- configurable components for model, directory, hostname, git, native context-window usage, Claude rate limits, cost, session, and output style
- configurable comma-separated suffix stripping for displayed hostnames

## Build and Install
```bash
# Build from source:
cargo build --release
# install locally
cargo install --path .
```

## Usage
Run `xline` in a terminal to open the theme editor:
```bash
xline
```

Add to Claude Code by editing ~/.claude/settings.json and adding/editing the top-level property:
```json
"statusLine": {
  "type": "command",
  "command": "xline",
  "padding": 0
}
```

## Inspiration
This project drew inspiration and themes from [CCometixLine](https://github.com/Haleclipse/CCometixLine).
