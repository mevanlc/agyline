pub mod context_window;
pub mod cost;
pub mod directory;
pub mod git;
pub mod hostname;
pub mod model;
pub mod output_style;
pub mod pull_request;
pub mod session;
pub mod usage;

use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

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

pub use context_window::ContextWindowComponent;
pub use cost::CostComponent;
pub use directory::DirectoryComponent;
pub use git::GitComponent;
pub use hostname::HostnameComponent;
pub use model::ModelComponent;
pub use output_style::OutputStyleComponent;
pub use pull_request::PullRequestComponent;
pub use session::SessionComponent;
pub use usage::{FiveHourUsageComponent, SevenDayUsageComponent};
