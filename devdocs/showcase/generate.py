#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import textwrap
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from ansi2html import Ansi2HTMLConverter
import tomlkit


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUTPUT_DIR = REPO_ROOT / "aidocs" / "showcase" / "out"
DEFAULT_BINARY = REPO_ROOT / "target" / "debug" / "xline"


@dataclass(frozen=True)
class SampleSpec:
    slug: str
    model_id: str
    model_name: str
    repo_name: str
    branch: str
    dirty: bool
    output_style: str
    cost_usd: float
    duration_ms: int
    api_duration_ms: int
    lines_added: int
    lines_removed: int
    input_tokens: int
    output_tokens: int
    cache_read_tokens: int


@dataclass(frozen=True)
class ShowcaseSpec:
    slug: str
    label: str
    theme: str
    sample: str
    variant: str
    surface: str


SAMPLES: dict[str, SampleSpec] = {
    "steady": SampleSpec(
        slug="steady",
        model_id="claude-sonnet-4-5",
        model_name="Sonnet 4.5",
        repo_name="xline-oss-demo",
        branch="main",
        dirty=False,
        output_style="concise",
        cost_usd=1.23,
        duration_ms=312_000,
        api_duration_ms=48_000,
        lines_added=24,
        lines_removed=6,
        input_tokens=12_000,
        output_tokens=4_000,
        cache_read_tokens=8_000,
    ),
    "heavy": SampleSpec(
        slug="heavy",
        model_id="claude-opus-4-1m",
        model_name="Opus 4.1",
        repo_name="reply-benchmark-pack",
        branch="feature/showcase-cards",
        dirty=True,
        output_style="detailed",
        cost_usd=12.84,
        duration_ms=4_230_000,
        api_duration_ms=820_000,
        lines_added=428,
        lines_removed=97,
        input_tokens=70_000,
        output_tokens=16_400,
        cache_read_tokens=100_000,
    ),
    "speed": SampleSpec(
        slug="speed",
        model_id="claude-haiku-4",
        model_name="Haiku 4",
        repo_name="reply-fast-path",
        branch="fix/reply-flow",
        dirty=False,
        output_style="terse",
        cost_usd=0.08,
        duration_ms=41_000,
        api_duration_ms=9_000,
        lines_added=7,
        lines_removed=2,
        input_tokens=2_600,
        output_tokens=600,
        cache_read_tokens=1_000,
    ),
}


SHOWCASES: list[ShowcaseSpec] = [
    ShowcaseSpec("default-steady", "Default", "Default", "steady", "expanded", "dark"),
    ShowcaseSpec("cometix-steady", "Cometix", "Cometix", "steady", "expanded", "dark"),
    ShowcaseSpec("gruvbox-heavy", "Gruvbox", "Gruvbox", "heavy", "expanded", "dark"),
    ShowcaseSpec("nord-steady", "Nord", "Nord", "steady", "expanded", "dark"),
    ShowcaseSpec("rose-pine-heavy", "Rose Pine", "Rose Pine", "heavy", "expanded", "dark"),
    ShowcaseSpec("tokyo-night-steady", "Tokyo Night", "Tokyo Night", "steady", "expanded", "dark"),
    ShowcaseSpec(
        "powerline-dark-heavy",
        "Powerline Dark",
        "Powerline Dark",
        "heavy",
        "expanded",
        "dark",
    ),
    ShowcaseSpec(
        "powerline-light-steady",
        "Powerline Light",
        "Powerline Light",
        "steady",
        "expanded",
        "light",
    ),
    ShowcaseSpec("minimal-ascii", "Minimal ASCII", "Minimal", "speed", "ascii", "light"),
]


