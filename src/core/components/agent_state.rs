use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct AgentStateComponent;

impl AgentStateComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for AgentStateComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let state = input.agent_state.as_deref()?.trim();
        if state.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("agent_state".into(), state.to_string());

        Some(ComponentData {
            primary: state.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::AgentState
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_agent_state() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "agent_state": "working",
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"}
        }))
        .unwrap();

        let data = AgentStateComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "working");
    }
}
