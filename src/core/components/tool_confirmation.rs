use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ToolConfirmationComponent;

impl ToolConfirmationComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ToolConfirmationComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let is_pending = input.tool_confirmation_pending.unwrap_or(false);
        if !is_pending {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("tool_confirmation_pending".into(), "true".into());

        Some(ComponentData {
            primary: "confirming".into(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::ToolConfirmation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_tool_confirmation_when_pending() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "tool_confirmation_pending": true
        }))
        .unwrap();

        let data = ToolConfirmationComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "confirming");
    }

    #[test]
    fn returns_none_when_not_pending() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "tool_confirmation_pending": false
        }))
        .unwrap();

        assert!(ToolConfirmationComponent::new().collect(&input).is_none());
    }
}
