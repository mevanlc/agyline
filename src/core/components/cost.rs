use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;

#[derive(Default)]
pub struct CostComponent;

impl CostComponent {
    pub fn new() -> Self {
        Self
    }
}

impl Component for CostComponent {
    fn collect(&self, _input: &InputData) -> Option<ComponentData> {
        None
    }

    fn id(&self) -> ComponentId {
        ComponentId::Cost
    }
}
