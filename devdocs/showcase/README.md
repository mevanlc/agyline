# Showcase Generator

Generate curated `xline` marketing assets from fixed sample sessions.

The generator:

- builds `xline` if needed
- uses a sandboxed `HOME` so your real `~/.claude/xline` config is untouched
- writes default themes into that sandbox
- activates a curated set of themes and variants
- renders ANSI statuslines from sample JSON inputs
- emits per-card ANSI, plain text, HTML, and PNG artifacts
- optionally captures the rendered line through `tmux` for pane-state comparison

## Requirements

- `uv`
- `cargo`
- `git`
- macOS `qlmanage` if you want PNGs
- `tmux` if you want tmux capture artifacts

## Usage

From the repo root:

```bash
cd aidocs/showcase
uv sync
uv run python generate.py
```

Artifacts are written to `aidocs/showcase/out/`.

Useful flags:

```bash
uv run python generate.py --skip-build
uv run python generate.py --no-png
uv run python generate.py --no-tmux
uv run python generate.py --only powerline-dark-heavy
```

## Output

The generator writes:

- `cards/<slug>.ansi`
- `cards/<slug>.txt`
- `cards/<slug>.html`
- `cards/<slug>.png` when PNG export is available
- `cards/<slug>.tmux.ansi` and `cards/<slug>.tmux.txt` when tmux capture is enabled
- `gallery.html`
- `gallery.png` when PNG export is available
- `manifest.json`
