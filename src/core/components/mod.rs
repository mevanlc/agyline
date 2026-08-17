pub mod agent_state;
pub mod artifact_count;
pub mod context_window;
pub mod cost;
pub mod directory;
pub mod email;
pub mod execution_mode;
pub mod git;
pub mod hostname;
pub mod model;
pub mod output_style;
pub mod pending_input;
pub mod plan_tier;
pub mod pull_request;
pub mod quota;
pub mod sandbox;
pub mod session;
pub mod task_count;
pub mod tool_confirmation;
pub mod usage;
pub mod vim_mode;
pub mod worktree;

use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

pub const METADATA_DISPLAYED_BRANCH: &str = "displayed_branch";

/// A component that collects data from [`InputData`] for display in the status line.
///
/// Each component extracts relevant fields from the input and returns them as
/// [`ComponentData`]. Returning `None` signals that the component has nothing to
/// display for this invocation (e.g. a field is absent or the value is not
/// meaningful), and the component will be omitted from the rendered output.
pub trait Component {
    /// Extract display data from `input`. Returns `None` to suppress the component.
    fn collect(&self, input: &InputData) -> Option<ComponentData>;

    /// The stable identifier for this component type.
    fn id(&self) -> ComponentId;
}

#[derive(Debug, Clone)]
pub struct ComponentData {
    pub primary: String,
    pub secondary: String,
    pub metadata: HashMap<String, String>,
}

pub use agent_state::AgentStateComponent;
pub use artifact_count::ArtifactCountComponent;
pub use context_window::ContextWindowComponent;
pub use cost::CostComponent;
pub use directory::DirectoryComponent;
pub use email::EmailComponent;
pub use execution_mode::ExecutionModeComponent;
pub use git::GitComponent;
pub use hostname::HostnameComponent;
pub use model::ModelComponent;
pub use output_style::OutputStyleComponent;
pub use pending_input::PendingInputComponent;
pub use plan_tier::PlanTierComponent;
pub use pull_request::PullRequestComponent;
pub use quota::QuotaComponent;
pub use sandbox::SandboxComponent;
pub use session::SessionComponent;
pub use task_count::TaskCountComponent;
pub use tool_confirmation::ToolConfirmationComponent;
pub use usage::{FiveHourUsageComponent, SevenDayUsageComponent};
pub use vim_mode::VimModeComponent;
pub use worktree::WorktreeComponent;
