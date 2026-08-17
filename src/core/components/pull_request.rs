use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;

#[derive(Default)]
pub struct PullRequestComponent {
    pub show_review_state: bool,
    pub show_url: bool,
    pub osc_hyperlinks: bool,
}

impl PullRequestComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_review_state(mut self, show_review_state: bool) -> Self {
        self.show_review_state = show_review_state;
        self
    }

    pub fn with_url(mut self, show_url: bool) -> Self {
        self.show_url = show_url;
        self
    }

    pub fn with_osc_hyperlinks(mut self, osc_hyperlinks: bool) -> Self {
        self.osc_hyperlinks = osc_hyperlinks;
        self
    }
}

impl Component for PullRequestComponent {
    fn collect(&self, _input: &InputData) -> Option<ComponentData> {
        None
    }

    fn id(&self) -> ComponentId {
        ComponentId::PullRequest
    }
}

#[cfg(test)]
mod tests {
    use super::PullRequestComponent;
    use crate::core::components::Component;
    use crate::core::input::InputData;

    #[test]
    fn collect_returns_none_without_pr_payload() {
        let input = InputData::default();
        assert!(PullRequestComponent::new().collect(&input).is_none());
    }
}
