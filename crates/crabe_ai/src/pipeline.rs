use crate::action::ActionWrapper;
use clap::Args;
use crabe_framework::component::{Component, DecisionComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap};
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct DecisionConfig {}

pub struct DecisionPipeline {
    action_wrapper: ActionWrapper,
}

impl DecisionPipeline {
    pub fn with_config(input_cfg: DecisionConfig, common_cfg: &CommonConfig) -> Self {
        Self {
            action_wrapper: ActionWrapper::default(),
        }
    }
}

impl Component for DecisionPipeline {
    fn close(mut self) {}
}

impl DecisionComponent for DecisionPipeline {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData) {
        let mut command_map = CommandMap::new();
        command_map.insert(
            0,
            Command {
                angular_velocity: 1.0,
                ..Default::default()
            },
        );

        (command_map, ToolData {})
    }
}
