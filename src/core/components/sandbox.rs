use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct SandboxComponent;

impl SandboxComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for SandboxComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let sandbox = input.sandbox.as_ref()?;
        let enabled = sandbox.enabled.unwrap_or(false);
        if !enabled {
            return None;
        }

        let allow_net = sandbox.allow_network.unwrap_or(true);
        let primary = if allow_net {
            "sandbox".to_string()
        } else {
            "sandbox (no-net)".to_string()
        };

        let mut metadata = HashMap::new();
        metadata.insert("enabled".into(), "true".into());
        metadata.insert("allow_network".into(), allow_net.to_string());

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Sandbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_sandbox_when_enabled() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "sandbox": {"enabled": true, "allow_network": false}
        }))
        .unwrap();

        let data = SandboxComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "sandbox (no-net)");
    }
}
