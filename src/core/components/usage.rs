use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::{InputData, RateLimitWindow};
use std::collections::HashMap;

#[derive(Default)]
pub struct UsageComponent;

impl UsageComponent {
    pub fn new() -> Self {
        Self
    }

    fn format_window(label: &str, window: &RateLimitWindow) -> Option<String> {
        let percentage = window.used_percentage?;
        let value = if percentage.fract() == 0.0 {
            format!("{percentage:.0}")
        } else {
            format!("{percentage:.1}")
        };
        Some(format!("{label} {value}%"))
    }
}

impl Component for UsageComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let limits = input.rate_limits.as_ref()?;
        let five_hour = limits
            .five_hour
            .as_ref()
            .and_then(|window| Self::format_window("5h", window));
        let seven_day = limits
            .seven_day
            .as_ref()
            .and_then(|window| Self::format_window("7d", window));

        let (primary, secondary) = match (five_hour, seven_day) {
            (Some(five_hour), Some(seven_day)) => (five_hour, seven_day),
            (Some(five_hour), None) => (five_hour, String::new()),
            (None, Some(seven_day)) => (seven_day, String::new()),
            (None, None) => return None,
        };

        let mut metadata = HashMap::new();
        if let Some(window) = &limits.five_hour {
            if let Some(percentage) = window.used_percentage {
                metadata.insert("five_hour_percentage".into(), percentage.to_string());
            }
            if let Some(resets_at) = window.resets_at {
                metadata.insert("five_hour_resets_at".into(), resets_at.to_string());
            }
        }
        if let Some(window) = &limits.seven_day {
            if let Some(percentage) = window.used_percentage {
                metadata.insert("seven_day_percentage".into(), percentage.to_string());
            }
            if let Some(resets_at) = window.resets_at {
                metadata.insert("seven_day_resets_at".into(), resets_at.to_string());
            }
        }

        Some(ComponentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Usage
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, UsageComponent};
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

    #[test]
    fn displays_both_rate_limit_windows() {
        let data = UsageComponent::new()
            .collect(&input(Some(RateLimits {
                five_hour: Some(RateLimitWindow {
                    used_percentage: Some(23.5),
                    resets_at: Some(1_738_425_600),
                }),
                seven_day: Some(RateLimitWindow {
                    used_percentage: Some(41.2),
                    resets_at: Some(1_738_857_600),
                }),
            })))
            .unwrap();

        assert_eq!(data.primary, "5h 23.5%");
        assert_eq!(data.secondary, "7d 41.2%");
        assert_eq!(
            data.metadata.get("five_hour_resets_at").unwrap(),
            "1738425600"
        );
    }

    #[test]
    fn promotes_the_only_available_window_to_primary() {
        let data = UsageComponent::new()
            .collect(&input(Some(RateLimits {
                five_hour: None,
                seven_day: Some(RateLimitWindow {
                    used_percentage: Some(40.0),
                    resets_at: None,
                }),
            })))
            .unwrap();

        assert_eq!(data.primary, "7d 40%");
        assert!(data.secondary.is_empty());
    }

    #[test]
    fn suppresses_component_without_percentages() {
        let data = UsageComponent::new().collect(&input(Some(RateLimits {
            five_hour: Some(RateLimitWindow {
                used_percentage: None,
                resets_at: Some(1_738_425_600),
            }),
            seven_day: None,
        })));

        assert!(data.is_none());
    }
}
