use serde::{Deserialize, Serialize};

use crate::config::types::{
    AnsiColor, ColorConfig, ComponentConfig, ComponentId, DEFAULT_HOSTNAME_RSTRIP,
    DEFAULT_PR_OSC_HYPERLINKS, DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL, IconConfig,
    PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL, StyleConfig,
    StyleMode, TextStyleConfig,
};

/// A complete user theme — settings + colors + icons for all components.
/// Stored as a .toml file under ~/.claude/xline/themes/{Name}.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTheme {
    /// Whether this is the active theme.
    pub active: bool,

    /// Style configuration (mode: plain/nerd_font/powerline).
    pub style: StyleConfig,

    /// All component configurations, in display order.
    /// Separator should always be last.
    pub components: Vec<ComponentConfig>,
}

impl UserTheme {
    /// Create a default theme with sensible starting values.
    pub fn default_theme() -> Self {
        use ComponentId::*;

        let components = vec![
            ComponentConfig {
                id: Model,
                enabled: true,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f916}".into(), // 🤖
                    nerd_font: "\u{e26d}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 14 }),
                    text: Some(AnsiColor::Color16 { c16: 14 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: Directory,
                enabled: true,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f4c1}".into(), // 📁
                    nerd_font: "\u{f024b}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 12 }),
                    text: Some(AnsiColor::Color16 { c16: 12 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: Hostname,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f5a5}\u{fe0f}".into(), // 🖥️
                    nerd_font: "\u{f108}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 12 }),
                    text: Some(AnsiColor::Color16 { c16: 12 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: std::collections::HashMap::from([(
                    "rstrip".into(),
                    serde_json::Value::String(DEFAULT_HOSTNAME_RSTRIP.into()),
                )]),
            },
            ComponentConfig {
                id: Git,
                enabled: true,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f33f}".into(), // 🌿
                    nerd_font: "\u{f02a2}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 10 }),
                    text: Some(AnsiColor::Color16 { c16: 10 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: PullRequest,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f500}".into(), // 🔀
                    nerd_font: "\u{ea64}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 12 }),
                    text: Some(AnsiColor::Color16 { c16: 12 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: std::collections::HashMap::from([
                    (
                        PR_OPTION_SHOW_REVIEW_STATE.into(),
                        serde_json::Value::Bool(DEFAULT_PR_SHOW_REVIEW_STATE),
                    ),
                    (
                        PR_OPTION_SHOW_URL.into(),
                        serde_json::Value::Bool(DEFAULT_PR_SHOW_URL),
                    ),
                    (
                        PR_OPTION_OSC_HYPERLINKS.into(),
                        serde_json::Value::Bool(DEFAULT_PR_OSC_HYPERLINKS),
                    ),
                ]),
            },
            ComponentConfig {
                id: ContextWindow,
                enabled: true,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{26a1}".into(), // ⚡
                    nerd_font: "\u{f0e7}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 11 }),
                    text: Some(AnsiColor::Color16 { c16: 11 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: UsageFiveHour,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f4ca}".into(), // 📊
                    nerd_font: "\u{f080}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 13 }),
                    text: Some(AnsiColor::Color16 { c16: 13 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: UsageSevenDay,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f4ca}".into(), // 📊
                    nerd_font: "\u{f080}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 13 }),
                    text: Some(AnsiColor::Color16 { c16: 13 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: Cost,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f4b0}".into(), // 💰
                    nerd_font: "\u{f0155}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 11 }),
                    text: Some(AnsiColor::Color16 { c16: 11 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: Session,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{23f1}\u{fe0f}".into(), // ⏱️
                    nerd_font: "\u{f64f}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 8 }),
                    text: Some(AnsiColor::Color16 { c16: 8 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: OutputStyle,
                enabled: false,
                icon: IconConfig {
                    per_model: None,
                    plain: "\u{1f4dd}".into(), // 📝
                    nerd_font: "\u{f0f6}".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 7 }),
                    text: Some(AnsiColor::Color16 { c16: 7 }),
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
            ComponentConfig {
                id: Separator,
                enabled: true,
                icon: IconConfig {
                    per_model: None,
                    plain: " | ".into(),
                    nerd_font: " | ".into(),
                },
                colors: ColorConfig {
                    icon: Some(AnsiColor::Color16 { c16: 8 }),
                    text: None,
                    background: None,
                },
                styles: TextStyleConfig { text_bold: false },
                options: Default::default(),
            },
        ];

        Self {
            active: true,
            style: StyleConfig {
                mode: StyleMode::Plain,
            },
            components,
        }
    }

    /// Get a component config by id.
    pub fn get_component(&self, id: ComponentId) -> Option<&ComponentConfig> {
        self.components.iter().find(|c| c.id == id)
    }

    /// Get a mutable component config by id.
    pub fn get_component_mut(&mut self, id: ComponentId) -> Option<&mut ComponentConfig> {
        self.components.iter_mut().find(|c| c.id == id)
    }

    /// Add components introduced after this theme was saved, preserving the
    /// theme's existing component order and placing additions in default order.
    pub fn add_missing_components(&mut self) {
        let defaults = Self::default_theme().components;

        for (index, default) in defaults.iter().enumerate() {
            if self.get_component(default.id).is_some() {
                continue;
            }

            let insert_at = defaults[index + 1..]
                .iter()
                .find_map(|next| self.components.iter().position(|c| c.id == next.id))
                .unwrap_or(self.components.len());
            self.components.insert(insert_at, default.clone());
        }
    }

    /// Get the separator component.
    pub fn separator(&self) -> Option<&ComponentConfig> {
        self.get_component(ComponentId::Separator)
    }

    /// Get the separator glyph for the current style mode.
    pub fn separator_glyph(&self) -> &str {
        self.separator()
            .map(|s| match self.style.mode {
                StyleMode::Plain | StyleMode::PlainPowerline => s.icon.plain.as_str(),
                StyleMode::NerdFont | StyleMode::Powerline => s.icon.nerd_font.as_str(),
            })
            .unwrap_or(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme_has_all_components() {
        let theme = UserTheme::default_theme();
        for id in ComponentId::ALL {
            assert!(
                theme.get_component(*id).is_some(),
                "missing component: {:?}",
                id
            );
        }
    }

    #[test]
    fn test_separator_is_last() {
        let theme = UserTheme::default_theme();
        assert_eq!(theme.components.last().unwrap().id, ComponentId::Separator);
    }

    #[test]
    fn test_default_theme_is_active() {
        let theme = UserTheme::default_theme();
        assert!(theme.active);
    }

    #[test]
    fn test_rate_limit_components_are_independent_and_disabled_by_default() {
        let theme = UserTheme::default_theme();
        let five_hour = theme.get_component(ComponentId::UsageFiveHour).unwrap();
        let seven_day = theme.get_component(ComponentId::UsageSevenDay).unwrap();

        assert!(!five_hour.enabled);
        assert!(!seven_day.enabled);
        assert_ne!(five_hour.id, seven_day.id);
    }

    #[test]
    fn test_pull_request_defaults_are_compact_and_hyperlinked() {
        let theme = UserTheme::default_theme();
        let pull_request = theme.get_component(ComponentId::PullRequest).unwrap();

        assert!(!pull_request.enabled);
        assert_eq!(
            pull_request
                .options
                .get(PR_OPTION_SHOW_REVIEW_STATE)
                .and_then(|value| value.as_bool()),
            Some(true)
        );
        assert_eq!(
            pull_request
                .options
                .get(PR_OPTION_SHOW_URL)
                .and_then(|value| value.as_bool()),
            Some(false)
        );
        assert_eq!(
            pull_request
                .options
                .get(PR_OPTION_OSC_HYPERLINKS)
                .and_then(|value| value.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn test_add_missing_components_uses_default_order() {
        let mut theme = UserTheme::default_theme();
        theme
            .components
            .retain(|c| c.id != ComponentId::Hostname && c.id != ComponentId::PullRequest);

        theme.add_missing_components();

        let directory = theme
            .components
            .iter()
            .position(|c| c.id == ComponentId::Directory)
            .unwrap();
        assert_eq!(theme.components[directory + 1].id, ComponentId::Hostname);
        assert!(!theme.components[directory + 1].enabled);
        assert_eq!(
            theme.components[directory + 1]
                .options
                .get("rstrip")
                .and_then(|value| value.as_str()),
            Some(DEFAULT_HOSTNAME_RSTRIP)
        );

        let git = theme
            .components
            .iter()
            .position(|c| c.id == ComponentId::Git)
            .unwrap();
        let pull_request = &theme.components[git + 1];
        assert_eq!(pull_request.id, ComponentId::PullRequest);
        assert!(!pull_request.enabled);
        assert_eq!(
            pull_request
                .options
                .get(PR_OPTION_OSC_HYPERLINKS)
                .and_then(|value| value.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn test_roundtrip_toml() {
        let theme = UserTheme::default_theme();
        let toml_str = toml::to_string_pretty(&theme).unwrap();
        let parsed: UserTheme = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.active, theme.active);
        assert_eq!(parsed.style.mode, theme.style.mode);
        assert_eq!(parsed.components.len(), theme.components.len());
        for (a, b) in parsed.components.iter().zip(theme.components.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.enabled, b.enabled);
            assert_eq!(a.icon.plain, b.icon.plain);
            assert_eq!(a.icon.nerd_font, b.icon.nerd_font);
            assert_eq!(a.colors.icon, b.colors.icon);
            assert_eq!(a.colors.text, b.colors.text);
            assert_eq!(a.colors.background, b.colors.background);
            assert_eq!(a.styles.text_bold, b.styles.text_bold);
            assert_eq!(a.options, b.options);
        }
    }
}
