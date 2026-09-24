use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};

/// Style applied to the `[` `]` brackets around keys.
fn bracket_style() -> Style {
    Style::default().fg(Color::Indexed(236))
}

/// Style applied to the key text inside brackets.
fn key_style() -> Style {
    Style::default().fg(Color::Indexed(245))
}

/// Style applied to the label portion.
fn label_style() -> Style {
    Style::default().fg(Color::Gray)
}

/// Render a grid of (key, label) pairs into aligned Lines.
///
/// Each row is a slice of (key, label) pairs. Keys are right-aligned
/// within their column, 1 space gap to label, 2 spaces between columns.
///
/// ```text
///    [←→] Panel  [↑↓] Nav     [⏎] Edit/Toggle  [C] Colors
///    [A/D] Theme   [^S] Menu         [S+↑↓] Reorder  [I] Icons
/// ```
pub fn render_grid(rows: &[&[(&str, &str)]]) -> Vec<Line<'static>> {
    if rows.is_empty() {
        return Vec::new();
    }

    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // Max display width of "[key]" and "label" per column
    let mut key_widths = vec![0usize; num_cols];
    let mut label_widths = vec![0usize; num_cols];

    for row in rows {
        for (col, (key, label)) in row.iter().enumerate() {
            key_widths[col] = key_widths[col].max(Text::raw(*key).width() + 2); // +2 for []
            label_widths[col] = label_widths[col].max(Text::raw(*label).width());
        }
    }

    rows.iter()
        .map(|row| {
            let mut spans: Vec<Span<'static>> = Vec::new();
            for (col, (key, label)) in row.iter().enumerate() {
                if col > 0 {
                    spans.push(Span::raw("  ".to_string()));
                }

                // Right-align key: pad left
                let key_w = Text::raw(*key).width() + 2; // +2 for []
                let pad = key_widths[col].saturating_sub(key_w);
                if pad > 0 {
                    spans.push(Span::raw(" ".repeat(pad)));
                }
                spans.push(Span::styled("[", bracket_style()));
                spans.push(Span::styled(key.to_string(), key_style()));
                spans.push(Span::styled("]", bracket_style()));

                // 1 space gap
                spans.push(Span::raw(" ".to_string()));

                // Left-align label: pad right
                let label_w = Text::raw(*label).width();
                let label_pad = label_widths[col].saturating_sub(label_w);
                spans.push(Span::styled(label.to_string(), label_style()));
                if label_pad > 0 {
                    spans.push(Span::raw(" ".repeat(label_pad)));
                }
            }
            Line::from(spans)
        })
        .collect()
}

/// Render a single row of key hints (convenience wrapper around `render_grid`).
pub fn render(pairs: &[(&str, &str)]) -> Line<'static> {
    let rows = render_grid(&[pairs]);
    rows.into_iter().next().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_grid_alignment_with_unicode_arrows() {
        let rows = [
            &[("\u{2190}\u{2192}", "Panel"), ("C", "Colors")][..],
            &[("Space", "Edit/Toggle"), ("I", "Icons")][..],
        ];
        let lines = render_grid(&rows);
        assert_eq!(lines.len(), 2);

        // Both lines should have the same total width
        assert_eq!(lines[0].width(), lines[1].width());
    }
}
