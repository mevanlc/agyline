use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct VimModeComponent;

impl VimModeComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for VimModeComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let vim = input.vim.as_ref()?;
        let mode = vim.mode.as_deref()?.trim();
        if mode.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        metadata.insert("vim_mode".into(), mode.to_string());

        Some(ComponentData {
            primary: mode.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::VimMode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_vim_mode() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "gemini-flash", "display_name": "Flash"},
            "workspace": {"current_dir": "/tmp"},
            "vim": {"mode": "NORMAL"}
        }))
        .unwrap();

        let data = VimModeComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "NORMAL");
    }
}
