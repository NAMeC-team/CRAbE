use clap::command;
use crabe_framework::data::IncomingDataset;
use crate::world::World;

pub struct Feedback;
pub struct ToolsData;
pub struct Commands;
pub struct ToolsCommands;

trait Receiver {
    fn step(&self, feedback: Feedback) -> IncomingDataset;
    fn close();
}

trait Filter {
    fn step(&self, data: IncomingDataset) -> World;
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