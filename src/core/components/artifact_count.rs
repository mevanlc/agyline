use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct ArtifactCountComponent {
    show_zero: bool,
}

impl ArtifactCountComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }
}

impl Component for ArtifactCountComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let count = input.artifact_count?;
        if count == 0 && !self.show_zero {
            return None;
        }

        let primary = if count == 1 {
            "1 artifact".to_string()
        } else {
            format!("{count} artifacts")
        };

        let mut metadata = HashMap::new();
        metadata.insert("artifact_count".into(), count.to_string());

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::ArtifactCount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_artifact_count() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "artifact_count": 3
        }))
        .unwrap();

        let data = ArtifactCountComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "3 artifacts");
    }
}
