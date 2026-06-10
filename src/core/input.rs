use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct InputData {
    pub model: Model,
    pub workspace: Workspace,
    pub effort: Option<Effort>,
    pub thinking: Option<Thinking>,
    pub transcript_path: String,
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

// Transcript parsing types

#[derive(Deserialize)]
pub struct TranscriptEntry {
    pub r#type: Option<String>,
    pub message: Option<TranscriptMessage>,
    #[serde(rename = "leafUuid")]
    pub leaf_uuid: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct TranscriptMessage {
    pub usage: Option<RawUsage>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PromptTokensDetails {
    #[serde(default)]
    pub cached_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RawUsage {
    #[serde(default)]
    pub input_tokens: Option<u32>,
    #[serde(default)]
    pub prompt_tokens: Option<u32>,
    #[serde(default)]
    pub output_tokens: Option<u32>,
    #[serde(default)]
    pub completion_tokens: Option<u32>,
    #[serde(default)]
    pub total_tokens: Option<u32>,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u32>,
    #[serde(default)]
    pub cache_creation_prompt_tokens: Option<u32>,
    #[serde(default)]
    pub cache_read_prompt_tokens: Option<u32>,
    #[serde(default)]
    pub cached_tokens: Option<u32>,
    #[serde(default)]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    #[serde(flatten, skip_serializing)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct NormalizedUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    pub cache_creation_input_tokens: u32,
    pub cache_read_input_tokens: u32,
}

impl NormalizedUsage {
    pub fn context_tokens(&self) -> u32 {
        self.input_tokens
            + self.cache_creation_input_tokens
            + self.cache_read_input_tokens
            + self.output_tokens
    }

    pub fn display_tokens(&self) -> u32 {
        let context = self.context_tokens();
        if context > 0 {
            return context;
        }
        if self.total_tokens > 0 {
            return self.total_tokens;
        }
        self.input_tokens.max(self.output_tokens)
    }
}

impl RawUsage {
    pub fn normalize(self) -> NormalizedUsage {
        let input = self.input_tokens.or(self.prompt_tokens).unwrap_or(0);
        let output = self.output_tokens.or(self.completion_tokens).unwrap_or(0);
        let total = self.total_tokens.unwrap_or(0);

        let cache_creation = self
            .cache_creation_input_tokens
            .or(self.cache_creation_prompt_tokens)
            .unwrap_or(0);

        let cache_read = self
            .cache_read_input_tokens
            .or(self.cache_read_prompt_tokens)
            .or(self.cached_tokens)
            .or_else(|| {
                self.prompt_tokens_details
                    .as_ref()
                    .and_then(|d| d.cached_tokens)
            })
            .unwrap_or(0);

        let total_value = if total > 0 {
            total
        } else if input > 0 || output > 0 || cache_read > 0 || cache_creation > 0 {
            input + output + cache_read + cache_creation
        } else {
            0
        };

        NormalizedUsage {
            input_tokens: input,
            output_tokens: output,
            total_tokens: total_value,
            cache_creation_input_tokens: cache_creation,
            cache_read_input_tokens: cache_read,
        }
    }
}
