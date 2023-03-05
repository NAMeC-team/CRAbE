use crate::config::CommonConfig;
use crate::data::output::{CommandMap, Feedback, FeedbackMap};
use crate::data::receiver::InboundData;
use crate::data::tool::{ToolCommands, ToolData};
use crate::data::world::World;

pub trait Component: ComponentBoxed {
    fn close(self);
}

pub trait ComponentBoxed {
    fn close_boxed(self: Box<Self>);
}

impl<T> ComponentBoxed for T
    where
        T: Component,
{
    fn close_boxed(self: Box<Self>) {
        (*self).close()
    }
}

impl<T> Component for Box<T>
    where
        T: ?Sized + Component,
{
    fn close(self) {
        self.close_boxed()
    }
}

// TODO: Document
pub trait InputComponent: Component {
    fn step(&mut self, feedback: &mut FeedbackMap) -> InboundData;
}

pub trait FilterComponent: Component {
    fn step(&mut self, data: InboundData, world: &mut World);
}

pub trait DecisionComponent: Component {
    fn step(&mut self, data: &World) -> (CommandMap, ToolData);
}

pub trait ToolComponent: Component {
    fn step(&mut self, world_data: &World, tools_data: &mut ToolData) -> ToolCommands;
}

pub trait GuardComponent: Component {
    fn step(
        &mut self,
        world_data: &mut World,
        command: &mut CommandMap,
        tools_commands: &mut ToolCommands,
    );
}

pub trait OutputComponent: Component {
    fn step(&mut self, commands: CommandMap, tool_commands: Option<ToolCommands>) -> FeedbackMap;
}
