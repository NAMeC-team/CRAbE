use crate::data::output::{Commands, Feedback, ToolsCommands};
pub use crate::data::receiver::InboundData;
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

trait Decision {
    fn step(&self, data: &World) -> (Commands, ToolsData);
}

trait Tools {
    fn step(&self, world_data: &World, tools_data: &mut ToolsData) -> ToolsCommands;
    fn close();
}

trait Guard {
    fn step(&self, world_data: &mut World, command: &mut Commands, tools_commands: &mut ToolsCommands);
    fn close();
}

trait Output {
    fn step(&self, command: &mut Commands, tools_commands: &mut ToolsCommands) -> Feedback;
    fn close();
}