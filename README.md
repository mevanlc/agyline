# agyline
`agyline` is a fast, customizable statusline generator and interactive TUI theme editor for the Google Antigravity CLI (and compatible AI coding assistants).

## Features
- **Keyboard-driven TUI editor** built with `ratatui`
- **Official Google Antigravity support**: Built-in support for `agent_state`, `model`, `vcs` (git), `context_window`, `quota` (weekly bucket), `task_count`, `execution_mode`, `vim`, `artifact_count`, `sandbox`, `plan_tier`, `email`, and more.
- **Smart model tier icons**: Gemini model tiers (`Flash`, `Pro`, `Ultra`, `Flash Lite`) with customizable icons, preserving Claude model tiers (`Opus`, `Sonnet`, `Haiku`, `Fable`, `Mythos`).
- **Fast native VCS handling**: Utilizes precomputed `vcs` payload from Antigravity to avoid slow subprocess calls.
- **Rich preset collection**: 10 built-in color schemes (Default, Cometix, Gruvbox, Late, Minimal, Nord, Powerline Dark, Powerline Light, Rose Pine, Tokyo Night) and 5 icon sets (Emoji, Late, Minimal, Nerd Font, Powerline).
- **Named reusable configs**: Themes associate component, icon, and color configs; customs and built-ins can be mixed.
- **Independent styling**: Plain/emoji or Nerd Font glyphs, optional Powerline connectors, colors, and bold text.
- **Responsive editor**: Four sections, persistent drafts, and usable workflows at 60×18.

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
agyline --setup

# Overwrite if a different statusLine command is already configured
agyline --setup-force

# Remove agyline from settings.json
agyline --unsetup
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
By default, `agyline` stores its named resources and active theme in `~/.config/agyline/config.toml`. Directory precedence is CLI, `AGYLINE_CONFIG_DIR`, `XLINE_CONFIG_DIR`, then the default. Override the config directory with `--config-dir`:
```bash
agyline --config-dir /tmp/agyline-test
```

Or via environment variable:
```bash
AGYLINE_CONFIG_DIR=/tmp/agyline-test agyline
```

## Editing named themes

Use `1`–`4` for Themes, Components, Icons, and Colors. In Themes, Enter opens an association picker. In the config libraries, Tab switches list/inspector; Enter edits. `m` opens all actions and `?` shows help.

Built-ins are read-only. Editing a built-in or shared config defaults to a named copy for the current theme. Explicit shared editing lists the affected themes. Duplicate reuses a theme's three references; Full fork copies all three configs.

Navigation retains drafts. `s` saves the current context and required related drafts; `S` saves all. `a` activates the current theme. Saved changes to active references affect the next statusline immediately. Normal quit offers save/discard/cancel; Ctrl+C discards unsaved drafts and exits immediately.

Rendering is read-only. A missing catalog uses built-in Default without creating files. Existing `themes/*.toml` files are ignored and untouched; this format has no automatic migration. `--install-themes` has been removed.

```sh
agyline --validate-config
```

See [configuration, sharing, recovery, and shortcuts](devdocs/CONFIGURATION.md) and the [example catalog](examples/config.toml).

## Payload Logging
To inspect raw JSON state payloads piped to `agyline`, set the `AGYLINE_LOG_FILE` environment variable to a file path:
```bash
AGYLINE_LOG_FILE=~/.gemini/antigravity-cli/statusline-payloads.log agyline
```
Incoming payloads will be formatted and appended with UTC ISO-8601 timestamps:
```
/* 2026-08-17T04:16:43.123Z */
{
  "agent_state": "idle",
  "cwd": "/Users/user/code/my-project",
  "model": {
    "display_name": "Gemini 3.5 Flash",
    "id": "gemini-3.5-flash"
  }
}
```

## Inspiration
This project drew inspiration from [CCometixLine](https://github.com/Haleclipse/CCometixLine).


