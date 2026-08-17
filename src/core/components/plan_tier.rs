use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct PlanTierComponent;

impl PlanTierComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for PlanTierComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let tier = input.plan_tier.as_deref()?.trim();
        if tier.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("plan_tier".into(), tier.to_string());

        Some(ComponentData {
            primary: tier.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::PlanTier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_plan_tier() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "plan_tier": "Pro"
        }))
        .unwrap();

        let data = PlanTierComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "Pro");
    }
}
