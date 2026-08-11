# xline
`xline` is a small Rust Claude Code Statusline.

## Features
- keyboard-driven TUI editor built with `ratatui`
- starter themes written automatically on first run
- plain, Nerd Font, emoji, and powerline-style rendering
- configurable components for model, directory, hostname, git, pull requests, native context-window usage, separate five-hour and seven-day Claude rate limits, cost, session, and output style
- pull-request number display with optional review state, URL, and separate OSC 8 hyperlinks for every visible PR field
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

## Config Directory
By default, `xline` stores its configuration in `~/.claude/xline`, with themes
in the `themes` subdirectory. Override the config directory for one-off runs or
isolated testing with `--config-dir`:
```bash
xline --config-dir /tmp/xline-test
```

For shells, wrappers, and CI, set `XLINE_CONFIG_DIR`:
```bash
XLINE_CONFIG_DIR=/tmp/xline-test xline
```

The command-line option takes precedence over `XLINE_CONFIG_DIR`, which takes
precedence over the default. The selected directory applies to the TUI,
statusline rendering, initial theme bootstrap, and `--install-themes`.

## Inspiration
This project drew inspiration and themes from [CCometixLine](https://github.com/Haleclipse/CCometixLine).
