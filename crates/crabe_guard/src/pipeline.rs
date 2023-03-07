use clap::Args;
use crabe_framework::component::{Component, GuardComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap};
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct GuardConfig {}

pub trait Guard {
    fn guard(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    );
}

pub struct GuardPipeline {
    guards: Vec<Box<dyn Guard>>,
}

impl GuardPipeline {
    pub fn with_config(input_cfg: GuardConfig, common_cfg: &CommonConfig) -> Self {
        Self { guards: vec![] }
    }
}

impl Component for GuardPipeline {
    fn close(mut self) {}
}

impl GuardComponent for GuardPipeline {
    fn step(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    ) {
        self.guards
            .iter_mut()
            .for_each(|x| x.guard(world, commands, tools_commands));
    }
}
