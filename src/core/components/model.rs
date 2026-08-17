use super::{Component, ComponentData};
use crate::config::types::{ComponentId, ModelEffort, PerModelIcons};
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelComponent {
    per_model: Option<PerModelIcons>,
    effort: ModelEffort,
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

    pub fn with_effort(mut self, effort: ModelEffort) -> Self {
        self.effort = effort;
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
        let show_effort = self.effort.should_display(input.model.is_third_party());
        let effort_code = if show_effort {
            input
                .model
                .effort
                .as_deref()
                .and_then(Self::effort_code)
                .unwrap_or_default()
                .to_string()
        } else {
            String::new()
        };
        let thinking_icon = if !self.thinking_icon.is_empty() {
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
            let tier = if model_id.contains("ultra") {
                &pm.ultra
            } else if model_id.contains("flash-lite")
                || model_id.contains("flash_lite")
                || model_id.contains("flash lite")
            {
                &pm.flash_lite
            } else if model_id.contains("flash") {
                &pm.flash
            } else if model_id.contains("pro") {
                &pm.pro
            } else if model_id.contains("mythos") {
                &pm.mythos
            } else if model_id.contains("fable") {
                &pm.fable
            } else if model_id.contains("opus") {
                &pm.opus
            } else if model_id.contains("haiku") {
                &pm.haiku
            } else if model_id.contains("sonnet") {
                &pm.sonnet
            } else {
                &pm.flash
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
    fn test_model_effort_states() {
        use crate::config::types::ModelEffort;
        use crate::core::components::Component;
        use crate::core::input::{InputData, Model};

        let gemini_input = InputData {
            model: Model {
                id: "Gemini 3.7 Flash (High)".into(),
                display_name: "Gemini 3.7 Flash (High)".into(),
                effort: Some("high".into()),
            },
            ..Default::default()
        };

        let claude_input = InputData {
            model: Model {
                id: "claude-3-7-sonnet".into(),
                display_name: "Claude 3.7 Sonnet".into(),
                effort: Some("high".into()),
            },
            ..Default::default()
        };

        // Show: shows for both
        let show_comp = ModelComponent::new().with_effort(ModelEffort::Show);
        assert_eq!(show_comp.collect(&gemini_input).unwrap().secondary, "h");
        assert_eq!(show_comp.collect(&claude_input).unwrap().secondary, "h");

        // Hide: hides for both
        let hide_comp = ModelComponent::new().with_effort(ModelEffort::Hide);
        assert_eq!(hide_comp.collect(&gemini_input).unwrap().secondary, "");
        assert_eq!(hide_comp.collect(&claude_input).unwrap().secondary, "");

        // Gemini: shows for Gemini, hides for Claude (3p)
        let gemini_comp = ModelComponent::new().with_effort(ModelEffort::Gemini);
        assert_eq!(gemini_comp.collect(&gemini_input).unwrap().secondary, "h");
        assert_eq!(gemini_comp.collect(&claude_input).unwrap().secondary, "");

        // ThirdParty: hides for Gemini, shows for Claude (3p)
        let tp_comp = ModelComponent::new().with_effort(ModelEffort::ThirdParty);
        assert_eq!(tp_comp.collect(&gemini_input).unwrap().secondary, "");
        assert_eq!(tp_comp.collect(&claude_input).unwrap().secondary, "h");
    }
}
