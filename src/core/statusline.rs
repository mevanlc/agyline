use crate::config::theme::UserTheme;
use crate::config::types::{
    ComponentConfig, ComponentId, DEFAULT_GIT_AUTOHIDE_BRANCH, DEFAULT_HOSTNAME_RSTRIP,
    DEFAULT_PR_OSC_HYPERLINKS, DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL,
    DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH, GIT_OPTION_AUTOHIDE_BRANCH, PR_OPTION_OSC_HYPERLINKS,
    PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL, UsageValue,
    WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH, WorktreeOutside,
};
use crate::core::components::{ComponentData, METADATA_DISPLAYED_BRANCH};
use crate::core::render;

pub struct StatusLineGenerator<'a> {
    theme: &'a UserTheme,
}

impl<'a> StatusLineGenerator<'a> {
    pub fn new(theme: &'a UserTheme) -> Self {
        Self { theme }
    }

    pub fn generate(&self, components: Vec<(ComponentConfig, ComponentData)>) -> String {
        let (texts, dynamic_icons) =
            render::texts_and_icons_from_data_for_mode(&components, self.theme.style.mode);

        // Build the render line, then patch in any dynamic icon overrides
        let mut line =
            render::build_render_line(&self.theme.components, self.theme.style.mode, &texts);

        // Apply dynamic icon overrides (e.g. from component metadata)
        if !dynamic_icons.is_empty() {
            for item in &mut line.items {
                if let render::RenderItem::Seg(seg) = item
                    && let Some(icon_override) = dynamic_icons.get(&seg.id)
                {
                    seg.icon = icon_override.clone();
                }
            }
        }

        render::render_ansi(&line)
    }
}

pub fn collect_all_components(
    theme: &UserTheme,
    input: &crate::core::input::InputData,
) -> Vec<(ComponentConfig, ComponentData)> {
    use crate::core::components::*;

    let mut results = Vec::new();

    for comp_cfg in &theme.components {
        if !comp_cfg.enabled || comp_cfg.id == ComponentId::Separator {
            continue;
        }

        let data = match comp_cfg.id {
            ComponentId::AgentState => AgentStateComponent::new().collect(input),
            ComponentId::Model => {
                let effort = crate::config::types::ModelEffort::from_options(&comp_cfg.options);
                let thinking_icon = comp_cfg
                    .options
                    .get("thinking_icon")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ModelComponent::new()
                    .with_per_model(comp_cfg.icon.per_model.clone())
                    .with_effort(effort)
                    .with_thinking_icon(thinking_icon)
                    .collect(input)
            }
            ComponentId::Directory => DirectoryComponent::new().collect(input),
            ComponentId::Git => {
                let show_sha = comp_cfg
                    .options
                    .get("show_sha")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                GitComponent::new().with_sha(show_sha).collect(input)
            }
            ComponentId::ContextWindow => ContextWindowComponent::new().collect(input),
            ComponentId::TaskCount => TaskCountComponent::new().collect(input),
            ComponentId::ExecutionMode => ExecutionModeComponent::new().collect(input),
            ComponentId::VimMode => VimModeComponent::new().collect(input),
            ComponentId::ArtifactCount => ArtifactCountComponent::new().collect(input),
            ComponentId::PendingInput => PendingInputComponent::new().collect(input),
            ComponentId::ToolConfirmation => ToolConfirmationComponent::new().collect(input),
            ComponentId::Sandbox => SandboxComponent::new().collect(input),
            ComponentId::PlanTier => PlanTierComponent::new().collect(input),
            ComponentId::Email => EmailComponent::new().collect(input),
            ComponentId::Worktree => {
                let show_original_branch = comp_cfg
                    .options
                    .get(WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH)
                    .and_then(|value| value.as_bool())
                    .unwrap_or(DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH);
                WorktreeComponent::new()
                    .with_original_branch(show_original_branch)
                    .with_outside_worktrees(WorktreeOutside::from_options(&comp_cfg.options))
                    .collect(input)
            }
            ComponentId::Hostname => {
                let rstrip = comp_cfg
                    .options
                    .get("rstrip")
                    .and_then(|value| value.as_str())
                    .unwrap_or(DEFAULT_HOSTNAME_RSTRIP);
                HostnameComponent::new().with_rstrip(rstrip).collect(input)
            }
            ComponentId::PullRequest => {
                let show_review_state = comp_cfg
                    .options
                    .get(PR_OPTION_SHOW_REVIEW_STATE)
                    .and_then(|value| value.as_bool())
                    .unwrap_or(DEFAULT_PR_SHOW_REVIEW_STATE);
                let show_url = comp_cfg
                    .options
                    .get(PR_OPTION_SHOW_URL)
                    .and_then(|value| value.as_bool())
                    .unwrap_or(DEFAULT_PR_SHOW_URL);
                let osc_hyperlinks = comp_cfg
                    .options
                    .get(PR_OPTION_OSC_HYPERLINKS)
                    .and_then(|value| value.as_bool())
                    .unwrap_or(DEFAULT_PR_OSC_HYPERLINKS);
                PullRequestComponent::new()
                    .with_review_state(show_review_state)
                    .with_url(show_url)
                    .with_osc_hyperlinks(osc_hyperlinks)
                    .collect(input)
            }
            ComponentId::UsageFiveHour => FiveHourUsageComponent::new()
                .with_value(UsageValue::from_options(&comp_cfg.options))
                .collect(input),
            ComponentId::UsageSevenDay => SevenDayUsageComponent::new()
                .with_value(UsageValue::from_options(&comp_cfg.options))
                .collect(input),
            ComponentId::Cost => CostComponent::new().collect(input),
            ComponentId::Session => SessionComponent::new().collect(input),
            ComponentId::OutputStyle => OutputStyleComponent::new().collect(input),
            ComponentId::Separator => unreachable!(),
            ComponentId::Unknown => None,
        };

        if let Some(data) = data {
            results.push((comp_cfg.clone(), data));
        }
    }

    autohide_duplicate_git_branch(&mut results);
    results
}

