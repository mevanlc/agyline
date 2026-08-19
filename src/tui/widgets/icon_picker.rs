use crate::data::icon_catalog::{IconCatalogData, IconEntry, IconPickerTab, SectionView};
use crate::tui::app::IconPickerState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

/// Total number of selectable (non-header) entries across all sections.
pub fn selectable_count(sections: &[SectionView<'_>]) -> usize {
    sections.iter().map(|s| s.entries.len()).sum()
}

/// Total number of flat rows (1 header + N entries per section).
pub fn flat_len(sections: &[SectionView<'_>]) -> usize {
    sections.iter().map(|s| s.entries.len() + 1).sum()
}

/// Map a selectable index → flat row index.
pub fn selectable_to_flat(sections: &[SectionView<'_>], sel: usize) -> Option<usize> {
    let mut flat = 0;
    let mut remaining = sel;
    for s in sections {
        flat += 1; // header
        let len = s.entries.len();
        if remaining < len {
            return Some(flat + remaining);
        }
        remaining -= len;
        flat += len;
    }
    None
}

/// Look up the IconEntry at a given selectable index.
pub fn entry_at_selectable<'a>(
    sections: &'a [SectionView<'a>],
    sel: usize,
) -> Option<&'a IconEntry> {
    let mut remaining = sel;
    for s in sections {
        let len = s.entries.len();
        if remaining < len {
            return s.entries.get(remaining);
        }
        remaining -= len;
    }
    None
}

pub fn render(f: &mut Frame, area: Rect, state: &IconPickerState, catalog: &IconCatalogData) {
    let popup = centered_rect(56, 26, area);
    f.render_widget(Clear, popup);

    let tab = *state.tab.current();
    let is_custom = tab == IconPickerTab::Custom;

    // Overall layout: outer block, then vertical sections inside
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Icon Picker ");

    let inner = outer_block.inner(popup);
    f.render_widget(outer_block, popup);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Icon Set tabs
            Constraint::Length(3), // Search / Input
            Constraint::Min(3),    // Icons list (or empty for Custom)
            Constraint::Length(3), // Keymap
        ])
        .split(inner);

    // --- Icon Set tabs ---
    render_tabs(f, layout[0], state);

    // --- Search / Input ---
    if is_custom {
        render_custom_input(f, layout[1], state);
    } else {
        render_search(f, layout[1], state);
    }

    // --- Icons list ---
    if is_custom {
        // Empty area for custom tab
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(236)));
        f.render_widget(block, layout[2]);
    } else {
        render_icon_list(f, layout[2], state, catalog);
    }

    // --- Keymap ---
    render_keymap(
        f,
        layout[3],
        &[
            ("Tab", "Switch Set"),
            ("\u{2190}\u{2192}", "Cursor"),
            ("\u{2191}\u{2193}", "Navigate"),
            ("Enter", "Select"),
            ("Esc", "Cancel"),
        ],
    );
}

fn render_tabs(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let tabs = [
        ("Emoji", IconPickerTab::Emoji),
        ("Nerd Font", IconPickerTab::NerdFont),
        ("Unicode", IconPickerTab::Unicode),
        ("Custom", IconPickerTab::Custom),
    ];

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));
    for (i, (label, tab)) in tabs.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default().fg(Color::DarkGray)));
        }
        let selected = state.tab == *tab;
        let indicator = if selected { "\u{2022}" } else { " " };
        let style = if selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Indexed(245))
        };
        spans.push(Span::styled(format!("[{}] {}", indicator, label), style));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Icon Set ");

    let line = Line::from(spans);
    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}

fn render_search(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let mut textarea = state.search_textarea.clone();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(Span::styled(" Search ", Style::default().fg(Color::Yellow)));

    textarea.set_block(block);
    textarea.set_cursor_line_style(Style::default());
    f.render_widget(&textarea, area);
}

