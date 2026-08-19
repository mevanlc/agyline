use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear},
};
use ratatui_textarea::TextArea;

pub fn render(f: &mut Frame, area: Rect, title: &str, textarea: &TextArea<'_>) {
    let popup = centered_rect(40, 3, area);
    f.render_widget(Clear, popup);

    let mut textarea = textarea.clone();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(format!(" {} ", title));

    textarea.set_block(block);
    textarea.set_cursor_line_style(Style::default());
    f.render_widget(&textarea, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
