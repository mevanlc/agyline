use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputData {
    pub model: Model,
    pub workspace: Workspace,
    pub effort: Option<Effort>,
    pub thinking: Option<Thinking>,
    pub context_window: Option<ContextWindow>,
    pub rate_limits: Option<RateLimits>,
    pub pr: Option<PullRequest>,
    pub cost: Option<Cost>,
    pub output_style: Option<OutputStyle>,
}

#[derive(Deserialize)]
pub struct Model {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub current_dir: String,
}

#[derive(Deserialize)]
pub struct Effort {
    pub level: Option<String>,
}

#[derive(Deserialize)]
pub struct Thinking {
    pub enabled: Option<bool>,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub context_window_size: Option<u64>,
    pub used_percentage: Option<f64>,
    pub remaining_percentage: Option<f64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Deserialize)]
pub struct CurrentUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Deserialize)]
pub struct RateLimits {
    pub five_hour: Option<RateLimitWindow>,
    pub seven_day: Option<RateLimitWindow>,
}

#[derive(Deserialize)]
pub struct RateLimitWindow {
    pub used_percentage: Option<f64>,
    pub resets_at: Option<u64>,
}

#[derive(Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub url: String,
    pub review_state: Option<String>,
}

#[derive(Deserialize)]
pub struct Cost {
    pub total_cost_usd: Option<f64>,
    pub total_duration_ms: Option<u64>,
    pub total_api_duration_ms: Option<u64>,
    pub total_lines_added: Option<u32>,
    pub total_lines_removed: Option<u32>,
}

#[derive(Deserialize)]
pub struct OutputStyle {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::InputData;

    #[test]
    fn deserializes_native_context_rate_limit_and_pull_request_payloads() {
        let input: InputData = serde_json::from_str(
            r#"{
                "model": {"id": "claude-sonnet-4-5", "display_name": "Sonnet 4.5"},
                "workspace": {"current_dir": "/tmp/project"},
                "effort": null,
                "thinking": null,
                "context_window": {
                    "total_input_tokens": 15500,
                    "total_output_tokens": 1200,
                    "context_window_size": 200000,
                    "used_percentage": 8,
                    "remaining_percentage": 92,
                    "current_usage": {
                        "input_tokens": 8500,
                        "output_tokens": 1200,
                        "cache_creation_input_tokens": 5000,
                        "cache_read_input_tokens": 2000
                    }
                },
                "rate_limits": {
                    "five_hour": {"used_percentage": 23.5, "resets_at": 1738425600},
                    "seven_day": {"used_percentage": 41.2, "resets_at": 1738857600}
                },
                "pr": {
                    "number": 482,
                    "url": "https://github.com/example/repo/pull/482",
                    "review_state": "approved"
                },
                "cost": null,
                "output_style": null
            }"#,
        )
        .unwrap();

        let context = input.context_window.unwrap();
        assert_eq!(context.total_input_tokens, Some(15_500));
        assert_eq!(
            context.current_usage.unwrap().cache_read_input_tokens,
            Some(2_000)
        );

        let limits = input.rate_limits.unwrap();
        assert_eq!(limits.five_hour.unwrap().used_percentage, Some(23.5));
        assert_eq!(limits.seven_day.unwrap().resets_at, Some(1_738_857_600));

        let pr = input.pr.unwrap();
        assert_eq!(pr.number, 482);
        assert_eq!(pr.url, "https://github.com/example/repo/pull/482");
        assert_eq!(pr.review_state.as_deref(), Some("approved"));
    }
}
