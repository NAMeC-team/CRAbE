use crabe_framework::data::{tool::ToolData, world::World};

use crate::action::ActionWrapper;

pub mod card_exception;

pub trait Exception {
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
        robots_handled: &mut Vec<u8>,
    );
}
