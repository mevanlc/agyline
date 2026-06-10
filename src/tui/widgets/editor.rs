use crate::config::theme::UserTheme;
use crate::config::types::{AnsiColor, ComponentId};
use crate::core::render;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSelection {
    Enabled,
    StyleMode,
    PlainIcon,
    NerdFontIcon,
    PerModelIcons,
    OpusIcon,
    SonnetIcon,
    HaikuIcon,
    FableIcon,
    MythosIcon,
    IconColor,
    TextColor,
    BackgroundColor,
    Bold,
}

impl FieldSelection {
    /// Build the visible field list for a given component.
    pub fn fields_for(comp: &crate::config::types::ComponentConfig) -> Vec<FieldSelection> {
        let mut fields = vec![Self::Enabled, Self::StyleMode];

        if comp.id == ComponentId::Model {
            let pm_enabled = comp.icon.per_model.as_ref().is_some_and(|pm| pm.enabled);
            if pm_enabled {
                fields.push(Self::PerModelIcons);
                fields.push(Self::OpusIcon);
                fields.push(Self::FableIcon);
                fields.push(Self::MythosIcon);
                fields.push(Self::SonnetIcon);
                fields.push(Self::HaikuIcon);
            } else {
                fields.push(Self::PlainIcon);
                fields.push(Self::NerdFontIcon);
                fields.push(Self::PerModelIcons);
            }
        } else {
            fields.push(Self::PlainIcon);
            fields.push(Self::NerdFontIcon);
        }

        fields.extend([
            Self::IconColor,
            Self::TextColor,
            Self::BackgroundColor,
            Self::Bold,
        ]);
        fields
    }

    /// Legacy fixed count for non-model components.
    pub fn count() -> usize {
        8
    }

    /// Legacy index lookup (for non-model components).
    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Self::Enabled,
            1 => Self::StyleMode,
            2 => Self::PlainIcon,
            3 => Self::NerdFontIcon,
            4 => Self::IconColor,
            5 => Self::TextColor,
            6 => Self::BackgroundColor,
            7 => Self::Bold,
            _ => Self::Enabled,
        }
    }
}

pub struct EditorWidget;

impl EditorWidget {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        theme: &UserTheme,
        selected_component: usize,
        is_focused: bool,
        selected_field: FieldSelection,
    ) {
        let comp = match theme.components.get(selected_component) {
            Some(c) => c,
            None => return,
        };

        let visible_fields = FieldSelection::fields_for(comp);

        let pm = comp.icon.per_model.as_ref();
        let field_data: Vec<(&str, String, Option<Color>)> = visible_fields
            .iter()
            .map(|f| match f {
                FieldSelection::Enabled => (
                    "Enabled",
                    if comp.enabled { "Yes" } else { "No" }.into(),
                    None,
                ),
                FieldSelection::StyleMode => {
                    ("Style Mode", theme.style.mode.display_name().into(), None)
                }
                FieldSelection::PlainIcon => ("Plain Icon", comp.icon.plain.clone(), None),
                FieldSelection::NerdFontIcon => ("Nerd Icon", comp.icon.nerd_font.clone(), None),
                FieldSelection::PerModelIcons => {
                    let enabled = pm.is_some_and(|p| p.enabled);
                    ("Per-Model", if enabled { "Yes" } else { "No" }.into(), None)
                }
                FieldSelection::OpusIcon => (
                    "Opus Icon",
                    pm.map_or(String::new(), |p| p.opus.clone()),
                    None,
                ),
                FieldSelection::FableIcon => (
                    "Fable Icon",
                    pm.map_or(String::new(), |p| p.fable.clone()),
                    None,
                ),
                FieldSelection::MythosIcon => (
                    "Mythos Icon",
                    pm.map_or(String::new(), |p| p.mythos.clone()),
                    None,
                ),
                FieldSelection::SonnetIcon => (
                    "Sonnet Icon",
                    pm.map_or(String::new(), |p| p.sonnet.clone()),
                    None,
                ),
                FieldSelection::HaikuIcon => (
                    "Haiku Icon",
                    pm.map_or(String::new(), |p| p.haiku.clone()),
                    None,
                ),
                FieldSelection::IconColor => (
                    "Icon Color",
                    format_color(comp.colors.icon.as_ref()),
                    swatch_color(comp.colors.icon.as_ref()),
                ),
                FieldSelection::TextColor => (
                    "Text Color",
                    format_color(comp.colors.text.as_ref()),
                    swatch_color(comp.colors.text.as_ref()),
                ),
                FieldSelection::BackgroundColor => (
                    "Bg Color",
                    format_color(comp.colors.background.as_ref()),
                    swatch_color(comp.colors.background.as_ref()),
                ),
                FieldSelection::Bold => (
                    "Bold",
                    if comp.styles.text_bold { "Yes" } else { "No" }.into(),
                    None,
                ),
            })
            .collect();

        let label_col_width = field_data
            .iter()
            .map(|(l, _, _)| l.len())
            .max()
            .unwrap_or(0)
            + 1;

        let all_items: Vec<ListItem> = field_data
            .iter()
            .enumerate()
            .map(|(i, (label, value, swatch))| {
                let field = visible_fields[i];
                let is_selected = field == selected_field && is_focused;

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                let cursor = if is_selected { "> " } else { "  " };

                let mut spans = vec![
                    Span::styled(cursor, style),
                    Span::styled(
                        format!(
                            "{:<width$} ",
                            format!("{}:", label),
                            width = label_col_width
                        ),
                        style,
                    ),
                    Span::styled(value.clone(), Style::default().fg(Color::White)),
                ];

                if let Some(color) = swatch {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        "\u{2588}\u{2588}",
                        Style::default().fg(*color),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let total = all_items.len();
        let selected_idx = visible_fields
            .iter()
            .position(|f| *f == selected_field)
            .unwrap_or(0);
        let inner_height = area.height.saturating_sub(2) as usize; // borders

        let items = if total <= inner_height {
            all_items
        } else {
            // Both arrows always shown; visible slots = inner_height - 2
            let visible = inner_height.saturating_sub(2);
            let half = visible / 2;
            let raw_offset = selected_idx.saturating_sub(half);
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

        let title = Line::from(vec![
            Span::styled(format!(" {} ", comp.id.display_name()), border_style),
            Span::styled(
                format!("- {} ", comp.id.description()),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(title);

        let list = List::new(items).block(block);
        f.render_widget(list, area);
    }
}

fn format_color(color: Option<&AnsiColor>) -> String {
    match color {
        Some(c) => c.to_string(),
        None => "\u{2014}".into(), // —
    }
}

fn swatch_color(color: Option<&AnsiColor>) -> Option<Color> {
    color.map(render::ansi_to_ratatui_color)
}
