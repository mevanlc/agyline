use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct Geometry {
    pub context: Rect,
    pub tabs: Rect,
    pub resources: Rect,
    pub body: Rect,
    pub preview: Rect,
    pub status: Rect,
    pub help: Rect,
    pub list: Rect,
    pub inspector: Rect,
}
impl Geometry {
    pub fn new(area: Rect, inspector_focused: bool) -> Self {
        let preview_height = if area.width >= 80 && area.height >= 30 {
            11
        } else if area.height >= 22 {
            3
        } else {
            1
        };
        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(preview_height),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);
        let (list, inspector) = if area.width >= 80 {
            let panels =
                Layout::horizontal([Constraint::Length(31), Constraint::Min(0)]).split(rows[3]);
            (panels[0], panels[1])
        } else if inspector_focused {
            (Rect::default(), rows[3])
        } else {
            (rows[3], Rect::default())
        };
        Self {
            context: rows[0],
            tabs: rows[1],
            resources: rows[2],
            body: rows[3],
            preview: rows[4],
            status: rows[5],
            help: rows[6],
            list,
            inspector,
        }
    }
}
pub fn clip(text: &str, width: usize) -> String {
    if text.width() <= width {
        return text.to_string();
    }
    if width == 0 {
        return String::new();
    }
    let mut result = String::new();
    let mut used = 0;
    for g in text.graphemes(true) {
        let w = g.width();
        if used + w > width - 1 {
            break;
        }
        result.push_str(g);
        used += w;
    }
    result.push('…');
    result
}
pub fn clip_line(line: Line<'static>, width: usize) -> Line<'static> {
    if line.width() <= width {
        return line;
    }
    let mut spans = Vec::new();
    let mut remaining = width.saturating_sub(1);
    for span in line.spans {
        let mut content = String::new();
        for g in span.content.graphemes(true) {
            if g.width() > remaining {
                break;
            }
            remaining -= g.width();
            content.push_str(g);
        }
        let truncated = content.len() < span.content.len();
        spans.push(Span::styled(content, span.style));
        if truncated {
            break;
        }
    }
    if width > 0 {
        spans.push(Span::raw("…"));
    }
    Line::from(spans)
}
pub fn popup(area: Rect) -> Rect {
    if area.width < 80 || area.height < 24 {
        return area;
    }
    let width = area.width.min(84);
    let height = area.height.min(24);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn agreed_sizes_preserve_editor_rows_and_preview_breakpoints() {
        for (w, h, content, preview) in [
            (60, 18, 10, 1),
            (80, 18, 10, 1),
            (80, 24, 14, 3),
            (80, 25, 15, 3),
            (80, 30, 12, 11),
            (120, 40, 22, 11),
        ] {
            let g = Geometry::new(Rect::new(0, 0, w, h), false);
            assert_eq!(g.body.height - 2, content, "{w}x{h}");
            assert_eq!(g.preview.height, preview);
            assert_eq!(g.list.width, if w < 80 { w } else { 31 });
        }
    }
    #[test]
    fn clipping_preserves_graphemes_and_obeys_display_width() {
        for value in ["a👩‍💻b", "色彩設計", "e\u{301}cho", "a very long name"] {
            for width in 0..12 {
                let clipped = clip(value, width);
                assert!(clipped.width() <= width);
            }
        }
        assert_eq!(clip("👩‍💻abcdef", 3), "👩‍💻…");
    }
}
