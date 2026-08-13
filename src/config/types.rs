use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub const DEFAULT_HOSTNAME_RSTRIP: &str = ".local,.localhost,.lan";
pub const GIT_OPTION_AUTOHIDE_BRANCH: &str = "autohide_branch";
pub const DEFAULT_GIT_AUTOHIDE_BRANCH: bool = true;
pub const WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH: &str = "show_original_branch";
pub const DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH: bool = false;
pub const WORKTREE_OPTION_OUTSIDE_WORKTREES: &str = "outside_worktrees";
pub const PR_OPTION_SHOW_REVIEW_STATE: &str = "show_review_state";
pub const PR_OPTION_SHOW_URL: &str = "show_url";
pub const PR_OPTION_OSC_HYPERLINKS: &str = "osc_hyperlinks";
pub const DEFAULT_PR_SHOW_REVIEW_STATE: bool = true;
pub const DEFAULT_PR_SHOW_URL: bool = false;
pub const DEFAULT_PR_OSC_HYPERLINKS: bool = true;
pub const USAGE_OPTION_VALUE: &str = "value";
pub const MODEL_OPTION_SHOW_EFFORT: &str = "show_effort";
pub const DEFAULT_MODEL_SHOW_EFFORT: bool = true;

/// What the Worktree component displays when Claude is not in a worktree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorktreeOutside {
    Hide,
    Show,
    #[default]
    Branch,
    Directory,
}

impl WorktreeOutside {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Hide => "Hide",
            Self::Show => "Show",
            Self::Branch => "Branch",
            Self::Directory => "Directory",
        }
    }

    pub fn component_name(self) -> &'static str {
        match self {
            Self::Hide => "Worktree",
            Self::Show => "Worktree (Always Show)",
            Self::Branch => "Worktree (or Branch)",
            Self::Directory => "Worktree (or Directory)",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hide => "hide",
            Self::Show => "show",
            Self::Branch => "branch",
            Self::Directory => "directory",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::Hide => Self::Show,
            Self::Show => Self::Branch,
            Self::Branch => Self::Directory,
            Self::Directory => Self::Hide,
        }
    }

    pub fn from_options(options: &HashMap<String, serde_json::Value>) -> Self {
        options
            .get(WORKTREE_OPTION_OUTSIDE_WORKTREES)
            .and_then(|value| value.as_str())
            .and_then(|value| match value {
                "hide" => Some(Self::Hide),
                "show" => Some(Self::Show),
                "branch" => Some(Self::Branch),
                "directory" => Some(Self::Directory),
                _ => None,
            })
            .unwrap_or_default()
    }
}

impl fmt::Display for WorktreeOutside {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

/// Which side of a rate-limit window the usage components display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageValue {
    #[default]
    Used,
    Remaining,
}

impl UsageValue {
    pub fn display_name(self) -> &'static str {
        match self {
            UsageValue::Used => "Used",
            UsageValue::Remaining => "Remaining",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            UsageValue::Used => "used",
            UsageValue::Remaining => "remaining",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            UsageValue::Used => UsageValue::Remaining,
            UsageValue::Remaining => UsageValue::Used,
        }
    }

    /// Read the setting out of a component's options, falling back to the default.
    pub fn from_options(options: &HashMap<String, serde_json::Value>) -> Self {
        options
            .get(USAGE_OPTION_VALUE)
            .and_then(|value| value.as_str())
            .and_then(|value| match value {
                "used" => Some(UsageValue::Used),
                "remaining" => Some(UsageValue::Remaining),
                _ => None,
            })
            .unwrap_or_default()
    }

    /// Apply the setting to a rate-limit percentage reported as "used".
    pub fn apply(self, used_percentage: f64) -> f64 {
        match self {
            UsageValue::Used => used_percentage,
            UsageValue::Remaining => (100.0 - used_percentage).clamp(0.0, 100.0),
        }
    }
}

impl fmt::Display for UsageValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentId {
    Model,
    Directory,
    Worktree,
    Hostname,
    Git,
    PullRequest,
    ContextWindow,
    #[serde(rename = "usage_5h")]
    UsageFiveHour,
    #[serde(rename = "usage_7d")]
    UsageSevenDay,
    Cost,
    Session,
    OutputStyle,
    Separator,
}

