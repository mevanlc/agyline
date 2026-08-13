use crate::config::theme::UserTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

pub struct ComponentListWidget;

impl ComponentListWidget {
    pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme, selected: usize, is_focused: bool) {
        let all_items: Vec<ListItem> = theme
            .components
            .iter()
            .enumerate()
            .map(|(i, comp)| {
                let check = if comp.enabled { "\u{2713}" } else { " " };
                let is_sel = i == selected;
                let cursor = if is_sel { ">" } else { " " };

                let style = if is_sel && is_focused {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_sel {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };

                ListItem::new(Line::from(Span::styled(
                    format!("{}[{}] {}", cursor, check, comp.display_name()),
                    style,
                )))
            })
            .collect();

        let total = all_items.len();
        let inner_height = area.height.saturating_sub(2) as usize; // borders

        let items = if total <= inner_height {
            all_items
        } else {
            // Both arrows always shown; visible slots = inner_height - 2
            let visible = inner_height.saturating_sub(2);
            let half = visible / 2;
            let raw_offset = selected.saturating_sub(half);
            let max_offset = total.saturating_sub(visible);
            let offset = raw_offset.min(max_offset);

            let has_above = offset > 0;
            let has_below = offset + visible < total;
            let arrow_active = Style::default().fg(Color::Gray);
            let arrow_inactive = Style::default().fg(Color::DarkGray);

            let mut visible_items: Vec<ListItem> = Vec::new();
            visible_items.push(ListItem::new(Line::from(Span::styled(
                " \u{2bac}",
                if has_above {
                    arrow_active
                } else {
                    arrow_inactive
                },
            ))));
            visible_items.extend(all_items.into_iter().skip(offset).take(visible));
            visible_items.push(ListItem::new(Line::from(Span::styled(
                " \u{2bae}",
                if has_below {
                    arrow_active
                } else {
                    arrow_inactive
                },
            ))));
            visible_items
        };

        let border_style = if is_focused {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(" Components ");

        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }
}
