use crate::data::output::{Commands, Feedback};
use crate::data::receiver::InboundData;
use crate::data::tool::{ToolsCommands, ToolsData};
use crate::data::world::World;

pub trait Receiver {
    fn step(&mut self, feedback: &mut Feedback) -> InboundData;
    fn close(&mut self);
}

pub trait Filter {
    fn step(&mut self, data: InboundData) -> World;
    fn close(&mut self);
}

pub trait Decision {
    fn step(&mut self, data: &World) -> (Commands, ToolsData);
    fn close(&mut self);
}

pub trait Tools {
    fn step(&mut self, world_data: &World, tools_data: &mut ToolsData) -> ToolsCommands;
    fn close(&mut self);
}

pub trait Guard {
    fn step(
        &mut self,
        world_data: &mut World,
        command: &mut Commands,
        tools_commands: &mut ToolsCommands,
    );
    fn close(&mut self);
}

pub trait Output {
    fn step(&mut self, command: &mut Commands, tools_commands: &mut ToolsCommands) -> Feedback;
    fn close(&mut self);
}
