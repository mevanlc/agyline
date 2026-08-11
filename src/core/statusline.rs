use crate::config::theme::UserTheme;
use crate::config::types::{
    ComponentConfig, ComponentId, DEFAULT_HOSTNAME_RSTRIP, DEFAULT_PR_OSC_HYPERLINKS,
    DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL, PR_OPTION_OSC_HYPERLINKS,
    PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL,
};
use crate::core::components::ComponentData;
use crate::core::render;

pub struct StatusLineGenerator<'a> {
    theme: &'a UserTheme,
}

impl<'a> StatusLineGenerator<'a> {
    pub fn new(theme: &'a UserTheme) -> Self {
        Self { theme }
    }

    pub fn generate(&self, components: Vec<(ComponentConfig, ComponentData)>) -> String {
        let (texts, dynamic_icons) = render::texts_and_icons_from_data(&components);

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
            ComponentId::Model => {
                let show_effort = comp_cfg
                    .options
                    .get("show_effort")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let thinking_icon = comp_cfg
                    .options
                    .get("thinking_icon")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                ModelComponent::new()
                    .with_per_model(comp_cfg.icon.per_model.clone())
                    .with_effort(show_effort)
                    .with_thinking_icon(thinking_icon)
                    .collect(input)
            }
            ComponentId::Directory => DirectoryComponent::new().collect(input),
            ComponentId::Hostname => {
                let rstrip = comp_cfg
                    .options
                    .get("rstrip")
                    .and_then(|value| value.as_str())
                    .unwrap_or(DEFAULT_HOSTNAME_RSTRIP);
                HostnameComponent::new().with_rstrip(rstrip).collect(input)
            }
            ComponentId::Git => {
                let show_sha = comp_cfg
                    .options
                    .get("show_sha")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                GitComponent::new().with_sha(show_sha).collect(input)
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
            ComponentId::ContextWindow => ContextWindowComponent::new().collect(input),
            ComponentId::UsageFiveHour => FiveHourUsageComponent::new().collect(input),
            ComponentId::UsageSevenDay => SevenDayUsageComponent::new().collect(input),
            ComponentId::Cost => CostComponent::new().collect(input),
            ComponentId::Session => SessionComponent::new().collect(input),
            ComponentId::OutputStyle => OutputStyleComponent::new().collect(input),
            ComponentId::Separator => unreachable!(),
        };

        if let Some(data) = data {
            results.push((comp_cfg.clone(), data));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::{StatusLineGenerator, collect_all_components};
    use crate::config::theme::UserTheme;
    use crate::config::types::{
        ComponentId, PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL,
    };
    use crate::core::input::InputData;

    const URL: &str = "https://github.com/example/repo/pull/482";

    fn input() -> InputData {
        serde_json::from_value(serde_json::json!({
            "model": {"id": "claude-sonnet-4-5", "display_name": "Sonnet 4.5"},
            "workspace": {"current_dir": "/tmp/project"},
            "pr": {"number": 482, "url": URL, "review_state": "approved"},
        }))
        .unwrap()
    }

    fn pr_theme(osc_hyperlinks: bool) -> UserTheme {
        let mut theme = UserTheme::default_theme();
        for component in &mut theme.components {
            component.enabled = false;
        }
        let pull_request = theme.get_component_mut(ComponentId::PullRequest).unwrap();
        pull_request.enabled = true;
        pull_request.icon.plain.clear();
        pull_request.colors.icon = None;
        pull_request.colors.text = None;
        pull_request.options.insert(
            PR_OPTION_SHOW_REVIEW_STATE.into(),
            serde_json::Value::Bool(true),
        );
        pull_request
            .options
            .insert(PR_OPTION_SHOW_URL.into(), serde_json::Value::Bool(true));
        pull_request.options.insert(
            PR_OPTION_OSC_HYPERLINKS.into(),
            serde_json::Value::Bool(osc_hyperlinks),
        );
        theme
    }

    fn link(text: &str) -> String {
        format!("\x1b]8;;{URL}\x1b\\{text}\x1b]8;;\x1b\\")
    }

    #[test]
    fn renders_a_separate_osc_hyperlink_for_every_visible_pr_field() {
        let theme = pr_theme(true);
        let components = collect_all_components(&theme, &input());
        let rendered = StatusLineGenerator::new(&theme).generate(components);

        assert_eq!(
            rendered,
            format!("{} {} {}", link("#482"), link("approved"), link(URL))
        );
    }

    #[test]
    fn renders_no_osc_hyperlinks_when_the_option_is_off() {
        let theme = pr_theme(false);
        let components = collect_all_components(&theme, &input());
        let rendered = StatusLineGenerator::new(&theme).generate(components);

        assert_eq!(rendered, format!("#482 approved {URL}"));
        assert!(!rendered.contains("\x1b]8;;"));
    }
}