fn render_custom_input(f: &mut Frame, area: Rect, state: &IconPickerState) {
    let mut textarea = state.custom_textarea.clone();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue))
        .title(Span::styled(" Input ", Style::default().fg(Color::Yellow)));

    textarea.set_block(block);
    textarea.set_cursor_line_style(Style::default());
    f.render_widget(&textarea, area);
}

fn render_icon_list(f: &mut Frame, area: Rect, state: &IconPickerState, catalog: &IconCatalogData) {
    let tab = *state.tab.current();
    let sections = catalog.sections(tab, state.search_query());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Icons ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible_height = inner.height as usize;
    if visible_height == 0 {
        return;
    }

    let total_flat = flat_len(&sections);
    let selected_flat = selectable_to_flat(&sections, state.selected_index);
    let scroll = state.scroll_offset;
    let view_end = scroll + visible_height;

    // Walk sections and only emit Line widgets for rows intersecting [scroll, view_end).
    // This avoids materializing the full 100k-row flat list for the Unicode tab.
    let mut lines: Vec<Line> = Vec::with_capacity(visible_height);
    let mut row = 0usize;
    'outer: for section in &sections {
        if row >= view_end {
            break;
        }
        // Header row
        if row >= scroll && row < view_end {
            let dashes = "\u{2500}".repeat(3);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{}\u{2500}{} ", dashes, dashes),
                    Style::default().fg(Color::Indexed(240)),
                ),
                Span::styled(section.title, Style::default().fg(Color::Indexed(245))),
                Span::styled(
                    format!(" {}", dashes),
                    Style::default().fg(Color::Indexed(240)),
                ),
            ]));
            if lines.len() == visible_height {
                break 'outer;
            }
        }
        row += 1;
        let entries_len = section.entries.len();
        let entries_end = row + entries_len;

        // Visible window inside this section's entries.
        let vis_start = scroll.max(row);
        let vis_end = view_end.min(entries_end);
        if vis_start < vis_end {
            for flat_row in vis_start..vis_end {
                let entry_idx = flat_row - row;
                let Some(entry) = section.entries.get(entry_idx) else {
                    break;
                };
                let is_selected = Some(flat_row) == selected_flat;
                let icon = &entry.icon;
                let name = &entry.name;
                if is_selected {
                    lines.push(Line::from(vec![
                        Span::styled(format!(" {} ", icon), Style::default().fg(Color::White)),
                        Span::styled(
                            name.clone(),
                            Style::default()
                                .fg(Color::White)
                                .bg(Color::Indexed(238))
                                .add_modifier(Modifier::BOLD),
                        ),
                        // Fill remaining width with selection background
                        Span::styled(
                            " ".repeat(
                                inner
                                    .width
                                    .saturating_sub((icon.chars().count() + name.len() + 3) as u16)
                                    as usize,
                            ),
                            Style::default().bg(Color::Indexed(238)),
                        ),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::styled(format!(" {} ", icon), Style::default().fg(Color::White)),
                        Span::styled(name.clone(), Style::default().fg(Color::Indexed(250))),
                    ]));
                }
                if lines.len() == visible_height {
                    break 'outer;
                }
            }
        }
        row = entries_end;
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);

    // Page counter in bottom-right of the Icons block border
    if total_flat > 0 {
        let total_pages = total_flat.div_ceil(visible_height);
        let current_page = scroll / visible_height + 1;
        let counter = format!(" Page {}/{} ", current_page, total_pages);
        let counter_width = counter.len() as u16;
        let counter_area = Rect {
            x: area.x + area.width.saturating_sub(counter_width + 1),
            y: area.y + area.height - 1,
            width: counter_width,
            height: 1,
        };
        f.render_widget(
            Paragraph::new(Span::styled(
                counter,
                Style::default().fg(Color::Indexed(240)),
            )),
            counter_area,
        );
    }
}

fn render_keymap(f: &mut Frame, area: Rect, hints: &[(&str, &str)]) {
    let line = super::key_hints::render(hints);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(236)))
        .title(" Keymap ");

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [vert] = vertical.areas(area);
    let [rect] = horizontal.areas(vert);
    rect
}