impl ComponentId {
    /// All component IDs in default order (separator last).
    pub const ALL: &[ComponentId] = &[
        ComponentId::Model,
        ComponentId::Directory,
        ComponentId::Worktree,
        ComponentId::Hostname,
        ComponentId::Git,
        ComponentId::PullRequest,
        ComponentId::ContextWindow,
        ComponentId::UsageFiveHour,
        ComponentId::UsageSevenDay,
        ComponentId::Cost,
        ComponentId::Session,
        ComponentId::OutputStyle,
        ComponentId::Separator,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            ComponentId::Model => "Model",
            ComponentId::Directory => "Directory",
            ComponentId::Worktree => "Worktree",
            ComponentId::Hostname => "Hostname",
            ComponentId::Git => "Git Status",
            ComponentId::PullRequest => "Pull Request",
            ComponentId::ContextWindow => "Context Window",
            ComponentId::UsageFiveHour => "Usage (5h)",
            ComponentId::UsageSevenDay => "Usage (7d)",
            ComponentId::Cost => "Cost",
            ComponentId::Session => "Session",
            ComponentId::OutputStyle => "Output Style",
            ComponentId::Separator => "Separator",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            ComponentId::Model => "Active Claude model name",
            ComponentId::Directory => "Current working directory",
            ComponentId::Worktree => "Active linked worktree",
            ComponentId::Hostname => "Current machine hostname",
            ComponentId::Git => "Branch, file changes, upstream status, and optional SHA",
            ComponentId::PullRequest => "Open pull request and review state",
            ComponentId::ContextWindow => "Context window fill percentage",
            ComponentId::UsageFiveHour => "Five-hour rate-limit usage",
            ComponentId::UsageSevenDay => "Seven-day rate-limit usage",
            ComponentId::Cost => "Estimated API cost for this session",
            ComponentId::Session => "Elapsed time in current session",
            ComponentId::OutputStyle => "Response verbosity mode",
            ComponentId::Separator => "Divider between left and right sides",
        }
    }

    pub fn short_name(self) -> &'static str {
        match self {
            ComponentId::Model => "Model",
            ComponentId::Directory => "Directory",
            ComponentId::Worktree => "Worktree",
            ComponentId::Hostname => "Hostname",
            ComponentId::Git => "Git",
            ComponentId::PullRequest => "PR",
            ComponentId::ContextWindow => "Ctx Window",
            ComponentId::UsageFiveHour => "Usage (5h)",
            ComponentId::UsageSevenDay => "Usage (7d)",
            ComponentId::Cost => "Cost",
            ComponentId::Session => "Session",
            ComponentId::OutputStyle => "Output",
            ComponentId::Separator => "Separator",
        }
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleMode {
    Plain,
    NerdFont,
    Powerline,
    PlainPowerline,
}

impl StyleMode {
    pub fn display_name(self) -> &'static str {
        match self {
            StyleMode::Plain => "Plain",
            StyleMode::NerdFont => "Nerd Font",
            StyleMode::Powerline => "Powerline",
            StyleMode::PlainPowerline => "Plain Powerline",
        }
    }
}

impl fmt::Display for StyleMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleConfig {
    pub mode: StyleMode,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            mode: StyleMode::Plain,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelTierIcons {
    pub plain: String,
    pub nerd_font: String,
}

impl ModelTierIcons {
    pub fn new(plain: impl Into<String>, nerd_font: impl Into<String>) -> Self {
        Self {
            plain: plain.into(),
            nerd_font: nerd_font.into(),
        }
    }

    pub fn for_mode(&self, mode: StyleMode) -> &str {
        match mode {
            StyleMode::Plain | StyleMode::PlainPowerline => &self.plain,
            StyleMode::NerdFont | StyleMode::Powerline => &self.nerd_font,
        }
    }

    pub fn for_mode_mut(&mut self, mode: StyleMode) -> &mut String {
        match mode {
            StyleMode::Plain | StyleMode::PlainPowerline => &mut self.plain,
            StyleMode::NerdFont | StyleMode::Powerline => &mut self.nerd_font,
        }
    }
}

impl Serialize for ModelTierIcons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.plain == self.nerd_font {
            serializer.serialize_str(&self.plain)
        } else {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("ModelTierIcons", 2)?;
            state.serialize_field("plain", &self.plain)?;
            state.serialize_field("nerd_font", &self.nerd_font)?;
            state.end()
        }
    }
}