fn autohide_duplicate_git_branch(components: &mut [(ComponentConfig, ComponentData)]) {
    let displayed_branches: Vec<String> = components
        .iter()
        .filter(|(config, _)| config.id != ComponentId::Git)
        .filter_map(|(_, data)| data.metadata.get(METADATA_DISPLAYED_BRANCH).cloned())
        .collect();

    let Some((config, data)) = components
        .iter_mut()
        .find(|(config, _)| config.id == ComponentId::Git)
    else {
        return;
    };
    let autohide = config
        .options
        .get(GIT_OPTION_AUTOHIDE_BRANCH)
        .and_then(|value| value.as_bool())
        .unwrap_or(DEFAULT_GIT_AUTOHIDE_BRANCH);

    if autohide
        && displayed_branches
            .iter()
            .any(|branch| branch == &data.primary)
    {
        data.primary = std::mem::take(&mut data.secondary);
    }
}

#[cfg(test)]
mod tests {
    use super::collect_all_components;
    use crate::config::theme::UserTheme;
    use crate::config::types::{
        ComponentId, GIT_OPTION_AUTOHIDE_BRANCH, WORKTREE_OPTION_OUTSIDE_WORKTREES,
    };
    use crate::core::input::InputData;
    use std::process::Command;

    #[test]
    fn collects_antigravity_statusline_components() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp/project"},
            "agent_state": "idle",
            "quota": {
                "gemini-weekly": {
                    "remaining_fraction": 0.55,
                    "reset_in_seconds": 250000,
                    "reset_time": "2026-08-20T04:24:24Z"
                }
            }
        }))
        .unwrap();

        let mut theme = UserTheme::default_theme();
        for component in &mut theme.components {
            component.enabled = matches!(
                component.id,
                ComponentId::Model | ComponentId::AgentState | ComponentId::UsageSevenDay
            );
        }

        let components = collect_all_components(&theme, &input);
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn git_autohides_only_a_duplicate_branch_field() {
        let repo = tempfile::tempdir().unwrap();
        let status = Command::new("git")
            .args(["init", "--quiet", "--initial-branch=topic"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        assert!(status.success());
        let input = serde_json::from_value(serde_json::json!({
            "model": {"id": "sonnet", "display_name": "Sonnet"},
            "workspace": {"current_dir": repo.path()}
        }))
        .unwrap();

        let mut theme = UserTheme::default_theme();
        for component in &mut theme.components {
            component.enabled = matches!(component.id, ComponentId::Worktree | ComponentId::Git);
        }

        let components = collect_all_components(&theme, &input);
        let worktree = components
            .iter()
            .find(|(config, _)| config.id == ComponentId::Worktree)
            .unwrap();
        let git = components
            .iter()
            .find(|(config, _)| config.id == ComponentId::Git)
            .unwrap();
        assert_eq!(worktree.1.primary, "topic");
        assert_eq!(git.1.primary, "\u{2713}");
        assert!(git.1.secondary.is_empty());

        theme
            .get_component_mut(ComponentId::Git)
            .unwrap()
            .options
            .insert(GIT_OPTION_AUTOHIDE_BRANCH.into(), false.into());
        let components = collect_all_components(&theme, &input);
        let git = components
            .iter()
            .find(|(config, _)| config.id == ComponentId::Git)
            .unwrap();
        assert_eq!(git.1.primary, "topic");
        assert_eq!(git.1.secondary, "\u{2713}");

        theme
            .get_component_mut(ComponentId::Git)
            .unwrap()
            .options
            .insert(GIT_OPTION_AUTOHIDE_BRANCH.into(), true.into());
        theme
            .get_component_mut(ComponentId::Worktree)
            .unwrap()
            .options
            .insert(WORKTREE_OPTION_OUTSIDE_WORKTREES.into(), "directory".into());
        let components = collect_all_components(&theme, &input);
        let git = components
            .iter()
            .find(|(config, _)| config.id == ComponentId::Git)
            .unwrap();
        assert_eq!(git.1.primary, "topic");
        assert_eq!(git.1.secondary, "\u{2713}");
    }
}
