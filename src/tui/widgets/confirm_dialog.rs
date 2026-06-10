use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn render(f: &mut Frame, area: Rect, message: &str, hints: &[(&str, &str)]) {
    let msg_lines: Vec<&str> = message.lines().collect();
    let max_line = msg_lines.iter().map(|l| l.len()).max().unwrap_or(20);
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
