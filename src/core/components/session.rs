use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct SessionComponent;

impl SessionComponent {
    pub fn new() -> Self {
        Self
    }

    fn format_duration(ms: u64) -> String {
        if ms < 1000 {
            format!("{}ms", ms)
        } else if ms < 60_000 {
            format!("{}s", ms / 1000)
        } else if ms < 3_600_000 {
            let m = ms / 60_000;
            let s = (ms % 60_000) / 1000;
            if s == 0 {
                format!("{}m", m)
            } else {
                format!("{}m{}s", m, s)
            }
        } else {
            let h = ms / 3_600_000;
            let m = (ms % 3_600_000) / 60_000;
            if m == 0 {
                format!("{}h", h)
            } else {
                format!("{}h{}m", h, m)
            }
        }
    }
}

impl Component for SessionComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        if let Some(cost_data) = &input.cost
            && let Some(duration) = cost_data.total_duration_ms
        {
            let primary = Self::format_duration(duration);

            let secondary = match (cost_data.total_lines_added, cost_data.total_lines_removed) {
                (Some(a), Some(r)) if a > 0 || r > 0 => {
                    format!("\x1b[32m+{}\x1b[0m \x1b[31m-{}\x1b[0m", a, r)
                }
                (Some(a), None) if a > 0 => format!("\x1b[32m+{}\x1b[0m", a),
                (None, Some(r)) if r > 0 => format!("\x1b[31m-{}\x1b[0m", r),
                _ => String::new(),
            };

            let mut metadata = HashMap::new();
            metadata.insert("duration_ms".into(), duration.to_string());
            if let Some(api) = cost_data.total_api_duration_ms {
                metadata.insert("api_duration_ms".into(), api.to_string());
            }
            if let Some(a) = cost_data.total_lines_added {
                metadata.insert("lines_added".into(), a.to_string());
            }
            if let Some(r) = cost_data.total_lines_removed {
                metadata.insert("lines_removed".into(), r.to_string());
            }

            return Some(ComponentData {
                primary,
                secondary,
                metadata,
            });
        }

        if let Some(conv_id) = input
            .conversation_id
            .as_deref()
            .or(input.session_id.as_deref())
        {
            let short_id = if conv_id.len() > 8 {
                &conv_id[..8]
            } else {
                conv_id
            };
            let mut metadata = HashMap::new();
            metadata.insert("conversation_id".into(), conv_id.to_string());
            return Some(ComponentData {
                primary: short_id.to_string(),
                secondary: String::new(),
                metadata,
            });
        }

        None
    }

    fn id(&self) -> ComponentId {
        ComponentId::Session
    }
}
