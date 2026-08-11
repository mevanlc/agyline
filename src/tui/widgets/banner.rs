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

const MASCOT_COLOR: Color = Color::Rgb(215, 119, 87);
const BORDER_COLOR: Color = Color::Rgb(215, 119, 87);
const TEXT_COLOR: Color = Color::Rgb(153, 153, 153);

/// Try to read the Claude Code version from ~/.claude.json (lastReleaseNotesSeen).
fn read_cc_version() -> String {
    let path = match dirs::home_dir() {
        Some(h) => h.join(".claude.json"),
        None => return "?.?.?".into(),
    };
    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return "?.?.?".into(),
    };
    let json: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => return "?.?.?".into(),
    };
    match json.get("lastReleaseNotesSeen").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => "?.?.?".into(),
    }
}

/// Render the Claude Code banner with mascot, then the statusline preview
/// immediately below (no gap), mimicking the real Claude Code startup screen.
pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Banner box
            Constraint::Length(1),  // Statusline (immediately below)
        ])
        .split(area);

    let version = read_cc_version();

    // --- Banner box (outer border) ---
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .border_style(Style::default().fg(BORDER_COLOR))
        .title(format!(" Claude Code v{} ", version))
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
            "Opus \u{00b7} Sonnet \u{00b7} Haiku",
            Style::default().fg(TEXT_COLOR),
        )),
        Line::from(Span::styled(
            format!("~/xline-v{}", env!("CARGO_PKG_VERSION")),
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
