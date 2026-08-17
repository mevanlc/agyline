use super::{
    Component, ComponentData, DirectoryComponent, GitComponent, METADATA_DISPLAYED_BRANCH,
};
use crate::config::types::{ComponentId, WorktreeOutside};
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct WorktreeComponent {
    show_original_branch: bool,
    outside_worktrees: WorktreeOutside,
}

impl WorktreeComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_original_branch(mut self, show: bool) -> Self {
        self.show_original_branch = show;
        self
    }

    pub fn with_outside_worktrees(mut self, mode: WorktreeOutside) -> Self {
        self.outside_worktrees = mode;
        self
    }
}

impl Component for WorktreeComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let (name, displays_branch) = match self.outside_worktrees {
            WorktreeOutside::Hide => return None,
            WorktreeOutside::Show => ("-".into(), false),
            WorktreeOutside::Branch => (
                GitComponent::branch_name(&input.workspace.current_dir)?,
                true,
            ),
            WorktreeOutside::Directory => (
                DirectoryComponent::directory_name(&input.workspace.current_dir),
                false,
            ),
        };

        if name.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        if displays_branch {
            metadata.insert(METADATA_DISPLAYED_BRANCH.into(), name.clone());
        }

        Some(ComponentData {
            primary: name,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Worktree
    }
}

#[cfg(test)]
mod tests {
    use super::WorktreeComponent;
    use crate::config::types::WorktreeOutside;
    use crate::core::components::{Component, METADATA_DISPLAYED_BRANCH};
    use crate::core::input::InputData;
    use std::process::Command;

    fn input(value: serde_json::Value) -> InputData {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn suppresses_itself_in_hide_mode() {
        let input = input(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp/project"}
        }));

        assert!(
            WorktreeComponent::new()
                .with_outside_worktrees(WorktreeOutside::Hide)
                .collect(&input)
                .is_none()
        );
    }

    #[test]
    fn show_mode_keeps_the_component_visible_as_a_seatbelt() {
        let input = input(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp/project"}
        }));

        let data = WorktreeComponent::new()
            .with_outside_worktrees(WorktreeOutside::Show)
            .collect(&input)
            .unwrap();
        assert_eq!(data.primary, "-");
    }

    #[test]
    fn directory_mode_matches_the_directory_component() {
        let input = input(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/repos/project"}
        }));

        let data = WorktreeComponent::new()
            .with_outside_worktrees(WorktreeOutside::Directory)
            .collect(&input)
            .unwrap();
        assert_eq!(data.primary, "project");
    }

    #[test]
    fn branch_mode_uses_the_current_git_branch() {
        let repo = tempfile::tempdir().unwrap();
        let status = Command::new("git")
            .args(["init", "--quiet", "--initial-branch=topic"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        assert!(status.success());
        let input = input(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": repo.path()}
        }));

        let data = WorktreeComponent::new()
            .with_outside_worktrees(WorktreeOutside::Branch)
            .collect(&input)
            .unwrap();
        assert_eq!(data.primary, "topic");
        assert_eq!(
            data.metadata
                .get(METADATA_DISPLAYED_BRANCH)
                .map(String::as_str),
            Some("topic")
        );
    }
}
