use super::{Component, ComponentData};
use crate::config::types::{ComponentId, UsageValue};
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct FiveHourUsageComponent {
    value: UsageValue,
}

impl FiveHourUsageComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: UsageValue) -> Self {
        self.value = value;
        self
    }
}

#[derive(Default)]
pub struct SevenDayUsageComponent {
    value: UsageValue,
}

impl SevenDayUsageComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: UsageValue) -> Self {
        self.value = value;
        self
    }
}

fn collect_quota_window(
    label: &str,
    bucket: Option<&crate::core::input::QuotaBucket>,
    value: UsageValue,
) -> Option<ComponentData> {
    let bucket = bucket?;
    let fraction = bucket.remaining_fraction?;
    let used_percentage = ((1.0 - fraction) * 100.0).clamp(0.0, 100.0);
    let displayed = value.apply(used_percentage);
    let value_str = if displayed.fract() == 0.0 {
        format!("{displayed:.0}")
    } else {
        format!("{displayed:.1}")
    };

    let mut metadata = HashMap::from([("percentage".into(), used_percentage.to_string())]);
    if let Some(reset_sec) = bucket.reset_in_seconds {
        metadata.insert("reset_in_seconds".into(), reset_sec.to_string());
    }

    Some(ComponentData {
        primary: format!("{label} {value_str}%"),
        secondary: String::new(),
        metadata,
    })
}

impl Component for FiveHourUsageComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let quota = input.quota.as_ref()?;
        let is_3p = input.model.is_third_party();

        let bucket = if is_3p {
            quota
                .get("3p-5h")
                .or_else(|| quota.get("gemini-5h"))
                .or_else(|| quota.get("5h"))
        } else {
            quota
                .get("gemini-5h")
                .or_else(|| quota.get("3p-5h"))
                .or_else(|| quota.get("5h"))
        };

        collect_quota_window("5h", bucket, self.value)
    }

    fn id(&self) -> ComponentId {
        ComponentId::UsageFiveHour
    }
}

impl Component for SevenDayUsageComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let quota = input.quota.as_ref()?;
        let is_3p = input.model.is_third_party();

        let bucket = if is_3p {
            quota
                .get("3p-weekly")
                .or_else(|| quota.get("gemini-weekly"))
                .or_else(|| quota.get("weekly"))
                .or_else(|| quota.get("7d"))
        } else {
            quota
                .get("gemini-weekly")
                .or_else(|| quota.get("3p-weekly"))
                .or_else(|| quota.get("weekly"))
                .or_else(|| quota.get("7d"))
        };

        collect_quota_window("7d", bucket, self.value)
    }

    fn id(&self) -> ComponentId {
        ComponentId::UsageSevenDay
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, FiveHourUsageComponent, SevenDayUsageComponent};
    use crate::config::types::UsageValue;
    use crate::core::input::InputData;

    #[test]
    fn test_antigravity_quota_for_5h_and_7d() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "Gemini 3.7 Flash (High)", "display_name": "Gemini 3.7 Flash (High)", "effort": "high"},
            "workspace": {"current_dir": "/tmp"},
            "quota": {
                "gemini-5h": {
                    "remaining_fraction": 0.914,
                    "reset_in_seconds": 14550,
                    "reset_time": "2026-08-17T08:23:39Z"
                },
                "gemini-weekly": {
                    "remaining_fraction": 0.554,
                    "reset_in_seconds": 259390,
                    "reset_time": "2026-08-20T04:24:24Z"
                }
            }
        }))
        .unwrap();

        let five_hour_default = FiveHourUsageComponent::new().collect(&input).unwrap();
        assert_eq!(five_hour_default.primary, "5h 91.4%");

        let five_hour_rem = FiveHourUsageComponent::new()
            .with_value(UsageValue::Remaining)
            .collect(&input)
            .unwrap();
        assert_eq!(five_hour_rem.primary, "5h 91.4%");

        let five_hour_used = FiveHourUsageComponent::new()
            .with_value(UsageValue::Used)
            .collect(&input)
            .unwrap();
        assert_eq!(five_hour_used.primary, "5h 8.6%");

        let seven_day_rem = SevenDayUsageComponent::new()
            .with_value(UsageValue::Remaining)
            .collect(&input)
            .unwrap();
        assert_eq!(seven_day_rem.primary, "7d 55.4%");
    }

    #[test]
    fn test_antigravity_3p_model_quota_switching() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "claude-3-7-sonnet", "display_name": "Claude 3.7 Sonnet"},
            "workspace": {"current_dir": "/tmp"},
            "quota": {
                "3p-5h": {
                    "remaining_fraction": 1.0,
                    "reset_in_seconds": 18000,
                    "reset_time": "2026-08-17T09:21:00Z"
                },
                "3p-weekly": {
                    "remaining_fraction": 0.666,
                    "reset_in_seconds": 286000,
                    "reset_time": "2026-08-20T11:57:07Z"
                },
                "gemini-5h": {
                    "remaining_fraction": 0.914,
                    "reset_in_seconds": 14550,
                    "reset_time": "2026-08-17T08:23:39Z"
                },
                "gemini-weekly": {
                    "remaining_fraction": 0.554,
                    "reset_in_seconds": 259390,
                    "reset_time": "2026-08-20T04:24:24Z"
                }
            }
        }))
        .unwrap();

        let five_hour = FiveHourUsageComponent::new()
            .with_value(UsageValue::Remaining)
            .collect(&input)
            .unwrap();
        assert_eq!(five_hour.primary, "5h 100%");

        let seven_day = SevenDayUsageComponent::new()
            .with_value(UsageValue::Remaining)
            .collect(&input)
            .unwrap();
        assert_eq!(seven_day.primary, "7d 66.6%");
    }
}
