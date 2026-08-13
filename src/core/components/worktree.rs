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
        let (name, displays_branch) = if let Some(worktree) = &input.worktree {
            (worktree.name.clone(), false)
        } else if input.workspace.git_worktree.is_some() {
            (
                DirectoryComponent::directory_name(&input.workspace.current_dir),
                false,
            )
        } else {
            match self.outside_worktrees {
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
            }
        };

        if name.is_empty() {
            return None;
        }

        let mut metadata = HashMap::new();
        if displays_branch {
            metadata.insert(METADATA_DISPLAYED_BRANCH.into(), name.clone());
        }
        if let Some(git_worktree) = &input.workspace.git_worktree {
            metadata.insert("git_worktree".into(), git_worktree.clone());
        }
        if let Some(worktree) = &input.worktree {
            metadata.insert("name".into(), worktree.name.clone());
            metadata.insert("path".into(), worktree.path.clone());
            metadata.insert("original_cwd".into(), worktree.original_cwd.clone());
            if let Some(branch) = &worktree.branch {
                metadata.insert("branch".into(), branch.clone());
            }
            if let Some(original_branch) = &worktree.original_branch {
                metadata.insert("original_branch".into(), original_branch.clone());
            }
        }

        let secondary = if self.show_original_branch {
            input
                .worktree
                .as_ref()
                .and_then(|worktree| worktree.original_branch.as_deref())
                .filter(|branch| !branch.is_empty())
                .map(|branch| format!("\u{2190} {branch}"))
                .unwrap_or_default()
        } else {
            String::new()
        };

        Some(ComponentData {
            primary: name,
            secondary,
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
    fn suppresses_itself_in_the_primary_checkout() {
        let input = input(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
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
    fn rich_worktree_name_wins_over_path_and_git_admin_name() {
        let input = input(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
            "workspace": {
                "current_dir": "/tmp/project/.claude/worktrees/actual-directory",
                "git_worktree": "actual-directory1"
            },
            "worktree": {
                "name": "logical-name",
                "path": "/tmp/project/.claude/worktrees/actual-directory",
                "branch": "worktree-logical-name",
                "original_cwd": "/tmp/project",
                "original_branch": "main"
            }
        }));

        let data = WorktreeComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "logical-name");
        assert!(data.secondary.is_empty());
        assert_eq!(
            data.metadata.get("git_worktree").map(String::as_str),
            Some("actual-directory1")
        );
    }

    #[test]
    fn legacy_worktree_uses_current_directory_basename_not_git_admin_name() {
        let input = input(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
            "workspace": {
                "current_dir": "C:\\repos\\shared\\",
                "git_worktree": "shared1"
            }
        }));

        let data = WorktreeComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "shared");
    }

    #[test]
    fn original_branch_is_the_only_optional_display_field() {
        let input = input(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
            "workspace": {"current_dir": "/tmp/hook-target"},
            "worktree": {
                "name": "logical-name",
                "path": "/tmp/hook-target",
                "original_cwd": "/tmp/non-git-origin",
                "original_branch": "main"
            }
        }));

        let data = WorktreeComponent::new()
            .with_original_branch(true)
            .collect(&input)
            .unwrap();
        assert_eq!(data.primary, "logical-name");
        assert_eq!(data.secondary, "\u{2190} main");
    }

    #[test]
    fn show_mode_keeps_the_component_visible_as_a_seatbelt() {
        let input = input(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
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
            "model": {"id": "sonnet", "display_name": "Sonnet"},
            "workspace": {"current_dir": "C:\\repos\\project\\"}
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
            "model": {"id": "sonnet", "display_name": "Sonnet"},
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
