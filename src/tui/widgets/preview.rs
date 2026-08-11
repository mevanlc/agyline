use crate::config::theme::UserTheme;
use crate::core::render;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct PreviewWidget;

impl PreviewWidget {
    pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme, compact: bool) {
        let texts = render::demo_texts_for_components(&theme.components);
        let line = render::build_render_line(&theme.components, theme.style.mode, &texts);
        let spans = render::render_spans(&line);
        let line = Line::from(spans);
        if compact {
            let paragraph = Paragraph::new(line);
            f.render_widget(paragraph, area);
        } else {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Preview ")
                .title_style(Style::default().fg(Color::DarkGray));
            let paragraph = Paragraph::new(line).block(block);
            f.render_widget(paragraph, area);
        }
    }
}