impl<'de> Deserialize<'de> for ModelTierIcons {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Single(String),
            Detailed { plain: String, nerd_font: String },
        }

        match Helper::deserialize(deserializer)? {
            Helper::Single(s) => Ok(ModelTierIcons {
                plain: s.clone(),
                nerd_font: s,
            }),
            Helper::Detailed { plain, nerd_font } => Ok(ModelTierIcons { plain, nerd_font }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PerModelIcons {
    pub enabled: bool,
    pub opus: ModelTierIcons,
    pub sonnet: ModelTierIcons,
    pub haiku: ModelTierIcons,
    pub fable: ModelTierIcons,
    pub mythos: ModelTierIcons,
}

impl Default for PerModelIcons {
    fn default() -> Self {
        Self {
            enabled: true,
            opus: ModelTierIcons::new("\u{1f989}", "\u{f03d2}"), // 🦉 / 󰏒 md-owl
            sonnet: ModelTierIcons::new("\u{1f3ad}", "\u{eeb6}"), // 🎭 /  fa-masks_theater
            haiku: ModelTierIcons::new("\u{1f338}", "\u{f024a}"), // 🌸 / 󰉊 md-flower
            fable: ModelTierIcons::new("\u{1f52e}", "\u{f0b2f}"), // 🔮 / 󰬯 md-crystal_ball
            mythos: ModelTierIcons::new("\u{1f451}", "\u{edeb}"), // 👑 /  fa-crown
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IconConfig {
    pub plain: String,
    pub nerd_font: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_model: Option<PerModelIcons>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnsiColor {
    Color16 { c16: u8 },
    Color256 { c256: u8 },
    Rgb { r: u8, g: u8, b: u8 },
}

impl PartialEq for AnsiColor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnsiColor::Color16 { c16: a }, AnsiColor::Color16 { c16: b }) => a == b,
            (AnsiColor::Color256 { c256: a }, AnsiColor::Color256 { c256: b }) => a == b,
            (
                AnsiColor::Rgb {
                    r: r1,
                    g: g1,
                    b: b1,
                },
                AnsiColor::Rgb {
                    r: r2,
                    g: g2,
                    b: b2,
                },
            ) => r1 == r2 && g1 == g2 && b1 == b2,
            _ => false,
        }
    }
}

impl Eq for AnsiColor {}

impl fmt::Display for AnsiColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnsiColor::Color16 { c16 } => write!(f, "c16({})", c16),
            AnsiColor::Color256 { c256 } => write!(f, "c256({})", c256),
            AnsiColor::Rgb { r, g, b } => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<AnsiColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<AnsiColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<AnsiColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyleConfig {
    pub text_bold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub id: ComponentId,
    pub enabled: bool,
    pub icon: IconConfig,
    pub colors: ColorConfig,
    pub styles: TextStyleConfig,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub options: HashMap<String, serde_json::Value>,
}

impl ComponentConfig {
    pub fn display_name(&self) -> &'static str {
        if self.id == ComponentId::Worktree {
            WorktreeOutside::from_options(&self.options).component_name()
        } else {
            self.id.display_name()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_labels_and_config_ids_are_stable() {
        assert_eq!(ComponentId::UsageFiveHour.display_name(), "Usage (5h)");
        assert_eq!(ComponentId::UsageSevenDay.display_name(), "Usage (7d)");
        assert_eq!(
            serde_json::to_string(&ComponentId::UsageFiveHour).unwrap(),
            "\"usage_5h\""
        );
        assert_eq!(
            serde_json::to_string(&ComponentId::UsageSevenDay).unwrap(),
            "\"usage_7d\""
        );
        assert_eq!(ComponentId::PullRequest.display_name(), "Pull Request");
        assert_eq!(ComponentId::Git.display_name(), "Git Status");
        assert_eq!(
            ComponentId::Git.description(),
            "Branch, file changes, upstream status, and optional SHA"
        );
        assert_eq!(
            serde_json::to_string(&ComponentId::PullRequest).unwrap(),
            "\"pull_request\""
        );
        assert_eq!(
            serde_json::to_string(&ComponentId::Worktree).unwrap(),
            "\"worktree\""
        );
    }

    #[test]
    fn worktree_outside_modes_cycle_in_editor_order() {
        let mut mode = WorktreeOutside::Hide;
        for expected in [
            WorktreeOutside::Show,
            WorktreeOutside::Branch,
            WorktreeOutside::Directory,
            WorktreeOutside::Hide,
        ] {
            mode = mode.toggled();
            assert_eq!(mode, expected);
        }
        assert_eq!(WorktreeOutside::default(), WorktreeOutside::Branch);
        assert_eq!(WorktreeOutside::Hide.component_name(), "Worktree");
        assert_eq!(
            WorktreeOutside::Show.component_name(),
            "Worktree (Always Show)"
        );
        assert_eq!(
            WorktreeOutside::Branch.component_name(),
            "Worktree (or Branch)"
        );
        assert_eq!(
            WorktreeOutside::Directory.component_name(),
            "Worktree (or Directory)"
        );
    }

    #[test]
    fn model_tier_icons_deserializes_single_and_detailed_forms() {
        let single_toml = r#"
            opus = "🐙"
            sonnet = { plain = "🎶", nerd_font = "󰏒" }
        "#;

        #[derive(Deserialize)]
        struct TestConfig {
            opus: ModelTierIcons,
            sonnet: ModelTierIcons,
        }

        let cfg: TestConfig = toml::from_str(single_toml).unwrap();
        assert_eq!(cfg.opus.plain, "🐙");
        assert_eq!(cfg.opus.nerd_font, "🐙");
        assert_eq!(cfg.sonnet.plain, "🎶");
        assert_eq!(cfg.sonnet.nerd_font, "󰏒");

        assert_eq!(cfg.opus.for_mode(StyleMode::Plain), "🐙");
        assert_eq!(cfg.opus.for_mode(StyleMode::NerdFont), "🐙");
        assert_eq!(cfg.sonnet.for_mode(StyleMode::Plain), "🎶");
        assert_eq!(cfg.sonnet.for_mode(StyleMode::NerdFont), "󰏒");
    }
}
