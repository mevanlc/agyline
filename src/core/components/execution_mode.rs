use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ExecutionModeComponent;

impl ExecutionModeComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ExecutionModeComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let mode = input.execution_mode.as_deref()?.trim();
        if mode.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("execution_mode".into(), mode.to_string());

        Some(ComponentData {
            primary: mode.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::ExecutionMode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_execution_mode() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "execution_mode": "planning"
        }))
        .unwrap();

        let data = ExecutionModeComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "planning");
    }
}
