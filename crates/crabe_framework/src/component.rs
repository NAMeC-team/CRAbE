use crate::data::output::{Commands, Feedback, ToolsCommands};
use crate::data::receiver::InboundData;
use crate::data::tool::ToolsData;
use crate::data::world::World;

pub trait Receiver {
    fn step(&self, feedback: Feedback) -> InboundData;
    fn close();
}

pub trait Filter {
    fn step(&self, data: InboundData) -> World;
    fn close();
}

pub trait Decision {
    fn step(&self, data: &World) -> (Commands, ToolsData);
}

pub trait Tools {
    fn step(&self, world_data: &World, tools_data: &mut ToolsData) -> ToolsCommands;
    fn close();
}

pub trait Guard {
    fn step(&self, world_data: &mut World, command: &mut Commands, tools_commands: &mut ToolsCommands);
    fn close();
}

pub trait Output {
    fn step(&self, command: &mut Commands, tools_commands: &mut ToolsCommands) -> Feedback;
    fn close();
}