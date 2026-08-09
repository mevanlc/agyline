use super::{Component, ComponentData};
use crate::config::types::{ComponentId, DEFAULT_HOSTNAME_RSTRIP};
use crate::core::input::InputData;
use std::collections::HashMap;

pub struct HostnameComponent {
    rstrip: Vec<String>,
}

impl Default for HostnameComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl HostnameComponent {
    pub fn new() -> Self {
        Self { rstrip: Vec::new() }.with_rstrip(DEFAULT_HOSTNAME_RSTRIP)
    }

    pub fn with_rstrip(mut self, value: &str) -> Self {
        self.rstrip = value
            .split(',')
            .map(str::trim)
            .filter(|suffix| !suffix.is_empty())
            .map(str::to_owned)
            .collect();
        self
    }

    fn system_hostname() -> Option<String> {
        let hostname = hostname::get().ok()?;
        let hostname = hostname.to_string_lossy().trim().to_owned();
        (!hostname.is_empty()).then_some(hostname)
    }

    fn display_hostname(&self, hostname: &str) -> Option<String> {
        let stripped = self
            .rstrip
            .iter()
            .find_map(|suffix| hostname.strip_suffix(suffix))
            .unwrap_or(hostname);
        (!stripped.is_empty()).then(|| stripped.to_owned())
    }
}

impl Component for HostnameComponent {
    fn collect(&self, _input: &InputData) -> Option<ComponentData> {
        let hostname = Self::system_hostname()?;
        Some(ComponentData {
            primary: self.display_hostname(&hostname)?,
            secondary: String::new(),
            metadata: HashMap::new(),
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Hostname
    }
}

#[cfg(test)]
mod tests {
    use super::HostnameComponent;
    use crate::config::types::ComponentId;
    use crate::core::components::Component;
    use crate::core::input::{InputData, Model, Workspace};

    fn input() -> InputData {
        InputData {
            model: Model {
                id: "test".into(),
                display_name: "Test".into(),
            },
            workspace: Workspace {
                current_dir: "/tmp".into(),
            },
            effort: None,
            thinking: None,
            context_window: None,
            rate_limits: None,
            cost: None,
            output_style: None,
        }
    }

    #[test]
    fn collects_the_system_hostname() {
        let component = HostnameComponent::new();
        let hostname = hostname::get().unwrap().to_string_lossy().trim().to_owned();
        let expected = component.display_hostname(&hostname).unwrap();
        let data = component.collect(&input()).unwrap();

        assert_eq!(data.primary, expected);
        assert!(data.secondary.is_empty());
        assert!(data.metadata.is_empty());
        assert_eq!(component.id(), ComponentId::Hostname);
    }

    #[test]
    fn strips_default_hostname_suffixes() {
        let component = HostnameComponent::new();

        assert_eq!(
            component.display_hostname("host.local").as_deref(),
            Some("host")
        );
        assert_eq!(
            component.display_hostname("host.localhost").as_deref(),
            Some("host")
        );
        assert_eq!(
            component.display_hostname("host.lan").as_deref(),
            Some("host")
        );
        assert_eq!(
            component.display_hostname("host.example").as_deref(),
            Some("host.example")
        );
    }

    #[test]
    fn parses_custom_comma_separated_suffixes() {
        let component = HostnameComponent::new().with_rstrip(" .corp, , .example ");

        assert_eq!(
            component.display_hostname("build.corp").as_deref(),
            Some("build")
        );
        assert_eq!(
            component.display_hostname("build.example").as_deref(),
            Some("build")
        );
        assert_eq!(
            component.display_hostname("build.local").as_deref(),
            Some("build.local")
        );
    }

    #[test]
    fn empty_rstrip_keeps_the_full_hostname() {
        let component = HostnameComponent::new().with_rstrip("");

        assert_eq!(
            component.display_hostname("host.local").as_deref(),
            Some("host.local")
        );
    }
}
