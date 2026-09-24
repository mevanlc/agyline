use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect, message: &str, hints: &[(&str, &str)]) {
    let msg_lines: Vec<&str> = message.lines().collect();
    let max_line = Text::raw(message).width().max(20);
    let width = (max_line as u16 + 6).clamp(30, 60);
    let height = msg_lines.len() as u16 + 4; // border(2) + blank + hints

    let popup = centered_rect(width, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow))
        .title(" Confirm ");

    let mut lines: Vec<Line> = msg_lines
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();
    lines.push(super::key_hints::render(hints));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_dialog_max_line_width() {
        let msg = "Delete theme '✨ Special Theme ✨'?\nThis action cannot be undone.";
        let max_line = Text::raw(msg).width().max(20);
        let width = (max_line as u16 + 6).clamp(30, 60);
        // Line 1 contains emojis (each width 2), total width is 35; line 2 is 29
        assert_eq!(max_line, 35);
        assert_eq!(width, 41);
    }
}