SURFACES: dict[str, dict[str, str]] = {
    "dark": {
        "page": "#0b1020",
        "card": "#131b2e",
        "border": "#28324a",
        "muted": "#8b9ab8",
        "title": "#f8fafc",
    },
    "light": {
        "page": "#e8edf5",
        "card": "#f8fafc",
        "border": "#c7d2e5",
        "muted": "#4b5563",
        "title": "#0f172a",
    },
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate xline showcase assets")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR)
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--no-png", action="store_true")
    parser.add_argument("--no-tmux", action="store_true")
    parser.add_argument("--only", action="append", default=[])
    return parser.parse_args()


def run(cmd: list[str], *, cwd: Path | None = None, env: dict[str, str] | None = None, stdin: str | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        env=env,
        input=stdin,
        text=True,
        capture_output=True,
        check=True,
    )


def build_binary(binary_path: Path) -> None:
    run(["cargo", "build", "--quiet"], cwd=REPO_ROOT)
    if not binary_path.exists():
        raise RuntimeError(f"expected built binary at {binary_path}")


def sample_usage(sample: SampleSpec) -> dict[str, int]:
    return {
        "input_tokens": sample.input_tokens,
        "output_tokens": sample.output_tokens,
        "cache_read_input_tokens": sample.cache_read_tokens,
        "total_tokens": sample.input_tokens + sample.output_tokens + sample.cache_read_tokens,
    }


def create_repo(repo_root: Path, transcript_root: Path, sample: SampleSpec) -> tuple[Path, Path]:
    repo_dir = repo_root / sample.repo_name
    repo_dir.mkdir(parents=True, exist_ok=True)

    run(["git", "init", "-b", sample.branch], cwd=repo_dir)
    run(["git", "config", "user.name", "xline showcase"], cwd=repo_dir)
    run(["git", "config", "user.email", "showcase@example.com"], cwd=repo_dir)

    (repo_dir / "README.md").write_text(
        f"# {sample.repo_name}\n\nGenerated fixture for xline showcase assets.\n",
        encoding="utf-8",
    )
    src_dir = repo_dir / "src"
    src_dir.mkdir(exist_ok=True)
    (src_dir / "main.rs").write_text(
        textwrap.dedent(
            """\
            fn main() {
                println!("showcase fixture");
            }
            """
        ),
        encoding="utf-8",
    )
    run(["git", "add", "."], cwd=repo_dir)
    run(["git", "commit", "-m", "Initial fixture"], cwd=repo_dir)

    if sample.dirty:
        (src_dir / "main.rs").write_text(
            textwrap.dedent(
                """\
                fn main() {
                    println!("showcase fixture with pending edits");
                }
                """
            ),
            encoding="utf-8",
        )

    transcript_root.mkdir(parents=True, exist_ok=True)
    transcript_path = transcript_root / f"{sample.slug}.jsonl"
    transcript_entry = {
        "type": "assistant",
        "message": {
            "usage": sample_usage(sample),
        },
    }
    transcript_path.write_text(json.dumps(transcript_entry) + "\n", encoding="utf-8")

    return repo_dir, transcript_path


def input_payload(sample: SampleSpec, repo_dir: Path, transcript_path: Path) -> dict[str, Any]:
    return {
        "model": {
            "id": sample.model_id,
            "display_name": sample.model_name,
        },
        "workspace": {
            "current_dir": str(repo_dir),
        },
        "transcript_path": str(transcript_path),
        "cost": {
            "total_cost_usd": sample.cost_usd,
            "total_duration_ms": sample.duration_ms,
            "total_api_duration_ms": sample.api_duration_ms,
            "total_lines_added": sample.lines_added,
            "total_lines_removed": sample.lines_removed,
        },
        "output_style": {
            "name": sample.output_style,
        },
    }


def write_default_themes(binary_path: Path, home_dir: Path) -> Path:
    env = os.environ.copy()
    env["HOME"] = str(home_dir)
    run([str(binary_path), "--write-default-themes", "--force"], env=env)
    themes_dir = home_dir / ".claude" / "xline" / "themes"
    if not themes_dir.exists():
        raise RuntimeError(f"themes directory was not created at {themes_dir}")
    return themes_dir


def load_theme(path: Path) -> Any:
    return tomlkit.parse(path.read_text(encoding="utf-8"))


def save_theme(path: Path, document: Any) -> None:
    path.write_text(tomlkit.dumps(document), encoding="utf-8")


def component_map(document: Any) -> dict[str, Any]:
    return {str(component["id"]): component for component in document["components"]}


def activate_theme(themes_dir: Path, theme_name: str, variant: str) -> Path:
    active_path: Path | None = None
    for path in sorted(themes_dir.glob("*.toml")):
        document = load_theme(path)
        document["active"] = path.stem == theme_name
        if path.stem == theme_name:
            active_path = path
            apply_variant(document, variant)
        save_theme(path, document)

    if active_path is None:
        raise RuntimeError(f"theme not found: {theme_name}")
    return active_path


def apply_variant(document: Any, variant: str) -> None:
    components = component_map(document)

    if variant == "expanded":
        for name in ("cost", "session", "output_style"):
            components[name]["enabled"] = True
        options = components["git"].get("options")
        if options is None:
            options = tomlkit.table()
            components["git"]["options"] = options
        options["show_sha"] = True
        return

    if variant == "ascii":
        document["style"]["mode"] = "plain"
        for component in document["components"]:
            comp_id = str(component["id"])
            component["enabled"] = comp_id in {
                "model",
                "directory",
                "git",
                "context_window",
                "session",
                "cost",
                "separator",
            }
            component["icon"]["plain"] = ""
            component["icon"]["nerd_font"] = ""
        components["separator"]["icon"]["plain"] = " | "
        components["separator"]["icon"]["nerd_font"] = " | "
        components["session"]["enabled"] = True
        components["cost"]["enabled"] = True
        return

    raise RuntimeError(f"unknown variant: {variant}")


def render_statusline(binary_path: Path, home_dir: Path, payload: dict[str, Any]) -> str:
    env = os.environ.copy()
    env["HOME"] = str(home_dir)
    result = run([str(binary_path)], env=env, stdin=json.dumps(payload))
    return result.stdout


def strip_ansi(text: str) -> str:
    import re

    return re.sub(r"\x1b\[[0-9;]*m", "", text)


def html_card(showcase: ShowcaseSpec, rendered_html: str) -> str:
    surface = SURFACES[showcase.surface]
    return textwrap.dedent(
        f"""\
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <title>{showcase.label}</title>
          <style>
            :root {{
              color-scheme: {'dark' if showcase.surface == 'dark' else 'light'};
            }}
            * {{
              box-sizing: border-box;
            }}
            html, body {{
              margin: 0;
              background: {surface['page']};
              width: 1400px;
              min-height: 280px;
              overflow: hidden;
              font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            }}
            body {{
              padding: 28px;
            }}
            .card {{
              border: 1px solid {surface['border']};
              border-radius: 20px;
              background: linear-gradient(180deg, {surface['card']} 0%, {surface['card']}ee 100%);
              padding: 24px 26px 28px;
              box-shadow: 0 18px 60px rgba(15, 23, 42, 0.18);
            }}
            .eyebrow {{
              font-size: 13px;
              letter-spacing: 0.08em;
              text-transform: uppercase;
              color: {surface['muted']};
              margin-bottom: 10px;
            }}
            .title {{
              font-size: 30px;
              font-weight: 700;
              color: {surface['title']};
              margin: 0 0 18px;
            }}
            .line {{
              white-space: pre-wrap;
              font-size: 30px;
              line-height: 1.45;
              overflow-wrap: anywhere;
              word-break: break-word;
            }}
          </style>
        </head>
        <body>
          <div class="card">
            <div class="eyebrow">xline showcase</div>
            <h1 class="title">{showcase.label}</h1>
            <div class="line">{rendered_html}</div>
          </div>
        </body>
        </html>
        """
    )


def gallery_html(cards: list[tuple[ShowcaseSpec, str]]) -> str:
    sections: list[str] = []
    for showcase, rendered_html in cards:
        surface = SURFACES[showcase.surface]
        sections.append(
            textwrap.dedent(
                f"""\
                <article class="card {showcase.surface}">
                  <div class="eyebrow">{showcase.theme}</div>
                  <h2>{showcase.label}</h2>
                  <div class="line">{rendered_html}</div>
                </article>
                """
            )
        )

    return textwrap.dedent(
        f"""\
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <title>xline showcase gallery</title>
          <style>
            * {{
              box-sizing: border-box;
            }}
            html, body {{
              margin: 0;
              width: 1800px;
              background:
                radial-gradient(circle at top left, #1d4ed8 0%, transparent 22%),
                radial-gradient(circle at top right, #7c3aed 0%, transparent 18%),
                linear-gradient(180deg, #0b1020 0%, #111827 100%);
              font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
              color: #f8fafc;
            }}
            body {{
              padding: 32px;
            }}
            .header {{
              margin-bottom: 28px;
            }}
            .header h1 {{
              margin: 0 0 8px;
              font-size: 44px;
            }}
            .header p {{
              margin: 0;
              color: #a5b4fc;
              font-size: 18px;
            }}
            .grid {{
              display: grid;
              grid-template-columns: 1fr 1fr;
              gap: 22px;
            }}
            .card {{
              border-radius: 18px;
              padding: 20px 22px 24px;
              border: 1px solid rgba(148, 163, 184, 0.18);
              box-shadow: 0 16px 40px rgba(15, 23, 42, 0.2);
            }}
            .card.dark {{
              background: rgba(15, 23, 42, 0.72);
            }}
            .card.light {{
              background: rgba(248, 250, 252, 0.96);
              color: #0f172a;
            }}
            .eyebrow {{
              font-size: 12px;
              letter-spacing: 0.08em;
              text-transform: uppercase;
              color: #94a3b8;
              margin-bottom: 8px;
            }}
            .card.light .eyebrow {{
              color: #64748b;
            }}
            h2 {{
              margin: 0 0 14px;
              font-size: 24px;
            }}
            .line {{
              white-space: pre-wrap;
              font-size: 22px;
              line-height: 1.4;
              overflow-wrap: anywhere;
              word-break: break-word;
            }}
          </style>
        </head>
        <body>
          <section class="header">
            <h1>xline statusline showcase</h1>
            <p>Curated theme variants rendered from fixed sample Claude Code sessions.</p>
          </section>
          <section class="grid">
            {''.join(sections)}
          </section>
        </body>
        </html>
        """
    )


def write_png_from_html(html_path: Path, output_path: Path) -> bool:
    if shutil.which("qlmanage") is None:
        return False

    temp_dir = output_path.parent / ".qlmanage"
    temp_dir.mkdir(parents=True, exist_ok=True)
    run(["qlmanage", "-t", "-s", "1800", "-o", str(temp_dir), str(html_path)])
    generated = temp_dir / f"{html_path.name}.png"
    if not generated.exists():
        return False
    shutil.move(str(generated), str(output_path))
    return True


def capture_via_tmux(ansi_path: Path) -> tuple[str, str] | None:
    if shutil.which("tmux") is None:
        return None

    session_name = f"xline-showcase-{os.getpid()}"
    with tempfile.TemporaryDirectory(prefix="xline-showcase-tmux-") as tmp:
        runner = Path(tmp) / "runner.sh"
        runner.write_text(
            textwrap.dedent(
                f"""\
                #!/bin/sh
                python3 - <<'PY'
                from pathlib import Path
                import time
                print(Path({str(ansi_path)!r}).read_text(encoding="utf-8"), end="")
                time.sleep(1)
                PY
                """
            ),
            encoding="utf-8",
        )
        runner.chmod(0o755)

        try:
            run(["tmux", "new-session", "-d", "-x", "180", "-y", "6", "-s", session_name, str(runner)])
            ansi_capture = run(["tmux", "capture-pane", "-p", "-e", "-J", "-t", session_name]).stdout
            plain_capture = run(["tmux", "capture-pane", "-p", "-J", "-t", session_name]).stdout
        finally:
            subprocess.run(["tmux", "kill-session", "-t", session_name], capture_output=True, text=True)

    return ansi_capture, plain_capture


def render_html_fragment(ansi_text: str, *, dark_background: bool) -> str:
    converter = Ansi2HTMLConverter(inline=True, dark_bg=dark_background)
    return converter.convert(ansi_text, full=False)


def ensure_clean_output(output_dir: Path) -> None:
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "cards").mkdir(parents=True, exist_ok=True)


