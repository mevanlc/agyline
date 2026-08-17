use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default)]
pub struct InputData {
    pub cwd: Option<String>,
    pub session_id: Option<String>,
    pub conversation_id: Option<String>,
    pub transcript_path: Option<String>,
    pub model: Model,
    pub workspace: Workspace,
    pub version: Option<String>,
    pub context_window: Option<ContextWindow>,
    pub exceeds_200k_tokens: Option<bool>,
    pub product: Option<String>,
    pub quota: Option<HashMap<String, QuotaBucket>>,
    pub agent_state: Option<String>,
    pub vcs: Option<Vcs>,
    pub sandbox: Option<Sandbox>,
    pub artifact_count: Option<u32>,
    pub plan_tier: Option<String>,
    pub email: Option<String>,
    pub pending_input_count: Option<u32>,
    pub tool_confirmation_pending: Option<bool>,
    pub task_count: Option<u32>,
    pub terminal_width: Option<u32>,
    pub execution_mode: Option<String>,
    pub vim: Option<VimState>,
}

#[derive(Deserialize, Default, Clone)]
pub struct Model {
    pub id: String,
    pub display_name: String,
    pub effort: Option<String>,
}

impl Model {
    pub fn is_third_party(&self) -> bool {
        let id = self.id.to_ascii_lowercase();
        let name = self.display_name.to_ascii_lowercase();

        !id.contains("gemini") && !name.contains("gemini")
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Workspace {
    pub current_dir: String,
    pub project_dir: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuotaBucket {
    pub remaining_fraction: Option<f64>,
    pub reset_time: Option<String>,
    pub reset_in_seconds: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Vcs {
    pub r#type: Option<String>,
    pub branch: Option<String>,
    pub client: Option<String>,
    pub dirty: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sandbox {
    pub enabled: Option<bool>,
    pub allow_network: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VimState {
    pub mode: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ContextWindow {
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub context_window_size: Option<u64>,
    pub used_percentage: Option<f64>,
    pub remaining_percentage: Option<f64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Deserialize, Clone)]
pub struct CurrentUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::InputData;

    #[test]
    fn deserializes_official_antigravity_payload() {
        let json = r#"{
            "cwd": "/home/user/my-project",
            "session_id": "12345678-abcd-ef01-2345-6789abcdef01",
            "conversation_id": "12345678-abcd-ef01-2345-6789abcdef01",
            "transcript_path": "/home/user/.gemini/antigravity/brain/12345678-abcd-ef01-2345-6789abcdef01/.system_generated/logs/transcript.jsonl",
            "model": {
                "id": "Gemini 3.7 Flash (High)",
                "display_name": "Gemini 3.7 Flash (High)",
                "effort": "high"
            },
            "workspace": {
                "current_dir": "/home/user/my-project",
                "project_dir": "/home/user/my-project"
            },
            "version": "1.0.13",
            "context_window": {
                "total_input_tokens": 88244,
                "total_output_tokens": 61074,
                "context_window_size": 1048576,
                "used_percentage": 14.24,
                "remaining_percentage": 85.76,
                "current_usage": {
                    "input_tokens": 63382,
                    "output_tokens": 346,
                    "cache_creation_input_tokens": 0,
                    "cache_read_input_tokens": 20857
                }
            },
            "exceeds_200k_tokens": false,
            "product": "antigravity",
            "quota": {
                "gemini-weekly": {
                    "remaining_fraction": 0.9378,
                    "reset_time": "2026-07-06T07:50:32Z",
                    "reset_in_seconds": 560580
                }
            },
            "agent_state": "idle",
            "vcs": {
                "type": "git",
                "branch": "main",
                "dirty": false
            },
            "sandbox": {
                "enabled": false
            },
            "artifact_count": 2,
            "plan_tier": "Pro",
            "email": "developer@email.com",
            "task_count": 1,
            "terminal_width": 111,
            "execution_mode": "planning"
        }"#;

        let input: InputData = serde_json::from_str(json).unwrap();
        assert_eq!(input.cwd.as_deref(), Some("/home/user/my-project"));
        assert_eq!(
            input.conversation_id.as_deref(),
            Some("12345678-abcd-ef01-2345-6789abcdef01")
        );
        assert_eq!(input.model.display_name, "Gemini 3.7 Flash (High)");
        assert_eq!(input.model.effort.as_deref(), Some("high"));
        assert_eq!(input.agent_state.as_deref(), Some("idle"));
        assert_eq!(input.task_count, Some(1));
        assert_eq!(input.artifact_count, Some(2));
        assert_eq!(input.plan_tier.as_deref(), Some("Pro"));
        assert_eq!(input.email.as_deref(), Some("developer@email.com"));
        assert_eq!(input.execution_mode.as_deref(), Some("planning"));
        assert_eq!(input.exceeds_200k_tokens, Some(false));

        let vcs = input.vcs.unwrap();
        assert_eq!(vcs.r#type.as_deref(), Some("git"));
        assert_eq!(vcs.branch.as_deref(), Some("main"));
        assert_eq!(vcs.dirty, Some(false));

        let quota = input.quota.unwrap();
        let weekly = quota.get("gemini-weekly").unwrap();
        assert_eq!(weekly.remaining_fraction, Some(0.9378));
        assert_eq!(weekly.reset_in_seconds, Some(560580));
    }
}
