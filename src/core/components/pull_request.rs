use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;

#[derive(Default)]
pub struct PullRequestComponent {
    show_review_state: bool,
    show_url: bool,
    osc_hyperlinks: bool,
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

    fn field(&self, url: &str, text: impl Into<String>) -> String {
        let text = text.into();
        if self.osc_hyperlinks && Self::safe_hyperlink_url(url) {
            format!("\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\")
        } else {
            text
        }
    }

    fn safe_hyperlink_url(url: &str) -> bool {
        !url.is_empty() && !url.chars().any(char::is_control)
    }
}

impl Component for PullRequestComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let pr = input.pr.as_ref()?;
        let primary = self.field(&pr.url, format!("#{}", pr.number));
        let mut secondary_fields = Vec::new();

        if self.show_review_state
            && let Some(review_state) = pr.review_state.as_deref()
            && !review_state.is_empty()
        {
            secondary_fields.push(self.field(&pr.url, review_state));
        }
        if self.show_url && !pr.url.is_empty() {
            secondary_fields.push(self.field(&pr.url, &pr.url));
        }

        let mut metadata = HashMap::from([
            ("number".into(), pr.number.to_string()),
            ("url".into(), pr.url.clone()),
        ]);
        if let Some(review_state) = &pr.review_state {
            metadata.insert("review_state".into(), review_state.clone());
        }

        Some(ComponentData {
            primary,
            secondary: secondary_fields.join(" "),
            metadata,
        })
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

    const URL: &str = "https://github.com/example/repo/pull/482";

    fn input(review_state: Option<&str>) -> InputData {
        serde_json::from_value(serde_json::json!({
            "model": {"id": "claude-sonnet-4-5", "display_name": "Sonnet 4.5"},
            "workspace": {"current_dir": "/tmp/project"},
            "pr": {
                "number": 482,
                "url": URL,
                "review_state": review_state,
            },
        }))
        .unwrap()
    }

    fn link(text: &str) -> String {
        format!("\x1b]8;;{URL}\x1b\\{text}\x1b]8;;\x1b\\")
    }

    #[test]
    fn hyperlinks_each_visible_field_separately() {
        let data = PullRequestComponent::new()
            .with_review_state(true)
            .with_url(true)
            .with_osc_hyperlinks(true)
            .collect(&input(Some("approved")))
            .unwrap();

        assert_eq!(data.primary, link("#482"));
        assert_eq!(
            data.secondary,
            format!("{} {}", link("approved"), link(URL))
        );
    }

    #[test]
    fn hides_optional_fields_independently() {
        let review_only = PullRequestComponent::new()
            .with_review_state(true)
            .collect(&input(Some("approved")))
            .unwrap();
        assert_eq!(review_only.primary, "#482");
        assert_eq!(review_only.secondary, "approved");

        let url_only = PullRequestComponent::new()
            .with_url(true)
            .collect(&input(Some("approved")))
            .unwrap();
        assert_eq!(url_only.primary, "#482");
        assert_eq!(url_only.secondary, URL);
    }

    #[test]
    fn osc_hyperlinks_off_leaves_every_visible_field_plain() {
        let data = PullRequestComponent::new()
            .with_review_state(true)
            .with_url(true)
            .with_osc_hyperlinks(false)
            .collect(&input(Some("approved")))
            .unwrap();

        assert_eq!(data.primary, "#482");
        assert_eq!(data.secondary, format!("approved {URL}"));
        assert!(!data.primary.contains("\x1b]8;;"));
        assert!(!data.secondary.contains("\x1b]8;;"));
    }

    #[test]
    fn missing_review_state_still_shows_number_and_url() {
        let data = PullRequestComponent::new()
            .with_review_state(true)
            .with_url(true)
            .collect(&input(None))
            .unwrap();

        assert_eq!(data.primary, "#482");
        assert_eq!(data.secondary, URL);
    }
}