def selected_showcases(only: list[str]) -> list[ShowcaseSpec]:
    if not only:
        return SHOWCASES
    wanted = set(only)
    chosen = [showcase for showcase in SHOWCASES if showcase.slug in wanted]
    missing = wanted - {showcase.slug for showcase in chosen}
    if missing:
        raise RuntimeError(f"unknown showcase slugs: {', '.join(sorted(missing))}")
    return chosen


def generate(args: argparse.Namespace) -> None:
    binary_path = args.binary.resolve()
    if not args.skip_build:
        build_binary(binary_path)
    elif not binary_path.exists():
        raise RuntimeError(f"binary not found at {binary_path}")

    output_dir = args.output_dir.resolve()
    ensure_clean_output(output_dir)

    cards: list[tuple[ShowcaseSpec, str]] = []
    manifest: dict[str, Any] = {"cards": []}

    with tempfile.TemporaryDirectory(prefix="xline-showcase-home-") as temp_home_root, tempfile.TemporaryDirectory(
        prefix="xline-showcase-work-"
    ) as temp_work_root:
        home_dir = Path(temp_home_root)
        work_root = Path(temp_work_root)
        themes_dir = write_default_themes(binary_path, home_dir)
        transcript_root = work_root / "transcripts"

        sample_state: dict[str, tuple[Path, Path]] = {}
        for sample in SAMPLES.values():
            sample_state[sample.slug] = create_repo(work_root, transcript_root, sample)

        for showcase in selected_showcases(args.only):
            activate_theme(themes_dir, showcase.theme, showcase.variant)
            sample = SAMPLES[showcase.sample]
            repo_dir, transcript_path = sample_state[sample.slug]
            payload = input_payload(sample, repo_dir, transcript_path)
            ansi_text = render_statusline(binary_path, home_dir, payload)
            plain_text = strip_ansi(ansi_text)
            rendered_html = render_html_fragment(ansi_text, dark_background=showcase.surface == "dark")

            cards_dir = output_dir / "cards"
            ansi_path = cards_dir / f"{showcase.slug}.ansi"
            text_path = cards_dir / f"{showcase.slug}.txt"
            html_path = cards_dir / f"{showcase.slug}.html"
            png_path = cards_dir / f"{showcase.slug}.png"

            ansi_path.write_text(ansi_text, encoding="utf-8")
            text_path.write_text(plain_text + "\n", encoding="utf-8")
            html_path.write_text(html_card(showcase, rendered_html), encoding="utf-8")

            png_written = False
            if not args.no_png:
                png_written = write_png_from_html(html_path, png_path)

            tmux_written = False
            if not args.no_tmux:
                tmux_capture = capture_via_tmux(ansi_path)
                if tmux_capture is not None:
                    tmux_ansi, tmux_plain = tmux_capture
                    (cards_dir / f"{showcase.slug}.tmux.ansi").write_text(tmux_ansi, encoding="utf-8")
                    (cards_dir / f"{showcase.slug}.tmux.txt").write_text(tmux_plain, encoding="utf-8")
                    tmux_written = True

            cards.append((showcase, rendered_html))
            manifest["cards"].append(
                {
                    "slug": showcase.slug,
                    "label": showcase.label,
                    "theme": showcase.theme,
                    "sample": sample.slug,
                    "variant": showcase.variant,
                    "surface": showcase.surface,
                    "artifacts": {
                        "ansi": str(ansi_path.relative_to(output_dir)),
                        "text": str(text_path.relative_to(output_dir)),
                        "html": str(html_path.relative_to(output_dir)),
                        "png": str(png_path.relative_to(output_dir)) if png_written else None,
                        "tmux": tmux_written,
                    },
                }
            )

    gallery_path = output_dir / "gallery.html"
    gallery_path.write_text(gallery_html(cards), encoding="utf-8")
    gallery_png = output_dir / "gallery.png"
    gallery_png_written = False
    if not args.no_png:
        gallery_png_written = write_png_from_html(gallery_path, gallery_png)

    manifest["gallery"] = {
        "html": str(gallery_path.relative_to(output_dir)),
        "png": str(gallery_png.relative_to(output_dir)) if gallery_png_written else None,
    }
    (output_dir / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    try:
        generate(parse_args())
    except subprocess.CalledProcessError as exc:
        sys.stderr.write(exc.stderr)
        sys.stderr.write(exc.stdout)
        return exc.returncode
    except Exception as exc:
        sys.stderr.write(f"error: {exc}\n")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
