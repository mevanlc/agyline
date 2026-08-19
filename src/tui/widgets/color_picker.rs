use crate::tui::app::{ColorPickerMode, ColorPickerState};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
};

pub fn render(f: &mut Frame, area: Rect, state: &ColorPickerState) {
    let popup = centered_rect(52, 14, area);
    f.render_widget(Clear, popup);

    let mut items: Vec<ListItem> = Vec::new();

    // Mode tabs
    let modes = [
        ("16-Color", ColorPickerMode::Color16),
        ("256-Color", ColorPickerMode::Color256),
        ("RGB", ColorPickerMode::Rgb),
    ];
    let mode_spans: Vec<Span> = modes
        .iter()
        .map(|(label, mode)| {
            if state.mode == *mode {
                Span::styled(
                    format!("[{}]", label),
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(format!(" {} ", label), Style::default().fg(Color::DarkGray))
            }
        })
        .collect();
    items.push(ListItem::new(Line::from(mode_spans)));
    items.push(ListItem::new(Line::from("")));

    match *state.mode.current() {
        ColorPickerMode::Color16 => {
            // Two columns: 0-7 left, 8-15 right
            for row in 0u8..8 {
                let left = row;
                let right = row + 8;
                let mut spans = Vec::new();

                // Left column
                let is_sel_l = left == state.c16_selection;
                let cursor_l = if is_sel_l { "> " } else { "  " };
                let color_l = c16_to_color(left);
                spans.push(Span::styled(
                    cursor_l,
                    if is_sel_l {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    },
                ));
                spans.push(Span::styled(
                    "\u{2588}\u{2588}",
                    Style::default().fg(color_l),
                ));
                spans.push(Span::styled(
                    format!(" {:3}", left),
                    if is_sel_l {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ));

                // Gap
                spans.push(Span::raw("  "));

                // Right column
                let is_sel_r = right == state.c16_selection;
                let cursor_r = if is_sel_r { "> " } else { "  " };
                let color_r = c16_to_color(right);
                spans.push(Span::styled(
                    cursor_r,
                    if is_sel_r {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    },
                ));
                spans.push(Span::styled(
                    "\u{2588}\u{2588}",
                    Style::default().fg(color_r),
                ));
                spans.push(Span::styled(
                    format!(" {:3}", right),
                    if is_sel_r {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ));

                items.push(ListItem::new(Line::from(spans)));
            }
        }
        ColorPickerMode::Color256 => {
            items.push(ListItem::new(Line::from(Span::styled(
                format!(
                    "Selected: {} [\u{2190}\u{2191}\u{2193}\u{2192} navigate]",
                    state.c256_selection
                ),
                Style::default().fg(Color::White),
            ))));
            let color = Color::Indexed(state.c256_selection);
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  Preview: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
                    Style::default().fg(color),
                ),
            ])));
        }
        ColorPickerMode::Rgb => {
            let labels = ["R", "G", "B"];
            for (i, label) in labels.iter().enumerate() {
                let is_sel = i == state.rgb_focus;
                let cursor = if is_sel { "> " } else { "  " };
                let val = match i {
                    0 => state.r_str(),
                    1 => state.g_str(),
                    _ => state.b_str(),
                };
                let style = if is_sel {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                items.push(ListItem::new(Line::from(Span::styled(
                    format!("{}{}: {}\u{2588}", cursor, label, val),
                    style,
                ))));
            }
            let r = state.r_val();
            let g = state.g_val();
            let b = state.b_val();
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  Preview: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
                    Style::default().fg(Color::Rgb(r, g, b)),
                ),
            ])));
        }
    }

    items.push(ListItem::new(Line::from("")));
    items.push(ListItem::new(super::key_hints::render(&[
        ("Tab", "Mode"),
        ("Enter", "Apply"),
        ("X", "Remove"),
        ("Esc", "Cancel"),
    ])));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Color Picker ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

fn c16_to_color(c: u8) -> Color {
    match c {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::Gray,
        8 => Color::DarkGray,
        9 => Color::LightRed,
        10 => Color::LightGreen,
        11 => Color::LightYellow,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,
        14 => Color::LightCyan,
        15 => Color::White,
        _ => Color::Reset,
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
