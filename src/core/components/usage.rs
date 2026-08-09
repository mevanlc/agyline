use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::{InputData, RateLimitWindow};
use std::collections::HashMap;

#[derive(Default)]
pub struct FiveHourUsageComponent;

impl FiveHourUsageComponent {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Default)]
pub struct SevenDayUsageComponent;

impl SevenDayUsageComponent {
    pub fn new() -> Self {
        Self
    }
}

fn collect_window(label: &str, window: Option<&RateLimitWindow>) -> Option<ComponentData> {
    let window = window?;
    let percentage = window.used_percentage?;
    let value = if percentage.fract() == 0.0 {
        format!("{percentage:.0}")
    } else {
        format!("{percentage:.1}")
    };

    let mut metadata = HashMap::from([("percentage".into(), percentage.to_string())]);
    if let Some(resets_at) = window.resets_at {
        metadata.insert("resets_at".into(), resets_at.to_string());
    }

    Some(ComponentData {
        primary: format!("{label} {value}%"),
        secondary: String::new(),
        metadata,
    })
}

impl Component for FiveHourUsageComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        collect_window(
            "5h",
            input
                .rate_limits
                .as_ref()
                .and_then(|limits| limits.five_hour.as_ref()),
        )
    }

    fn id(&self) -> ComponentId {
        ComponentId::UsageFiveHour
    }
}

impl Component for SevenDayUsageComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        collect_window(
            "7d",
            input
                .rate_limits
                .as_ref()
                .and_then(|limits| limits.seven_day.as_ref()),
        )
    }

    fn id(&self) -> ComponentId {
        ComponentId::UsageSevenDay
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, FiveHourUsageComponent, SevenDayUsageComponent};
    use crate::core::input::{InputData, Model, RateLimitWindow, RateLimits, Workspace};

    fn input(rate_limits: Option<RateLimits>) -> InputData {
        InputData {
            model: Model {
                id: "claude-sonnet-4-5".into(),
                display_name: "Sonnet 4.5".into(),
            },
            workspace: Workspace {
                current_dir: "/tmp/project".into(),
            },
            effort: None,
            thinking: None,
            context_window: None,
            rate_limits,
            cost: None,
            output_style: None,
        }
    }

    fn both_windows() -> InputData {
        input(Some(RateLimits {
            five_hour: Some(RateLimitWindow {
                used_percentage: Some(23.5),
                resets_at: Some(1_738_425_600),
            }),
            seven_day: Some(RateLimitWindow {
                used_percentage: Some(41.2),
                resets_at: Some(1_738_857_600),
            }),
        }))
    }

    #[test]
    fn five_hour_component_displays_only_five_hour_usage() {
        let data = FiveHourUsageComponent::new()
            .collect(&both_windows())
            .unwrap();

        assert_eq!(data.primary, "5h 23.5%");
        assert!(data.secondary.is_empty());
        assert_eq!(data.metadata.get("resets_at").unwrap(), "1738425600");
    }

    #[test]
    fn seven_day_component_displays_only_seven_day_usage() {
        let data = SevenDayUsageComponent::new()
            .collect(&both_windows())
            .unwrap();

        assert_eq!(data.primary, "7d 41.2%");
        assert!(data.secondary.is_empty());
        assert_eq!(data.metadata.get("resets_at").unwrap(), "1738857600");
    }

    #[test]
    fn each_component_requires_its_own_window_percentage() {
        let input = input(Some(RateLimits {
            five_hour: Some(RateLimitWindow {
                used_percentage: None,
                resets_at: Some(1_738_425_600),
            }),
            seven_day: Some(RateLimitWindow {
                used_percentage: Some(40.0),
                resets_at: None,
            }),
        }));

        assert!(FiveHourUsageComponent::new().collect(&input).is_none());
        assert_eq!(
            SevenDayUsageComponent::new()
                .collect(&input)
                .unwrap()
                .primary,
            "7d 40%"
        );
    }
}
