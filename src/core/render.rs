use std::collections::HashMap;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use unicode_width::UnicodeWidthStr;

use crate::config::types::{
    AnsiColor, ComponentConfig, ComponentId, DEFAULT_GIT_AUTOHIDE_BRANCH,
    DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL, GIT_OPTION_AUTOHIDE_BRANCH,
    PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL, StyleMode, UsageValue, WorktreeOutside,
};

const POWERLINE_SEPARATOR_GLYPH: &str = "\u{e0b0}";
const PLAIN_POWERLINE_SEPARATOR_GLYPH: &str = "\u{25ba}";

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// A fully-resolved segment ready for rendering.
pub struct Segment {
    pub id: ComponentId,
    pub icon: String,
    pub text: String,
    pub secondary: String,
    pub icon_color: Option<AnsiColor>,
    pub text_color: Option<AnsiColor>,
    pub bg: Option<AnsiColor>,
    pub text_bold: bool,
}

/// Separator between two adjacent segments, fully resolved.
pub struct SepToken {
    pub glyph: String,
    pub fg: Option<AnsiColor>,
    pub bg: Option<AnsiColor>,
}

pub enum RenderItem {
    Seg(Segment),
    Sep(SepToken),
}

/// A render-ready statusline: alternating segments and separators.
pub struct RenderLine {
    pub items: Vec<RenderItem>,
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Text content for a single component: (primary, secondary).
pub type SegmentText = (String, String);

/// Build a render-ready line from theme components, style mode, and text content.
///
/// `texts` maps ComponentId → (primary, secondary). Components missing from the
/// map are skipped (useful for real statusline where a collector returned None).
pub fn build_render_line(
    components: &[ComponentConfig],
    mode: StyleMode,
    texts: &HashMap<ComponentId, SegmentText>,
) -> RenderLine {
    let is_powerline = matches!(mode, StyleMode::Powerline | StyleMode::PlainPowerline);

    // Gather separator info
    let sep_cfg = components.iter().find(|c| c.id == ComponentId::Separator);
    let sep_enabled = sep_cfg.is_some_and(|s| s.enabled);
    let sep_glyph = sep_cfg
        .map(|s| match mode {
            StyleMode::Plain | StyleMode::PlainPowerline => s.icon.plain.as_str(),
            StyleMode::NerdFont | StyleMode::Powerline => s.icon.nerd_font.as_str(),
        })
        .unwrap_or(" | ");
    let sep_icon_color = sep_cfg.and_then(|s| s.colors.icon.clone());

    // Build segments for enabled, non-separator components that have text
    let enabled: Vec<&ComponentConfig> = components
        .iter()
        .filter(|c| c.enabled && c.id != ComponentId::Separator && texts.contains_key(&c.id))
        .collect();

    let mut items: Vec<RenderItem> = Vec::new();

    for (i, comp) in enabled.iter().enumerate() {
        // Insert separator before every segment after the first
        if i > 0 {
            if is_powerline {
                let prev_bg = enabled[i - 1].colors.background.clone();
                let curr_bg = comp.colors.background.clone();
                items.push(RenderItem::Sep(SepToken {
                    glyph: powerline_separator_glyph(mode).into(),
                    fg: prev_bg,
                    bg: curr_bg,
                }));
            } else if sep_enabled {
                items.push(RenderItem::Sep(SepToken {
                    glyph: sep_glyph.to_string(),
                    fg: sep_icon_color.clone(),
                    bg: None,
                }));
            }
        }

        let icon = match mode {
            StyleMode::Plain | StyleMode::PlainPowerline => comp.icon.plain.clone(),
            StyleMode::NerdFont | StyleMode::Powerline => comp.icon.nerd_font.clone(),
        };

        let (primary, secondary) = texts.get(&comp.id).cloned().unwrap_or_default();

        items.push(RenderItem::Seg(Segment {
            id: comp.id,
            icon,
            text: primary,
            secondary,
            icon_color: comp.colors.icon.clone(),
            text_color: comp.colors.text.clone(),
            bg: comp.colors.background.clone(),
            text_bold: comp.styles.text_bold,
        }));
    }

    RenderLine { items }
}

fn powerline_separator_glyph(mode: StyleMode) -> &'static str {
    match mode {
        StyleMode::PlainPowerline => PLAIN_POWERLINE_SEPARATOR_GLYPH,
        StyleMode::Powerline => POWERLINE_SEPARATOR_GLYPH,
        StyleMode::Plain | StyleMode::NerdFont => unreachable!("non-powerline mode"),
    }
}

