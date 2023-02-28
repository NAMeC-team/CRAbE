use crate::data::output::{Command, CommandMap, Feedback, FeedbackMap};
use crate::data::receiver::InboundData;
use crate::data::tool::{ToolsCommands, ToolsData};
use crate::data::world::World;

pub trait InputComponent {
    fn step(&mut self, feedback: &mut FeedbackMap) -> InboundData;
    fn close(&mut self);
}

pub trait FilterComponent {
    fn step(&mut self, data: InboundData, world: &mut World); // TODO: Remove this
    fn close(&mut self);
}

pub trait DecisionComponent {
    fn step(&mut self, data: &World) -> (CommandMap, ToolsData);
    fn close(&mut self);
}

pub trait ToolsComponent {
    fn step(&mut self, world_data: &World, tools_data: &mut ToolsData) -> ToolsCommands;
    fn close(&mut self);
}

pub trait GuardComponent {
    fn step(
        &mut self,
        world_data: &mut World,
        command: &mut CommandMap,
        tools_commands: &mut ToolsCommands,
    );
    fn close(&mut self);
}

pub trait OutputComponent {
    fn step(&mut self, command: &mut CommandMap, tools_commands: &mut ToolsCommands) -> Feedback;
    fn close(&mut self);
}
