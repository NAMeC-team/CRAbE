use crate::data::output::{CommandMap, Feedback, FeedbackMap};
use crate::data::receiver::InboundData;
use crate::data::tool::{ToolCommands, ToolData};
use crate::data::world::World;

// TODO: Document
pub trait InputComponent {
    fn step(&mut self, feedback: &mut FeedbackMap) -> InboundData;
    fn close(&mut self);
}

pub trait FilterComponent {
    fn step(&mut self, data: InboundData, world: &mut World);
    fn close(&mut self);
}

pub trait DecisionComponent {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData);
    fn close(&mut self);
}

pub trait ToolComponent {
    fn step(&mut self, world_data: &World, tools_data: &mut ToolData) -> ToolCommands;
    fn close(&mut self);
}

pub trait GuardComponent {
    fn step(
        &mut self,
        world_data: &mut World,
        command: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    );
    fn close(&mut self);
}

pub trait OutputComponent {
    fn step(&mut self, command: &mut CommandMap, tools_commands: &mut ToolCommands) -> Feedback;
    fn close(&mut self);
}
