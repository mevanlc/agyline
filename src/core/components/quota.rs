use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct QuotaComponent {
    bucket_name: Option<String>,
}

impl QuotaComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_bucket(mut self, bucket_name: Option<String>) -> Self {
        self.bucket_name = bucket_name;
        self
    }
}

impl Component for QuotaComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let quota_map = input.quota.as_ref()?;
        if quota_map.is_empty() {
            return None;
        }

        // Find the requested bucket or first available
        let (bucket_key, bucket) = if let Some(ref req_key) = self.bucket_name {
            quota_map
                .get_key_value(req_key)
                .map(|(k, v)| (k.as_str(), v))?
        } else if let Some((k, v)) = quota_map.get_key_value("gemini-weekly") {
            (k.as_str(), v)
        } else {
            let (k, v) = quota_map.iter().next()?;
            (k.as_str(), v)
        };

        let label = bucket_key.strip_prefix("gemini-").unwrap_or(bucket_key);

        let percentage_str = if let Some(fraction) = bucket.remaining_fraction {
            format!("{:.0}%", (fraction * 100.0).clamp(0.0, 100.0))
        } else {
            "-".into()
        };

        let primary = format!("{label} {percentage_str}");

        let mut metadata = HashMap::new();
        metadata.insert("bucket".into(), bucket_key.to_string());
        if let Some(fraction) = bucket.remaining_fraction {
            metadata.insert("remaining_fraction".into(), fraction.to_string());
        }
        if let Some(reset_sec) = bucket.reset_in_seconds {
            metadata.insert("reset_in_seconds".into(), reset_sec.to_string());
        }

        Some(ComponentData {
            primary,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Quota
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::input::InputData;

    #[test]
    fn collects_quota_info() {
        let input: InputData = serde_json::from_value(serde_json::json!({
            "model": {"id": "gemini-flash", "display_name": "Flash"},
            "workspace": {"current_dir": "/tmp"},
            "quota": {
                "gemini-weekly": {
                    "remaining_fraction": 0.9378,
                    "reset_time": "2026-07-06T07:50:32Z",
                    "reset_in_seconds": 560580
                }
            }
        }))
        .unwrap();

        let data = QuotaComponent::new().collect(&input).unwrap();
        assert_eq!(data.primary, "weekly 94%");
    }
}
