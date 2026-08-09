use crate::config::theme::UserTheme;
use crate::config::types::{ComponentConfig, ComponentId, DEFAULT_HOSTNAME_RSTRIP};
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
