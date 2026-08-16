use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct EmailComponent;

impl EmailComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for EmailComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let email = input.email.as_deref()?.trim();
        if email.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("email".into(), email.to_string());

        Some(ComponentData {
            primary: email.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Email
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_email() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "gemini-flash", "display_name": "Flash"},
            "workspace": {"current_dir": "/tmp"},
            "email": "user@example.com"
        }))
        .unwrap();

        let data = EmailComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "user@example.com");
    }
}
