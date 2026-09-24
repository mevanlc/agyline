use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::path::PathBuf;

pub fn render(
    f: &mut Frame,
    area: Rect,
    themes: &[(String, PathBuf)],
    current_index: usize,
    active_name: Option<&str>,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Themes ");

    // Available width inside the border (minus 2 for border chars)
    let inner_width = area.width.saturating_sub(2) as usize;

    // Build entries with their display widths
    let sep = "  \u{2502}  "; // " │ "
    let sep_w = Text::raw(sep).width();
    let arrow_left = " < ";
    let arrow_right = " > ";
    let arrow_w = Text::raw(arrow_left).width();

    let entries: Vec<(String, bool)> = themes
        .iter()
        .map(|(name, _)| {
            let is_active = active_name == Some(name.as_str());
            let label = if is_active {
                format!("{}*", name)
            } else {
                name.clone()
            };
            (label, is_active)
        })
        .collect();

    // Find a window of entries centered on current_index that fits
    let mut start = current_index;
    let mut end = current_index + 1;

    // Width of just the selected entry + arrows
    let entry_width = |i: usize| Text::raw(&entries[i].0).width();

    let mut total_w = arrow_w * 2 + entry_width(current_index);

    // Expand window outward, alternating left and right
    loop {
        let mut grew = false;

        // Try expanding left
        if start > 0 {
            let candidate_w = sep_w + entry_width(start - 1);
            if total_w + candidate_w <= inner_width {
                start -= 1;
                total_w += candidate_w;
                grew = true;
            }
        }

        // Try expanding right
        if end < entries.len() {
            let candidate_w = sep_w + entry_width(end);
            if total_w + candidate_w <= inner_width {
                total_w += candidate_w;
                end += 1;
                grew = true;
            }
        }

        if !grew {
            break;
        }
    }

    // Build spans
    let mut spans: Vec<Span> = Vec::new();

    let left_arrow_style = if start > 0 {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    spans.push(Span::styled(arrow_left, left_arrow_style));

    for (i, (label, is_active)) in entries.iter().enumerate().take(end).skip(start) {
        if i > start {
            spans.push(Span::styled(sep, Style::default().fg(Color::Indexed(236))));
        }

        let style = if i == current_index {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if *is_active {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Indexed(245))
        };

        spans.push(Span::styled(label.clone(), style));
    }

    let right_arrow_style = if end < entries.len() {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    spans.push(Span::styled(arrow_right, right_arrow_style));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}
