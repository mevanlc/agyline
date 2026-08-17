use crate::config::theme::UserTheme;
use crate::core::render;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

const MASCOT_COLOR: Color = Color::Rgb(66, 133, 244);
const BORDER_COLOR: Color = Color::Rgb(66, 133, 244);
const TEXT_COLOR: Color = Color::Rgb(153, 153, 153);

/// Try to read the Antigravity CLI version or Claude Code version fallback.
fn read_cli_version() -> String {
    if let Some(h) = dirs::home_dir() {
        let agy_path = h
            .join(".gemini")
            .join("antigravity-cli")
            .join("version.json");
        if let Ok(contents) = std::fs::read_to_string(&agy_path)
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents)
            && let Some(s) = json.get("version").and_then(|v| v.as_str())
            && !s.is_empty()
        {
            return s.to_string();
        }

        let claude_path = h.join(".claude.json");
        if let Ok(contents) = std::fs::read_to_string(&claude_path)
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents)
            && let Some(s) = json.get("lastReleaseNotesSeen").and_then(|v| v.as_str())
            && !s.is_empty()
        {
            return s.to_string();
        }
    }
    "1.0.0".into()
}

/// Render the Antigravity CLI banner with mascot, then the statusline preview
/// immediately below (no gap), mimicking the real Antigravity startup screen.
pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Banner box
            Constraint::Length(1),  // Statusline (immediately below)
        ])
        .split(area);

    let version = read_cli_version();

    // --- Banner box (outer border) ---
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .border_style(Style::default().fg(BORDER_COLOR))
        .title(format!(" Google Antigravity CLI v{} ", version))
        .title_style(Style::default().fg(BORDER_COLOR));

    let inner = outer_block.inner(layout[0]);
    f.render_widget(outer_block, layout[0]);

    // Split inner area into left column (fixed) and right column
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(23), // Left: mascot + info (aligns divider with panel border below)
            Constraint::Min(1),     // Right: future content
        ])
        .split(inner);

    // --- Left column: welcome + mascot + info ---
    let left_content = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome back!",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "\u{2590}\u{259b}\u{2588}\u{2588}\u{2588}\u{259c}\u{258c}",
            Style::default().fg(MASCOT_COLOR),
        )),
        Line::from(Span::styled(
            "\u{259d}\u{259c}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{259b}\u{2598}",
            Style::default().fg(MASCOT_COLOR),
        )),
        Line::from(Span::styled(
            " \u{2598}\u{2598} \u{259d}\u{259d}",
            Style::default().fg(MASCOT_COLOR),
        )),
        Line::from(Span::styled(
            "Flash \u{00b7} Pro \u{00b7} Ultra",
            Style::default().fg(TEXT_COLOR),
        )),
        Line::from(Span::styled(
            format!("~/agyline-v{}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(TEXT_COLOR),
        )),
    ]);

    let left = Paragraph::new(left_content).alignment(Alignment::Center);
    f.render_widget(left, columns[0]);

    // --- Divider: │ column between left and right ---
    let divider_lines: Vec<Line> = (0..columns[1].height)
        .map(|_| Line::from(Span::styled("\u{2502}", Style::default().fg(BORDER_COLOR))))
        .collect();
    let divider = Paragraph::new(Text::from(divider_lines));
    f.render_widget(
        divider,
        Rect {
            x: columns[1].x,
            y: columns[1].y,
            width: 1,
            height: columns[1].height,
        },
    );

    // --- Statusline preview (immediately below, no border) ---
    let texts = render::demo_texts_for_components(&theme.components);
    let line = render::build_render_line(&theme.components, theme.style.mode, &texts);
    let mut spans = vec![Span::raw("  ")]; // indent to match real CC
    spans.extend(render::render_spans(&line));

    let statusline = Paragraph::new(Line::from(spans));
    f.render_widget(statusline, layout[1]);
}

/// Height needed for the banner + statusline.
pub const HEIGHT: u16 = 11;
