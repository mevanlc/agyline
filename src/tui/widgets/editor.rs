use crate::config::theme::ResolvedTheme;
use crate::config::types::{
    AnsiColor, ComponentId, DEFAULT_GIT_AUTOHIDE_BRANCH, DEFAULT_HOSTNAME_RSTRIP,
    DEFAULT_PR_OSC_HYPERLINKS, DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL,
    DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH, GIT_OPTION_AUTOHIDE_BRANCH, MODEL_OPTION_REPLACE,
    MODEL_OPTION_SEARCH, PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL,
    UsageValue, WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH, WorktreeOutside,
};
use crate::core::render;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldSelection {
    Enabled,
    GlyphMode,
    Powerline,
    PowerlinePlain,
    PowerlineNerd,
    PlainIcon,
    NerdFontIcon,
    HostnameRstrip,
    WorktreeOutside,
    WorktreeOriginalBranch,
    GitAutohideBranch,
    GitShowSha,
    PrReviewState,
    PrUrl,
    PrOscHyperlinks,
    UsageValue,
    PerModelIcons,
    EffortLevel,
    ThinkingIcon,
    ModelSearch,
    ModelReplace,
    FlashIcon,
    ProIcon,
    UltraIcon,
    FlashLiteIcon,
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
        let mut fields = vec![Self::Enabled];

        if comp.id == ComponentId::Model {
            let pm_enabled = comp.icon.per_model.as_ref().is_some_and(|pm| pm.enabled);
            if pm_enabled {
                fields.push(Self::PerModelIcons);
                fields.push(Self::EffortLevel);
                fields.push(Self::ThinkingIcon);
                fields.push(Self::ModelSearch);
                fields.push(Self::ModelReplace);
                fields.push(Self::FlashIcon);
                fields.push(Self::ProIcon);
                fields.push(Self::UltraIcon);
                fields.push(Self::FlashLiteIcon);
                fields.push(Self::OpusIcon);
                fields.push(Self::SonnetIcon);
                fields.push(Self::HaikuIcon);
                fields.push(Self::FableIcon);
                fields.push(Self::MythosIcon);
            } else {
                fields.push(Self::PlainIcon);
                fields.push(Self::NerdFontIcon);
                fields.push(Self::PerModelIcons);
                fields.push(Self::EffortLevel);
                fields.push(Self::ThinkingIcon);
                fields.push(Self::ModelSearch);
                fields.push(Self::ModelReplace);
            }
        } else {
            fields.push(Self::PlainIcon);
            fields.push(Self::NerdFontIcon);
            if comp.id == ComponentId::Hostname {
                fields.push(Self::HostnameRstrip);
            }
            if comp.id == ComponentId::Worktree {
                fields.push(Self::WorktreeOutside);
                fields.push(Self::WorktreeOriginalBranch);
            }
            if comp.id == ComponentId::Git {
                fields.push(Self::GitAutohideBranch);
                fields.push(Self::GitShowSha);
            }
            if comp.id == ComponentId::PullRequest {
                fields.extend([Self::PrReviewState, Self::PrUrl, Self::PrOscHyperlinks]);
            }
            if matches!(
                comp.id,
                ComponentId::UsageFiveHour | ComponentId::UsageSevenDay
            ) {
                fields.push(Self::UsageValue);
            }
        }

        fields.extend([
            Self::IconColor,
            Self::TextColor,
            Self::BackgroundColor,
            Self::Bold,
        ]);
        fields
    }

    pub fn fields_for_kind(
        comp: &crate::config::types::ComponentConfig,
        kind: crate::config::catalog::Kind,
    ) -> Vec<Self> {
        use crate::config::catalog::Kind;
        if comp.id == ComponentId::Separator {
            return match kind {
                Kind::Icons => vec![
                    Self::GlyphMode,
                    Self::Powerline,
                    Self::Enabled,
                    Self::PlainIcon,
                    Self::NerdFontIcon,
                    Self::PowerlinePlain,
                    Self::PowerlineNerd,
                ],
                Kind::Colors => vec![Self::IconColor],
                _ => vec![],
            };
        }
        Self::fields_for(comp)
            .into_iter()
            .filter(|field| match kind {
                Kind::Components => matches!(
                    field,
                    Self::Enabled
                        | Self::HostnameRstrip
                        | Self::WorktreeOutside
                        | Self::WorktreeOriginalBranch
                        | Self::GitAutohideBranch
                        | Self::GitShowSha
                        | Self::PrReviewState
                        | Self::PrUrl
                        | Self::PrOscHyperlinks
                        | Self::UsageValue
                        | Self::EffortLevel
                        | Self::ModelSearch
                        | Self::ModelReplace
                ),
                Kind::Colors => matches!(
                    field,
                    Self::IconColor | Self::TextColor | Self::BackgroundColor | Self::Bold
                ),
                Kind::Icons => !matches!(
                    field,
                    Self::Enabled
                        | Self::HostnameRstrip
                        | Self::WorktreeOutside
                        | Self::WorktreeOriginalBranch
                        | Self::GitAutohideBranch
                        | Self::GitShowSha
                        | Self::PrReviewState
                        | Self::PrUrl
                        | Self::PrOscHyperlinks
                        | Self::UsageValue
                        | Self::EffortLevel
                        | Self::ModelSearch
                        | Self::ModelReplace
                        | Self::IconColor
                        | Self::TextColor
                        | Self::BackgroundColor
                        | Self::Bold
                ),
                Kind::Theme => false,
            })
            .collect()
    }
}

