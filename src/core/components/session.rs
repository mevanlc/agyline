use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct SessionComponent;

impl SessionComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for SessionComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        if let Some(conv_id) = input
            .conversation_id
            .as_deref()
            .or(input.session_id.as_deref())
        {
            if conv_id.is_empty() {
                return None;
            }
            let short_id = if conv_id.len() > 8 {
                &conv_id[..8]
            } else {
                conv_id
            };
            let mut metadata = HashMap::new();
            metadata.insert("conversation_id".into(), conv_id.to_string());
            return Some(ComponentData {
                primary: short_id.to_string(),
                secondary: String::new(),
                metadata,
            });
        }

        None
    }

    fn id(&self) -> ComponentId {
        ComponentId::Session
    }
}