/// Pad icons across multiple RenderLines so segment columns align.
///
/// For each segment position, finds the max icon display width across all
/// lines and right-pads shorter icons with spaces to match.
pub fn align_lines(lines: &mut [RenderLine]) {
    // Collect segment icon widths per position across all lines
    let max_positions = lines
        .iter()
        .map(|l| {
            l.items
                .iter()
                .filter(|i| matches!(i, RenderItem::Seg(_)))
                .count()
        })
        .max()
        .unwrap_or(0);

    // Find max icon width at each segment position
    let mut max_widths = vec![0usize; max_positions];
    for line in lines.iter() {
        let mut pos = 0;
        for item in &line.items {
            if let RenderItem::Seg(seg) = item {
                if pos < max_positions {
                    let w = UnicodeWidthStr::width(seg.icon.as_str());
                    max_widths[pos] = max_widths[pos].max(w);
                }
                pos += 1;
            }
        }
    }

    // Pad each icon to its column's max width
    for line in lines.iter_mut() {
        let mut pos = 0;
        for item in &mut line.items {
            if let RenderItem::Seg(seg) = item {
                if pos < max_positions {
                    let w = UnicodeWidthStr::width(seg.icon.as_str());
                    let pad = max_widths[pos].saturating_sub(w);
                    if pad > 0 {
                        seg.icon.push_str(&" ".repeat(pad));
                    }
                }
                pos += 1;
            }
        }
    }
}

