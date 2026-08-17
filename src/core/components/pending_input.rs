use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct PendingInputComponent {
    show_zero: bool,
}

impl PendingInputComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }
}

impl Component for PendingInputComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let count = input.pending_input_count?;
        if count == 0 && !self.show_zero {
            return None;
        }

        let primary = if count == 1 {
            "1 pending".to_string()
        } else {
            format!("{count} pending")
        };

        let mut metadata = HashMap::new();
        metadata.insert("pending_input_count".into(), count.to_string());

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::PendingInput
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_pending_input() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "pending_input_count": 2
        }))
        .unwrap();

        let data = PendingInputComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "2 pending");
    }
}
