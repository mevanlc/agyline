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
        let cycle = input.cycle_mode.as_deref().unwrap_or("").trim();
        let exec = input.execution_mode.as_deref().unwrap_or("").trim();

        if cycle.is_empty() && exec.is_empty() {
            return None;
        }

        let primary = match (!cycle.is_empty(), !exec.is_empty()) {
            (true, true) => format!("{} · {}", cycle, exec),
            (true, false) => cycle.to_string(),
            (false, true) => exec.to_string(),
            (false, false) => unreachable!(),
        };

        let mut metadata = HashMap::new();
        if !cycle.is_empty() {
            metadata.insert("cycle_mode".into(), cycle.to_string());
        }
        if !exec.is_empty() {
            metadata.insert("execution_mode".into(), exec.to_string());
        }

        Some(ComponentData {
            primary,
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