/// Like `align_lines` but takes `&mut [&mut RenderLine]` so you can align
/// across separately-owned collections.
pub fn align_lines_refs(lines: &mut [&mut RenderLine]) {
    let max_positions = lines
        .iter()
        .map(|l| {
            l.items
                .iter()
                .filter(|i| matches!(i, RenderItem::Seg(_)))
                .count()
        })
        .max()
        .unwrap_or(0);

    let mut max_widths = vec![0usize; max_positions];
    for line in lines.iter() {
        let mut pos = 0;
        for item in &line.items {
            if let RenderItem::Seg(seg) = item {
                if pos < max_positions {
                    let w = UnicodeWidthStr::width(seg.icon.as_str());
                    max_widths[pos] = max_widths[pos].max(w);
                }
                pos += 1;
            }
        }
    }

    for line in lines.iter_mut() {
        let mut pos = 0;
        for item in &mut line.items {
            if let RenderItem::Seg(seg) = item {
                if pos < max_positions {
                    let w = UnicodeWidthStr::width(seg.icon.as_str());
                    let pad = max_widths[pos].saturating_sub(w);
                    if pad > 0 {
                        seg.icon.push_str(&" ".repeat(pad));
                    }
                }
                pos += 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Demo text helpers
// ---------------------------------------------------------------------------

/// Full demo text for the TUI preview bar.
pub fn demo_texts_full() -> HashMap<ComponentId, SegmentText> {
    HashMap::from([
        (ComponentId::AgentState, ("idle".into(), String::new())),
        (
            ComponentId::Model,
            ("Gemini 3.5 Flash".into(), String::new()),
        ),
        (ComponentId::Directory, ("project".into(), String::new())),
        (ComponentId::Git, ("main".into(), "\u{2713}".into())),
        (
            ComponentId::ContextWindow,
            ("14% \u{b7} 88k tokens".into(), String::new()),
        ),
        (ComponentId::Quota, ("weekly 94%".into(), String::new())),
        (ComponentId::TaskCount, ("1 task".into(), String::new())),
        (
            ComponentId::ExecutionMode,
            ("planning".into(), String::new()),
        ),
        (ComponentId::VimMode, ("NORMAL".into(), String::new())),
        (
            ComponentId::ArtifactCount,
            ("2 artifacts".into(), String::new()),
        ),
        (
            ComponentId::PendingInput,
            ("1 pending".into(), String::new()),
        ),
        (
            ComponentId::ToolConfirmation,
            ("confirming".into(), String::new()),
        ),
        (ComponentId::Sandbox, ("sandbox".into(), String::new())),
        (ComponentId::PlanTier, ("Pro".into(), String::new())),
        (
            ComponentId::Email,
            ("developer@email.com".into(), String::new()),
        ),
        (ComponentId::Worktree, ("refactor".into(), String::new())),
        (ComponentId::Hostname, ("workstation".into(), String::new())),
        (ComponentId::PullRequest, ("#482".into(), "approved".into())),
        (ComponentId::UsageFiveHour, ("5h 23%".into(), String::new())),
        (ComponentId::UsageSevenDay, ("7d 41%".into(), String::new())),
        (ComponentId::Cost, ("$1.23".into(), String::new())),
        (ComponentId::Session, ("5m".into(), String::new())),
        (ComponentId::OutputStyle, ("concise".into(), String::new())),
    ])
}

/// Full demo text adjusted for component-specific display options.
pub fn demo_texts_for_components(
    components: &[ComponentConfig],
) -> HashMap<ComponentId, SegmentText> {
    let mut texts = demo_texts_full();

    for (id, label, used) in [
        (ComponentId::UsageFiveHour, "5h", 23.0),
        (ComponentId::UsageSevenDay, "7d", 41.0),
    ] {
        let Some(usage) = components.iter().find(|component| component.id == id) else {
            continue;
        };
        let percentage = UsageValue::from_options(&usage.options).apply(used);
        texts.insert(id, (format!("{label} {percentage:.0}%"), String::new()));
    }

    let mut worktree_displays_branch = false;
    if let Some(worktree) = components
        .iter()
        .find(|component| component.id == ComponentId::Worktree)
    {
        match WorktreeOutside::from_options(&worktree.options) {
            WorktreeOutside::Hide => {
                texts.remove(&ComponentId::Worktree);
            }
            WorktreeOutside::Show => {
                texts.insert(ComponentId::Worktree, ("-".into(), String::new()));
            }
            WorktreeOutside::Branch => {
                texts.insert(ComponentId::Worktree, ("main".into(), String::new()));
                worktree_displays_branch = worktree.enabled;
            }
            WorktreeOutside::Directory => {
                texts.insert(ComponentId::Worktree, ("project".into(), String::new()));
            }
        }
    }

    let git_autohides_branch = components
        .iter()
        .find(|component| component.id == ComponentId::Git)
        .is_some_and(|git| {
            git.options
                .get(GIT_OPTION_AUTOHIDE_BRANCH)
                .and_then(|value| value.as_bool())
                .unwrap_or(DEFAULT_GIT_AUTOHIDE_BRANCH)
        });
    if worktree_displays_branch
        && git_autohides_branch
        && let Some((primary, secondary)) = texts.get_mut(&ComponentId::Git)
        && primary == "main"
    {
        *primary = std::mem::take(secondary);
    }

    let Some(pull_request) = components
        .iter()
        .find(|component| component.id == ComponentId::PullRequest)
    else {
        return texts;
    };

    let show_review_state = pull_request
        .options
        .get(PR_OPTION_SHOW_REVIEW_STATE)
        .and_then(|value| value.as_bool())
        .unwrap_or(DEFAULT_PR_SHOW_REVIEW_STATE);
    let show_url = pull_request
        .options
        .get(PR_OPTION_SHOW_URL)
        .and_then(|value| value.as_bool())
        .unwrap_or(DEFAULT_PR_SHOW_URL);
    let mut secondary = Vec::new();
    if show_review_state {
        secondary.push("approved");
    }
    if show_url {
        secondary.push("https://github.com/example/repo/pull/482");
    }
    texts.insert(
        ComponentId::PullRequest,
        ("#482".into(), secondary.join(" ")),
    );
    texts
}

/// Compact demo text for import menu previews.
pub fn demo_texts_compact() -> HashMap<ComponentId, SegmentText> {
    HashMap::from([
        (ComponentId::Model, ("Model".into(), String::new())),
        (ComponentId::Directory, ("prj".into(), String::new())),
        (ComponentId::Worktree, ("wt".into(), String::new())),
        (ComponentId::Hostname, ("host".into(), String::new())),
        (ComponentId::Git, ("main".into(), "\u{2713}".into())),
        (ComponentId::PullRequest, ("#482".into(), String::new())),
        (ComponentId::ContextWindow, ("12%".into(), String::new())),
        (ComponentId::UsageFiveHour, ("5h 23%".into(), String::new())),
        (ComponentId::UsageSevenDay, ("7d 41%".into(), String::new())),
        (ComponentId::Cost, ("$1".into(), String::new())),
        (ComponentId::Session, ("5m".into(), String::new())),
        (ComponentId::OutputStyle, ("concise".into(), String::new())),
    ])
}

/// Build texts from real ComponentData (for statusline mode).
pub fn texts_from_data(
    data: &[(ComponentConfig, crate::core::components::ComponentData)],
) -> HashMap<ComponentId, SegmentText> {
    data.iter()
        .map(|(cfg, d)| (cfg.id, (d.primary.clone(), d.secondary.clone())))
        .collect()
}

/// Build a texts map that also carries dynamic_icon metadata.
/// Returns (texts, dynamic_icons) where dynamic_icons maps ComponentId → icon override.
pub fn texts_and_icons_from_data(
    data: &[(ComponentConfig, crate::core::components::ComponentData)],
) -> (
    HashMap<ComponentId, SegmentText>,
    HashMap<ComponentId, String>,
) {
    texts_and_icons_from_data_for_mode(data, StyleMode::Plain)
}

/// Build a texts map that carries dynamic_icon metadata for a specific style mode.
pub fn texts_and_icons_from_data_for_mode(
    data: &[(ComponentConfig, crate::core::components::ComponentData)],
    mode: StyleMode,
) -> (
    HashMap<ComponentId, SegmentText>,
    HashMap<ComponentId, String>,
) {
    let mut texts = HashMap::new();
    let mut icons = HashMap::new();
    for (cfg, d) in data {
        texts.insert(cfg.id, (d.primary.clone(), d.secondary.clone()));
        let dynamic = match mode {
            StyleMode::Plain | StyleMode::PlainPowerline => d
                .metadata
                .get("dynamic_icon_plain")
                .or_else(|| d.metadata.get("dynamic_icon")),
            StyleMode::NerdFont | StyleMode::Powerline => d
                .metadata
                .get("dynamic_icon_nerd_font")
                .or_else(|| d.metadata.get("dynamic_icon")),
        };
        if let Some(icon) = dynamic {
            icons.insert(cfg.id, icon.clone());
        }
    }
    (texts, icons)
}

// ---------------------------------------------------------------------------
// Ratatui Span backend
// ---------------------------------------------------------------------------

/// Convert a RenderLine to ratatui Spans for TUI display.
pub fn render_spans(line: &RenderLine) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    for item in &line.items {
        match item {
            RenderItem::Sep(sep) => {
                let style = ansi_to_style_with_bg(sep.fg.as_ref(), sep.bg.as_ref(), false);
                spans.push(Span::styled(sep.glyph.clone(), style));
            }
            RenderItem::Seg(seg) => {
                let icon_style =
                    ansi_to_style_with_bg(seg.icon_color.as_ref(), seg.bg.as_ref(), false);
                let text_style =
                    ansi_to_style_with_bg(seg.text_color.as_ref(), seg.bg.as_ref(), seg.text_bold);

                if seg.bg.is_some() {
                    if seg.icon.is_empty() {
                        spans.push(Span::styled(" ".to_string(), icon_style));
                    } else {
                        spans.push(Span::styled(format!(" {} ", seg.icon), icon_style));
                    }
                    if !seg.text.is_empty() {
                        spans.push(Span::styled(format!("{} ", seg.text), text_style));
                    }
                    if !seg.secondary.is_empty() {
                        spans.push(Span::styled(format!("{} ", seg.secondary), text_style));
                    }
                } else if seg.text.is_empty() && seg.secondary.is_empty() {
                    // Icons-only mode: just the icon, no trailing space
                    spans.push(Span::styled(seg.icon.clone(), icon_style));
                } else if seg.icon.is_empty() {
                    spans.push(Span::styled(seg.text.clone(), text_style));
                    if !seg.secondary.is_empty() {
                        spans.push(Span::styled(format!(" {}", seg.secondary), text_style));
                    }
                } else {
                    spans.push(Span::styled(format!("{} ", seg.icon), icon_style));
                    spans.push(Span::styled(seg.text.clone(), text_style));
                    if !seg.secondary.is_empty() {
                        spans.push(Span::styled(format!(" {}", seg.secondary), text_style));
                    }
                }
            }
        }
    }

    if spans.is_empty() {
        spans.push(Span::raw("(no enabled components)"));
    }

    spans
}

// ---------------------------------------------------------------------------
// ANSI string backend
// ---------------------------------------------------------------------------

/// Convert a RenderLine to an ANSI escape-coded string for terminal output.
pub fn render_ansi(line: &RenderLine) -> String {
    let mut result = String::new();
    let needs_reset = line.items.iter().any(|item| match item {
        RenderItem::Seg(seg) => seg.bg.is_some(),
        RenderItem::Sep(sep) => sep.bg.is_some(),
    });

    for item in &line.items {
        match item {
            RenderItem::Sep(sep) => {
                result.push_str(&ansi_sep(sep));
            }
            RenderItem::Seg(seg) => {
                result.push_str(&ansi_segment(seg));
            }
        }
    }

    if needs_reset && !result.is_empty() {
        result.push_str("\x1b[0m");
    }

    result
}

fn ansi_segment(seg: &Segment) -> String {
    if let Some(bg) = &seg.bg {
        let bg_code = bg_ansi_code(bg);
        let text_styled = style_no_reset(&seg.text, seg.text_color.as_ref(), seg.text_bold);
        let mut content = if seg.icon.is_empty() {
            format!(" {} ", text_styled)
        } else {
            let icon_colored = match &seg.icon_color {
                Some(c) => fg_ansi_no_reset(c, &seg.icon),
                None => seg.icon.clone(),
            };
            format!(" {} {} ", icon_colored, text_styled)
        };
        if !seg.secondary.is_empty() {
            let sec = style_no_reset(&seg.secondary, seg.text_color.as_ref(), seg.text_bold);
            content.push_str(&format!("{} ", sec));
        }
        format!("{}{}\x1b[49m", bg_code, content)
    } else {
        let text_styled = apply_style(&seg.text, seg.text_color.as_ref(), seg.text_bold);
        let mut out = if seg.icon.is_empty() {
            text_styled
        } else {
            let icon_colored = apply_color(&seg.icon, seg.icon_color.as_ref());
            format!("{} {}", icon_colored, text_styled)
        };
        if !seg.secondary.is_empty() {
            out.push_str(&format!(
                " {}",
                apply_style(&seg.secondary, seg.text_color.as_ref(), seg.text_bold)
            ));
        }
        out
    }
}

fn ansi_sep(sep: &SepToken) -> String {
    match (&sep.fg, &sep.bg) {
        (Some(fg), Some(bg)) => {
            format!(
                "{}{}{}\x1b[0m",
                bg_ansi_code(bg),
                fg_ansi_code(fg),
                sep.glyph
            )
        }
        (Some(fg), None) => {
            format!("{}{}\x1b[0m", fg_ansi_code(fg), sep.glyph)
        }
        (None, Some(bg)) => {
            format!("{}{}\x1b[0m", bg_ansi_code(bg), sep.glyph)
        }
        (None, None) => sep.glyph.clone(),
    }
}

// ---------------------------------------------------------------------------
// ANSI escape helpers
// ---------------------------------------------------------------------------

// ANSI SGR base codes for the 16-color palette
const ANSI_FG_NORMAL_BASE: u8 = 30; // colors 0–7  → SGR 30–37
const ANSI_FG_BRIGHT_BASE: u8 = 90; // colors 8–15 → SGR 90–97
const ANSI_BG_NORMAL_BASE: u8 = 40; // colors 0–7  → SGR 40–47
const ANSI_BG_BRIGHT_BASE: u8 = 100; // colors 8–15 → SGR 100–107

fn apply_color(text: &str, color: Option<&AnsiColor>) -> String {
    match color {
        Some(c) => format!("{}{}\x1b[0m", fg_ansi_code(c), text),
        None => text.to_string(),
    }
}

fn apply_style(text: &str, color: Option<&AnsiColor>, bold: bool) -> String {
    let mut codes = Vec::new();
    if bold {
        codes.push("1".into());
    }
    if let Some(c) = color {
        codes.push(fg_sgr(c));
    }
    if codes.is_empty() {
        text.to_string()
    } else {
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
    }
}

fn fg_ansi_no_reset(color: &AnsiColor, text: &str) -> String {
    format!("{}{}", fg_ansi_code(color), text)
}

fn style_no_reset(text: &str, color: Option<&AnsiColor>, bold: bool) -> String {
    let mut codes = Vec::new();
    if bold {
        codes.push("1".into());
    }
    if let Some(c) = color {
        codes.push(fg_sgr(c));
    }
    if codes.is_empty() {
        text.to_string()
    } else {
        format!("\x1b[{}m{}", codes.join(";"), text)
    }
}

fn fg_ansi_code(color: &AnsiColor) -> String {
    format!("\x1b[{}m", fg_sgr(color))
}

fn fg_sgr(color: &AnsiColor) -> String {
    match color {
        AnsiColor::Color16 { c16 } => {
            if *c16 < 8 {
                (ANSI_FG_NORMAL_BASE + c16).to_string()
            } else {
                (ANSI_FG_BRIGHT_BASE + (c16 - 8)).to_string()
            }
        }
        AnsiColor::Color256 { c256 } => format!("38;5;{}", c256),
        AnsiColor::Rgb { r, g, b } => format!("38;2;{};{};{}", r, g, b),
    }
}

fn bg_ansi_code(color: &AnsiColor) -> String {
    match color {
        AnsiColor::Color16 { c16 } => {
            let code = if *c16 < 8 {
                ANSI_BG_NORMAL_BASE + c16
            } else {
                ANSI_BG_BRIGHT_BASE + (c16 - 8)
            };
            format!("\x1b[{}m", code)
        }
        AnsiColor::Color256 { c256 } => format!("\x1b[48;5;{}m", c256),
        AnsiColor::Rgb { r, g, b } => format!("\x1b[48;2;{};{};{}m", r, g, b),
    }
}

// ---------------------------------------------------------------------------
// Ratatui color conversion helpers
// ---------------------------------------------------------------------------

fn ansi_to_style_with_bg(fg: Option<&AnsiColor>, bg: Option<&AnsiColor>, bold: bool) -> Style {
    let mut style = Style::default();
    if let Some(c) = fg {
        style = style.fg(ansi_to_ratatui_color(c));
    }
    if let Some(c) = bg {
        style = style.bg(ansi_to_ratatui_color(c));
    }
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    style
}

pub fn ansi_to_ratatui_color(color: &AnsiColor) -> Color {
    match color {
        AnsiColor::Color16 { c16 } => match c16 {
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
        },
        AnsiColor::Color256 { c256 } => Color::Indexed(*c256),
        AnsiColor::Rgb { r, g, b } => Color::Rgb(*r, *g, *b),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::config::theme::UserTheme;
    use crate::config::types::{
        ComponentId, GIT_OPTION_AUTOHIDE_BRANCH, PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL,
        StyleMode, WORKTREE_OPTION_OUTSIDE_WORKTREES, WorktreeOutside,
    };

    use super::{
        PLAIN_POWERLINE_SEPARATOR_GLYPH, RenderItem, build_render_line, demo_texts_for_components,
        render_ansi,
    };

    #[test]
    fn pull_request_demo_tracks_visible_fields() {
        let mut theme = UserTheme::default_theme();
        let pull_request = theme.get_component_mut(ComponentId::PullRequest).unwrap();
        pull_request.options.insert(
            PR_OPTION_SHOW_REVIEW_STATE.into(),
            serde_json::Value::Bool(false),
        );
        pull_request
            .options
            .insert(PR_OPTION_SHOW_URL.into(), serde_json::Value::Bool(true));

        let texts = demo_texts_for_components(&theme.components);
        assert_eq!(
            texts.get(&ComponentId::PullRequest),
            Some(&(
                "#482".into(),
                "https://github.com/example/repo/pull/482".into()
            ))
        );
    }

    #[test]
    fn worktree_demo_tracks_the_outside_worktrees_mode() {
        let mut theme = UserTheme::default_theme();
        for (mode, expected) in [
            (WorktreeOutside::Hide, None),
            (WorktreeOutside::Show, Some("-")),
            (WorktreeOutside::Branch, Some("main")),
            (WorktreeOutside::Directory, Some("project")),
        ] {
            theme
                .get_component_mut(ComponentId::Worktree)
                .unwrap()
                .options
                .insert(
                    WORKTREE_OPTION_OUTSIDE_WORKTREES.into(),
                    mode.as_str().into(),
                );

            let texts = demo_texts_for_components(&theme.components);
            assert_eq!(
                texts
                    .get(&ComponentId::Worktree)
                    .map(|(primary, _)| primary.as_str()),
                expected
            );
        }
    }

    #[test]
    fn git_demo_autohides_only_a_branch_displayed_by_worktree() {
        let mut theme = UserTheme::default_theme();
        theme.get_component_mut(ComponentId::Git).unwrap().enabled = true;
        theme
            .get_component_mut(ComponentId::Worktree)
            .unwrap()
            .enabled = true;

        let texts = demo_texts_for_components(&theme.components);
        assert_eq!(
            texts.get(&ComponentId::Git),
            Some(&("\u{2713}".into(), String::new()))
        );

        theme
            .get_component_mut(ComponentId::Git)
            .unwrap()
            .options
            .insert(GIT_OPTION_AUTOHIDE_BRANCH.into(), false.into());
        let texts = demo_texts_for_components(&theme.components);
        assert_eq!(
            texts.get(&ComponentId::Git),
            Some(&("main".into(), "\u{2713}".into()))
        );

        theme
            .get_component_mut(ComponentId::Git)
            .unwrap()
            .options
            .insert(
                GIT_OPTION_AUTOHIDE_BRANCH.into(),
                serde_json::Value::Bool(true),
            );
        theme
            .get_component_mut(ComponentId::Worktree)
            .unwrap()
            .enabled = false;
        let texts = demo_texts_for_components(&theme.components);
        assert_eq!(
            texts.get(&ComponentId::Git),
            Some(&("main".into(), "\u{2713}".into()))
        );
    }

    #[test]
    fn plain_powerline_uses_solid_pointer_separator() {
        let mut theme = UserTheme::default_theme();
        let powerline = crate::presets::icon_sets::find("Powerline").unwrap();
        let scheme = crate::presets::color_schemes::find("Powerline Dark").unwrap();
        powerline.apply_to(&mut theme.components);
        scheme.apply_to(&mut theme.components);

        let mut texts = HashMap::new();
        texts.insert(ComponentId::Model, ("model".into(), String::new()));
        texts.insert(ComponentId::Directory, ("dir".into(), String::new()));

        let line = build_render_line(&theme.components, StyleMode::PlainPowerline, &texts);
        let sep = line
            .items
            .iter()
            .find_map(|item| match item {
                RenderItem::Sep(sep) => Some(sep),
                RenderItem::Seg(_) => None,
            })
            .unwrap();

        assert_eq!(sep.glyph, PLAIN_POWERLINE_SEPARATOR_GLYPH);
    }

    #[test]
    fn render_ansi_resets_after_plain_powerline_line() {
        let mut theme = UserTheme::default_theme();
        let powerline = crate::presets::icon_sets::find("Powerline").unwrap();
        let scheme = crate::presets::color_schemes::find("Powerline Dark").unwrap();
        powerline.apply_to(&mut theme.components);
        scheme.apply_to(&mut theme.components);

        let mut texts = HashMap::new();
        texts.insert(ComponentId::Model, ("model".into(), String::new()));

        let line = build_render_line(&theme.components, StyleMode::PlainPowerline, &texts);
        let ansi = render_ansi(&line);

        assert!(ansi.ends_with("\x1b[0m"));
    }
}
