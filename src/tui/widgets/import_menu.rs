use crate::config::manager;
use crate::config::theme::UserTheme;
use crate::config::types::StyleMode;
use crate::core::render;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
};

pub fn render_colors(f: &mut Frame, area: Rect, selection: usize, theme: &UserTheme) {
    let user_themes = manager::list_themes().unwrap_or_default();
    let user_theme_data = load_user_theme_data(&user_themes);
    let schemes = filter_color_schemes(&user_theme_data);
    let texts = render::demo_texts_compact();

    // Pad names to max width
    let max_name_width = schemes
        .iter()
        .map(|s| Text::raw(s.name).width())
        .chain(user_themes.iter().map(|(n, _)| Text::raw(n).width()))
        .max()
        .unwrap_or(0);

    // Build render lines for presets
    let mut render_lines: Vec<render::RenderLine> = schemes
        .iter()
        .map(|scheme| {
            let mut preview_theme = theme.clone();
            scheme.apply_to(&mut preview_theme.components);
            let mode = if scheme.is_powerline() {
                if let Some(pl_icons) = crate::presets::icon_sets::find("Powerline") {
                    pl_icons.apply_to(&mut preview_theme.components);
                }
                StyleMode::Powerline
            } else {
                preview_theme.style.mode
            };
            render::build_render_line(&preview_theme.components, mode, &texts)
        })
        .collect();

    // Build render lines for user themes (apply their colors to current theme's icons)
    let mut user_render_lines: Vec<render::RenderLine> = user_theme_data
        .iter()
        .map(|ut| {
            let mut preview_theme = theme.clone();
            for src in &ut.components {
                if let Some(dest) = preview_theme.components.iter_mut().find(|c| c.id == src.id) {
                    dest.colors = src.colors.clone();
                    dest.styles = src.styles.clone();
                }
            }
            render::build_render_line(&preview_theme.components, preview_theme.style.mode, &texts)
        })
        .collect();

    // Align all lines together
    let mut all_lines: Vec<&mut render::RenderLine> = render_lines
        .iter_mut()
        .chain(user_render_lines.iter_mut())
        .collect();
    render::align_lines_refs(&mut all_lines);

    let mut items: Vec<ListItem> = Vec::new();

    for (i, (scheme, line)) in schemes.iter().zip(render_lines.iter()).enumerate() {
        let preview_spans = render::render_spans(line);

        let cursor = if i == selection { "> " } else { "  " };
        let cursor_style = item_style(i, selection);
        let pad = max_name_width.saturating_sub(Text::raw(scheme.name).width());
        let padded_name = format!("{}{} ", scheme.name, " ".repeat(pad));

        let mut spans = vec![
            Span::styled(cursor.to_string(), cursor_style),
            Span::styled(padded_name, Style::default().fg(Color::DarkGray)),
        ];
        spans.extend(preview_spans);

        items.push(ListItem::new(Line::from(spans)));
    }

    if !user_themes.is_empty() && !schemes.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "\u{2500}\u{2500}\u{2500} User Themes \u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    for (i, ((name, _), line)) in user_themes.iter().zip(user_render_lines.iter()).enumerate() {
        let preview_spans = render::render_spans(line);
        let idx = schemes.len() + i;
        let cursor = if idx == selection { "> " } else { "  " };
        let cursor_style = item_style(idx, selection);
        let pad = max_name_width.saturating_sub(Text::raw(name).width());
        let padded_name = format!("{}{} ", name, " ".repeat(pad));

        let mut spans = vec![
            Span::styled(cursor.to_string(), cursor_style),
            Span::styled(padded_name, Style::default().fg(Color::DarkGray)),
        ];
        spans.extend(preview_spans);

        items.push(ListItem::new(Line::from(spans)));
    }

    // Footer note
    let note_style = Style::default().fg(Color::DarkGray);
    items.push(ListItem::new(Line::from(Span::styled(
        "  * Only colors will be imported.",
        note_style,
    ))));

    let height = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
    let popup = centered_rect(70, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Import Color Scheme ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

pub fn render_icons(f: &mut Frame, area: Rect, selection: usize, theme: &UserTheme) {
    let user_themes = manager::list_themes().unwrap_or_default();
    let user_theme_data = load_user_theme_data(&user_themes);
    let icon_sets = filter_icon_sets(&user_theme_data);
    let texts = render::demo_texts_compact();

    // Pad names to max width
    let max_name_width = icon_sets
        .iter()
        .map(|s| Text::raw(s.name).width())
        .chain(user_themes.iter().map(|(n, _)| Text::raw(n).width()))
        .max()
        .unwrap_or(0);

    // Build render lines for presets
    let mut render_lines: Vec<render::RenderLine> = icon_sets
        .iter()
        .map(|set| {
            let mut preview_theme = theme.clone();
            set.apply_to(&mut preview_theme.components);
            let mode = if set.is_powerline() {
                if let Some(pl) = crate::presets::color_schemes::find("Powerline Dark") {
                    pl.apply_to(&mut preview_theme.components);
                }
                StyleMode::Powerline
            } else {
                preview_theme.style.mode
            };
            render::build_render_line(&preview_theme.components, mode, &texts)
        })
        .collect();

    // Build render lines for user themes (apply their icons to current theme's colors)
    let mut user_render_lines: Vec<render::RenderLine> = user_theme_data
        .iter()
        .map(|ut| {
            let mut preview_theme = theme.clone();
            for src in &ut.components {
                if let Some(dest) = preview_theme.components.iter_mut().find(|c| c.id == src.id) {
                    dest.icon = src.icon.clone();
                }
            }
            let mode = if matches!(
                ut.style.mode,
                StyleMode::Powerline | StyleMode::PlainPowerline
            ) {
                if let Some(pl) = crate::presets::color_schemes::find("Powerline Dark") {
                    pl.apply_to(&mut preview_theme.components);
                }
                StyleMode::Powerline
            } else {
                preview_theme.style.mode
            };
            render::build_render_line(&preview_theme.components, mode, &texts)
        })
        .collect();

    // Align all lines together
    let mut all_lines: Vec<&mut render::RenderLine> = render_lines
        .iter_mut()
        .chain(user_render_lines.iter_mut())
        .collect();
    render::align_lines_refs(&mut all_lines);

    let mut items: Vec<ListItem> = Vec::new();

    for (i, (set, line)) in icon_sets.iter().zip(render_lines.iter()).enumerate() {
        let preview_spans = render::render_spans(line);

        let cursor = if i == selection { "> " } else { "  " };
        let cursor_style = item_style(i, selection);
        let pad = max_name_width.saturating_sub(Text::raw(set.name).width());
        let padded_name = format!("{}{} ", set.name, " ".repeat(pad));

        let mut spans = vec![
            Span::styled(cursor.to_string(), cursor_style),
            Span::styled(padded_name, Style::default().fg(Color::DarkGray)),
        ];
        spans.extend(preview_spans);

        items.push(ListItem::new(Line::from(spans)));
    }

    if !user_themes.is_empty() && !icon_sets.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "\u{2500}\u{2500}\u{2500} User Themes \u{2500}\u{2500}\u{2500}",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    for (i, ((name, _), line)) in user_themes.iter().zip(user_render_lines.iter()).enumerate() {
        let preview_spans = render::render_spans(line);
        let idx = icon_sets.len() + i;
        let cursor = if idx == selection { "> " } else { "  " };
        let cursor_style = item_style(idx, selection);
        let pad = max_name_width.saturating_sub(Text::raw(name).width());
        let padded_name = format!("{}{} ", name, " ".repeat(pad));

        let mut spans = vec![
            Span::styled(cursor.to_string(), cursor_style),
            Span::styled(padded_name, Style::default().fg(Color::DarkGray)),
        ];
        spans.extend(preview_spans);

        items.push(ListItem::new(Line::from(spans)));
    }

    // Footer note
    let note_style = Style::default().fg(Color::DarkGray);
    items.push(ListItem::new(Line::from(Span::styled(
        "  * Colors are just a preview.",
        note_style,
    ))));
    items.push(ListItem::new(Line::from(Span::styled(
        "    Only icons will be imported.",
        note_style,
    ))));

    let height = (items.len() as u16 + 2).min(area.height.saturating_sub(4));
    let popup = centered_rect(70, height, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(" Import Icon Set ");

    let list = List::new(items).block(block);
    f.render_widget(list, popup);
}

fn item_style(index: usize, selection: usize) -> Style {
    if index == selection {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}

fn load_user_theme_data(user_themes: &[(String, std::path::PathBuf)]) -> Vec<UserTheme> {
    user_themes
        .iter()
        .filter_map(|(_, path)| manager::load_theme(path).ok())
        .collect()
}

/// Color scheme presets not already supplied by any user theme.
pub fn filter_color_schemes(
    user_theme_data: &[UserTheme],
) -> Vec<crate::presets::color_schemes::ColorScheme> {
    crate::presets::color_schemes::all()
        .into_iter()
        .filter(|s| {
            !user_theme_data
                .iter()
                .any(|t| s.is_supplied_by(&t.components))
        })
        .collect()
}

/// Icon set presets not already supplied by any user theme.
pub fn filter_icon_sets(user_theme_data: &[UserTheme]) -> Vec<crate::presets::icon_sets::IconSet> {
    crate::presets::icon_sets::all()
        .into_iter()
        .filter(|s| {
            !user_theme_data
                .iter()
                .any(|t| s.is_supplied_by(&t.components))
        })
        .collect()
}
