use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct TaskCountComponent {
    show_zero: bool,
}

impl TaskCountComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_zero(mut self, show_zero: bool) -> Self {
        self.show_zero = show_zero;
        self
    }
}

impl Component for TaskCountComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let count = input.task_count?;
        if count == 0 && !self.show_zero {
            return None;
        }

        let primary = if count == 1 {
            "1 task".to_string()
        } else {
            format!("{count} tasks")
        };

        let mut metadata = HashMap::new();
        metadata.insert("task_count".into(), count.to_string());

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::TaskCount
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_task_count() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "gemini-flash", "display_name": "Flash"},
            "workspace": {"current_dir": "/tmp"},
            "task_count": 2
        }))
        .unwrap();

        let data = TaskCountComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "2 tasks");
    }

    #[test]
    fn suppresses_zero_by_default() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "gemini-flash", "display_name": "Flash"},
            "workspace": {"current_dir": "/tmp"},
            "task_count": 0
        }))
        .unwrap();

        assert!(TaskCountComponent::new().collect(&input).is_none());
    }
}