pub struct EditorWidget;

impl EditorWidget {
    pub fn render(
        f: &mut Frame,
        area: Rect,
        theme: &ResolvedTheme,
        selected_component: usize,
        is_focused: bool,
        selected_field: FieldSelection,
        kind: crate::config::catalog::Kind,
    ) {
        let comp = match theme.components.get(selected_component) {
            Some(c) => c,
            None => return,
        };

        let visible_fields = FieldSelection::fields_for_kind(comp, kind);

        let pm = comp.icon.per_model.as_ref();
        let field_data: Vec<(&str, String, Option<Color>)> = visible_fields
            .iter()
            .map(|f| match f {
                FieldSelection::Enabled => (
                    "Enabled",
                    if comp.enabled { "Yes" } else { "No" }.into(),
                    None,
                ),
                FieldSelection::GitShowSha => (
                    "Show SHA",
                    if comp
                        .options
                        .get("show_sha")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::GlyphMode => (
                    "Glyph Mode",
                    if matches!(
                        theme.style.mode,
                        crate::config::types::StyleMode::Plain
                            | crate::config::types::StyleMode::PlainPowerline
                    ) {
                        "Plain"
                    } else {
                        "Nerd Font"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::Powerline => (
                    "Powerline",
                    if matches!(
                        theme.style.mode,
                        crate::config::types::StyleMode::Powerline
                            | crate::config::types::StyleMode::PlainPowerline
                    ) {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::PowerlinePlain => {
                    ("Connector Plain", theme.powerline_plain.clone(), None)
                }
                FieldSelection::PowerlineNerd => {
                    ("Connector Nerd", theme.powerline_nerd_font.clone(), None)
                }
                FieldSelection::PlainIcon => ("Plain Icon", comp.icon.plain.clone(), None),
                FieldSelection::NerdFontIcon => ("Nerd Icon", comp.icon.nerd_font.clone(), None),
                FieldSelection::HostnameRstrip => (
                    "RStrip",
                    comp.options
                        .get("rstrip")
                        .and_then(|value| value.as_str())
                        .unwrap_or(DEFAULT_HOSTNAME_RSTRIP)
                        .into(),
                    None,
                ),
                FieldSelection::WorktreeOutside => (
                    "Outside Worktrees",
                    WorktreeOutside::from_options(&comp.options)
                        .display_name()
                        .into(),
                    None,
                ),
                FieldSelection::WorktreeOriginalBranch => (
                    "Original Branch",
                    if comp
                        .options
                        .get(WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH)
                        .and_then(|value| value.as_bool())
                        .unwrap_or(DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::GitAutohideBranch => (
                    "Autohide Branch",
                    if comp
                        .options
                        .get(GIT_OPTION_AUTOHIDE_BRANCH)
                        .and_then(|value| value.as_bool())
                        .unwrap_or(DEFAULT_GIT_AUTOHIDE_BRANCH)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::PrReviewState => (
                    "Review State",
                    if comp
                        .options
                        .get(PR_OPTION_SHOW_REVIEW_STATE)
                        .and_then(|value| value.as_bool())
                        .unwrap_or(DEFAULT_PR_SHOW_REVIEW_STATE)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::PrUrl => (
                    "URL",
                    if comp
                        .options
                        .get(PR_OPTION_SHOW_URL)
                        .and_then(|value| value.as_bool())
                        .unwrap_or(DEFAULT_PR_SHOW_URL)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::PrOscHyperlinks => (
                    "OSC Hyperlinks",
                    if comp
                        .options
                        .get(PR_OPTION_OSC_HYPERLINKS)
                        .and_then(|value| value.as_bool())
                        .unwrap_or(DEFAULT_PR_OSC_HYPERLINKS)
                    {
                        "Yes"
                    } else {
                        "No"
                    }
                    .into(),
                    None,
                ),
                FieldSelection::UsageValue => (
                    "Value",
                    UsageValue::from_options(&comp.options)
                        .display_name()
                        .into(),
                    None,
                ),
                FieldSelection::PerModelIcons => {
                    let enabled = pm.is_some_and(|p| p.enabled);
                    ("Per-Model", if enabled { "Yes" } else { "No" }.into(), None)
                }
                FieldSelection::EffortLevel => {
                    let effort = crate::config::types::ModelEffort::from_options(&comp.options);
                    ("Effort", effort.display_name().into(), None)
                }
                FieldSelection::ThinkingIcon => (
                    "Thinking Icon",
                    comp.options
                        .get("thinking_icon")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .into(),
                    None,
                ),
                FieldSelection::ModelSearch => (
                    "Search",
                    comp.options
                        .get(MODEL_OPTION_SEARCH)
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .into(),
                    None,
                ),
                FieldSelection::ModelReplace => (
                    "Replace",
                    comp.options
                        .get(MODEL_OPTION_REPLACE)
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .into(),
                    None,
                ),
                FieldSelection::FlashIcon => (
                    "Flash Icon",
                    pm.map_or(String::new(), |p| {
                        p.flash.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::ProIcon => (
                    "Pro Icon",
                    pm.map_or(String::new(), |p| {
                        p.pro.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::UltraIcon => (
                    "Ultra Icon",
                    pm.map_or(String::new(), |p| {
                        p.ultra.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::FlashLiteIcon => (
                    "Flash Lite Icon",
                    pm.map_or(String::new(), |p| {
                        p.flash_lite.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::OpusIcon => (
                    "Opus Icon",
                    pm.map_or(String::new(), |p| {
                        p.opus.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::FableIcon => (
                    "Fable Icon",
                    pm.map_or(String::new(), |p| {
                        p.fable.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::MythosIcon => (
                    "Mythos Icon",
                    pm.map_or(String::new(), |p| {
                        p.mythos.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::SonnetIcon => (
                    "Sonnet Icon",
                    pm.map_or(String::new(), |p| {
                        p.sonnet.for_mode(theme.style.mode).to_string()
                    }),
                    None,
                ),
                FieldSelection::HaikuIcon => (
                    "Haiku Icon",
                    pm.map_or(String::new(), |p| {
                        p.haiku.for_mode(theme.style.mode).to_string()
                    }),
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
            .map(|(l, _, _)| Text::raw(*l).width())
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

                let label_text = format!("{}:", label);
                let pad = label_col_width.saturating_sub(Text::raw(&label_text).width());
                let padded_label = format!("{}{} ", label_text, " ".repeat(pad));

                let mut spans = vec![
                    Span::styled(cursor, style),
                    Span::styled(padded_label, style),
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

        let offset = selected_idx
            .saturating_sub(inner_height / 2)
            .min(total.saturating_sub(inner_height));
        let items: Vec<_> = all_items
            .into_iter()
            .skip(offset)
            .take(inner_height)
            .collect();

        let border_style = if is_focused {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let title = Line::from(vec![
            Span::styled(
                format!(
                    " {} ",
                    if comp.id == ComponentId::Unknown {
                        "Defaults"
                    } else if comp.id == ComponentId::Separator
                        && kind == crate::config::catalog::Kind::Icons
                    {
                        "Mode / Separators"
                    } else {
                        comp.display_name()
                    }
                ),
                border_style,
            ),
            Span::styled(
                format!(
                    "- {} ",
                    if comp.id == ComponentId::Unknown {
                        "Fallback values"
                    } else if comp.id == ComponentId::Separator {
                        "Between visible components"
                    } else {
                        comp.id.description()
                    }
                ),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_style)
            .title(title)
            .title_bottom(format!(" {}/{} ↑↓ ", selected_idx + 1, total));

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

#[cfg(test)]
mod tests {
    use super::FieldSelection;
    use crate::config::theme::ResolvedTheme;
    use crate::config::types::ComponentId;

    #[test]
    fn rstrip_is_only_available_for_hostname() {
        let theme = ResolvedTheme::default_theme();
        let hostname = theme.get_component(ComponentId::Hostname).unwrap();
        let directory = theme.get_component(ComponentId::Directory).unwrap();

        assert!(FieldSelection::fields_for(hostname).contains(&FieldSelection::HostnameRstrip));
        assert!(!FieldSelection::fields_for(directory).contains(&FieldSelection::HostnameRstrip));
    }

    #[test]
    fn pull_request_fields_include_all_visibility_toggles() {
        let theme = ResolvedTheme::default_theme();
        let pull_request = theme.get_component(ComponentId::PullRequest).unwrap();
        let fields = FieldSelection::fields_for(pull_request);

        assert!(fields.contains(&FieldSelection::PrReviewState));
        assert!(fields.contains(&FieldSelection::PrUrl));
        assert!(fields.contains(&FieldSelection::PrOscHyperlinks));
    }

    #[test]
    fn original_branch_is_only_available_for_worktree() {
        let theme = ResolvedTheme::default_theme();
        let worktree = theme.get_component(ComponentId::Worktree).unwrap();
        let git = theme.get_component(ComponentId::Git).unwrap();

        assert!(FieldSelection::fields_for(worktree).contains(&FieldSelection::WorktreeOutside));
        assert!(
            FieldSelection::fields_for(worktree).contains(&FieldSelection::WorktreeOriginalBranch)
        );
        assert!(!FieldSelection::fields_for(git).contains(&FieldSelection::WorktreeOutside));
        assert!(!FieldSelection::fields_for(git).contains(&FieldSelection::WorktreeOriginalBranch));
    }

    #[test]
    fn autohide_branch_is_only_available_for_git_status() {
        let theme = ResolvedTheme::default_theme();
        let git = theme.get_component(ComponentId::Git).unwrap();
        let worktree = theme.get_component(ComponentId::Worktree).unwrap();

        assert!(FieldSelection::fields_for(git).contains(&FieldSelection::GitAutohideBranch));
        assert!(!FieldSelection::fields_for(worktree).contains(&FieldSelection::GitAutohideBranch));
    }

    #[test]
    fn value_is_only_available_for_the_usage_components() {
        let theme = ResolvedTheme::default_theme();
        let five_hour = theme.get_component(ComponentId::UsageFiveHour).unwrap();
        let seven_day = theme.get_component(ComponentId::UsageSevenDay).unwrap();
        let context = theme.get_component(ComponentId::ContextWindow).unwrap();

        assert!(FieldSelection::fields_for(five_hour).contains(&FieldSelection::UsageValue));
        assert!(FieldSelection::fields_for(seven_day).contains(&FieldSelection::UsageValue));
        assert!(!FieldSelection::fields_for(context).contains(&FieldSelection::UsageValue));
    }

    #[test]
    fn model_search_and_replace_are_only_available_for_model() {
        let theme = ResolvedTheme::default_theme();
        let model = theme.get_component(ComponentId::Model).unwrap();
        let directory = theme.get_component(ComponentId::Directory).unwrap();

        assert!(FieldSelection::fields_for(model).contains(&FieldSelection::ModelSearch));
        assert!(FieldSelection::fields_for(model).contains(&FieldSelection::ModelReplace));
        assert!(!FieldSelection::fields_for(directory).contains(&FieldSelection::ModelSearch));
        assert!(!FieldSelection::fields_for(directory).contains(&FieldSelection::ModelReplace));
    }
}
