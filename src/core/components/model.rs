use super::{Component, ComponentData};
use crate::config::types::{ComponentId, PerModelIcons};
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelComponent {
    per_model: Option<PerModelIcons>,
    show_effort: bool,
    thinking_icon: String,
}

impl ModelComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_per_model(mut self, per_model: Option<PerModelIcons>) -> Self {
        self.per_model = per_model;
        self
    }

    pub fn with_effort(mut self, show_effort: bool) -> Self {
        self.show_effort = show_effort;
        self
    }

    pub fn with_thinking_icon(mut self, thinking_icon: String) -> Self {
        self.thinking_icon = thinking_icon;
        self
    }

    fn format_display_name(display_name: &str) -> String {
        let mut out = String::with_capacity(display_name.len());
        let mut rest = display_name;

        while let Some(start) = rest.find('(') {
            out.push_str(&rest[..start]);
            let after_open = &rest[start + 1..];

            if let Some(end) = after_open.find(" context)") {
                let token = &after_open[..end];
                if !token.is_empty() && !token.chars().any(char::is_whitespace) {
                    out.push_str(token);
                    rest = &after_open[end + " context)".len()..];
                    continue;
                }
            }

            out.push('(');
            rest = after_open;
        }

        out.push_str(rest);
        out
    }

    fn effort_code(level: &str) -> Option<&'static str> {
        match level {
            "low" => Some("l"),
            "medium" => Some("m"),
            "high" => Some("h"),
            "xhigh" => Some("H"),
            "max" => Some("X"),
            _ => None,
        }
    }
}

impl Component for ModelComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let display_name = Self::format_display_name(&input.model.display_name);
        let effort_code = if self.show_effort {
            input
                .effort
                .as_ref()
                .and_then(|effort| effort.level.as_deref())
                .and_then(Self::effort_code)
                .unwrap_or_default()
                .to_string()
        } else {
            String::new()
        };
        let thinking_icon = if input
            .thinking
            .as_ref()
            .and_then(|thinking| thinking.enabled)
            .unwrap_or(false)
        {
            self.thinking_icon.as_str()
        } else {
            ""
        };
        let secondary = format!("{}{}", effort_code, thinking_icon);
        let mut metadata = HashMap::new();
        metadata.insert("model_id".into(), input.model.id.clone());
        metadata.insert("display_name".into(), display_name.clone());

        if let Some(pm) = &self.per_model
            && pm.enabled
        {
            let model_id = input.model.id.to_ascii_lowercase();
            let tier = if model_id.contains("mythos") {
                &pm.mythos
            } else if model_id.contains("fable") {
                &pm.fable
            } else if model_id.contains("opus") {
                &pm.opus
            } else if model_id.contains("haiku") {
                &pm.haiku
            } else {
                &pm.sonnet
            };
            if !tier.plain.is_empty() {
                metadata.insert("dynamic_icon_plain".into(), tier.plain.clone());
            }
            if !tier.nerd_font.is_empty() {
                metadata.insert("dynamic_icon_nerd_font".into(), tier.nerd_font.clone());
            }
        }

        Some(ComponentData {
            primary: display_name,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Model
    }
}

#[cfg(test)]
mod tests {
    use super::ModelComponent;

    #[test]
    fn rewrites_context_suffix_to_bare_token() {
        assert_eq!(
            ModelComponent::format_display_name("Claude Sonnet (200k context)"),
            "Claude Sonnet 200k"
        );
        assert_eq!(
            ModelComponent::format_display_name("Claude Opus (1M context)"),
            "Claude Opus 1M"
        );
    }

    #[test]
    fn leaves_other_parenthesized_text_unchanged() {
        assert_eq!(
            ModelComponent::format_display_name("Claude Sonnet (beta)"),
            "Claude Sonnet (beta)"
        );
        assert_eq!(
            ModelComponent::format_display_name("Claude Sonnet (200 k context)"),
            "Claude Sonnet (200 k context)"
        );
    }
}
