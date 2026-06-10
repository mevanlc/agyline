use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentId {
    Model,
    Directory,
    Git,
    ContextWindow,
    Usage,
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
        ComponentId::Git,
        ComponentId::ContextWindow,
        ComponentId::Usage,
        ComponentId::Cost,
        ComponentId::Session,
        ComponentId::OutputStyle,
        ComponentId::Separator,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            ComponentId::Model => "Model",
            ComponentId::Directory => "Directory",
            ComponentId::Git => "Git",
            ComponentId::ContextWindow => "Context Window",
            ComponentId::Usage => "Usage",
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
            ComponentId::Git => "Branch and working tree status",
            ComponentId::ContextWindow => "Context window fill percentage",
            ComponentId::Usage => "API token count for this session",
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
            ComponentId::Git => "Git",
            ComponentId::ContextWindow => "Ctx Window",
            ComponentId::Usage => "Usage",
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerModelIcons {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub opus: String,
    #[serde(default)]
    pub sonnet: String,
    #[serde(default)]
    pub haiku: String,
    #[serde(default)]
    pub fable: String,
    #[serde(default)]
    pub mythos: String,
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
