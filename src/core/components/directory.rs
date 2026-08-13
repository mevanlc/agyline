use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectoryComponent;

impl DirectoryComponent {
    pub fn new() -> Self {
        Self
    }

    pub(crate) fn directory_name(path: &str) -> String {
        let path = path.trim_end_matches(['/', '\\']);
        let unix_name = path.split('/').next_back().unwrap_or("");
        let windows_name = path.split('\\').next_back().unwrap_or("");

        let result = if windows_name.len() < path.len() {
            windows_name
        } else if unix_name.len() < path.len() {
            unix_name
        } else {
            path
        };

        if result.is_empty() {
            "root".to_string()
        } else {
            result.to_string()
        }
    }
}

impl Component for DirectoryComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let dir_name = Self::directory_name(&input.workspace.current_dir);

        let mut metadata = HashMap::new();
        metadata.insert("full_path".into(), input.workspace.current_dir.clone());

        Some(ComponentData {
            primary: dir_name,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Directory
    }
}
