use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::{ContextWindow, CurrentUsage, InputData};
use std::collections::HashMap;

#[derive(Default)]
pub struct ContextWindowComponent;

impl ContextWindowComponent {
    pub fn new() -> Self {
        Self
    }

    fn format_percentage(percentage: f64) -> String {
        if percentage.fract() == 0.0 {
            format!("{percentage:.0}%")
        } else {
            format!("{percentage:.1}%")
        }
    }

    fn format_tokens(tokens: u64) -> String {
        if tokens >= 1000 {
            let thousands = tokens as f64 / 1000.0;
            if thousands.fract() == 0.0 {
                format!("{thousands:.0}k")
            } else {
                format!("{thousands:.1}k")
            }
        } else {
            tokens.to_string()
        }
    }

    fn current_input_tokens(usage: &CurrentUsage) -> u64 {
        usage.input_tokens.unwrap_or(0)
            + usage.cache_creation_input_tokens.unwrap_or(0)
            + usage.cache_read_input_tokens.unwrap_or(0)
    }

    fn input_tokens(context: &ContextWindow) -> Option<u64> {
        context.total_input_tokens.or_else(|| {
            context
                .current_usage
                .as_ref()
                .map(Self::current_input_tokens)
        })
    }

    fn total_tokens(context: &ContextWindow) -> Option<u64> {
        match (context.total_input_tokens, context.total_output_tokens) {
            (Some(input), Some(output)) => Some(input + output),
            (Some(input), None) => Some(input),
            (None, Some(output)) => Some(output),
            (None, None) => context
                .current_usage
                .as_ref()
                .map(|usage| Self::current_input_tokens(usage) + usage.output_tokens.unwrap_or(0)),
        }
    }

    fn used_percentage(context: &ContextWindow) -> Option<f64> {
        context
            .used_percentage
            .or_else(|| {
                context
                    .remaining_percentage
                    .map(|remaining| 100.0 - remaining)
            })
            .or_else(|| {
                let input_tokens = Self::input_tokens(context)?;
                let limit = context.context_window_size.filter(|limit| *limit > 0)?;
                Some(input_tokens as f64 / limit as f64 * 100.0)
            })
    }
}

impl Component for ContextWindowComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let context = input.context_window.as_ref();
        let percentage = context.and_then(Self::used_percentage);
        let tokens = context.and_then(Self::total_tokens);

        let percentage_display = percentage
            .map(Self::format_percentage)
            .unwrap_or_else(|| "-".into());
        let tokens_display = tokens
            .map(Self::format_tokens)
            .unwrap_or_else(|| "-".into());

        let mut metadata = HashMap::new();
        metadata.insert(
            "percentage".into(),
            percentage
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".into()),
        );
        metadata.insert(
            "tokens".into(),
            tokens
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".into()),
        );
        metadata.insert(
            "limit".into(),
            context
                .and_then(|value| value.context_window_size)
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".into()),
        );

        Some(ComponentData {
            primary: format!("{percentage_display} \u{b7} {tokens_display} tokens"),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::ContextWindow
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, ContextWindowComponent};
    use crate::core::input::{ContextWindow, CurrentUsage, InputData, Model, Workspace};

    fn input(context_window: Option<ContextWindow>) -> InputData {
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
            context_window,
            rate_limits: None,
            pr: None,
            cost: None,
            output_style: None,
        }
    }

    #[test]
    fn uses_native_percentage_and_combined_token_totals() {
        let data = ContextWindowComponent::new()
            .collect(&input(Some(ContextWindow {
                total_input_tokens: Some(20_000),
                total_output_tokens: Some(4_000),
                context_window_size: Some(200_000),
                used_percentage: Some(10.0),
                remaining_percentage: Some(90.0),
                current_usage: None,
            })))
            .unwrap();

        assert_eq!(data.primary, "10% · 24k tokens");
        assert_eq!(data.metadata.get("percentage").unwrap(), "10");
        assert_eq!(data.metadata.get("tokens").unwrap(), "24000");
        assert_eq!(data.metadata.get("limit").unwrap(), "200000");
    }

    #[test]
    fn derives_missing_percentage_from_native_limit() {
        let data = ContextWindowComponent::new()
            .collect(&input(Some(ContextWindow {
                total_input_tokens: Some(25_000),
                total_output_tokens: Some(5_000),
                context_window_size: Some(200_000),
                used_percentage: None,
                remaining_percentage: None,
                current_usage: None,
            })))
            .unwrap();

        assert_eq!(data.primary, "12.5% · 30k tokens");
    }

    #[test]
    fn falls_back_to_native_current_usage_breakdown() {
        let data = ContextWindowComponent::new()
            .collect(&input(Some(ContextWindow {
                total_input_tokens: None,
                total_output_tokens: None,
                context_window_size: Some(200_000),
                used_percentage: None,
                remaining_percentage: Some(92.0),
                current_usage: Some(CurrentUsage {
                    input_tokens: Some(8_500),
                    output_tokens: Some(1_200),
                    cache_creation_input_tokens: Some(5_000),
                    cache_read_input_tokens: Some(2_000),
                }),
            })))
            .unwrap();

        assert_eq!(data.primary, "8% · 16.7k tokens");
    }

    #[test]
    fn renders_unknown_values_when_context_is_absent() {
        let data = ContextWindowComponent::new().collect(&input(None)).unwrap();

        assert_eq!(data.primary, "- · - tokens");
    }
}
