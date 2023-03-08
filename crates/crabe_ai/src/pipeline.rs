use crate::action::ActionWrapper;
use crate::manager::manual::Manual;
use crate::manager::Manager;
use clap::Args;
use crabe_framework::component::{Component, DecisionComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct DecisionConfig {}

pub struct DecisionPipeline {
    action_wrapper: ActionWrapper,
    manager: Box<dyn Manager>,
}

impl DecisionPipeline {
    pub fn with_config(_decision_cfg: DecisionConfig, _common_cfg: &CommonConfig) -> Self {
        Self {
            action_wrapper: ActionWrapper::default(),
            manager: Box::new(Manual::new()),
        }
    }
}

impl Component for DecisionPipeline {
    fn close(self) {}
}

impl DecisionComponent for DecisionPipeline {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData) {
        // TODO : Don't create ToolsData here
        let mut tool_data = ToolData::default();
        self.manager
            .step(data, &mut tool_data, &mut self.action_wrapper);

        (self.action_wrapper.compute(data, &mut tool_data), tool_data)
    }
}
