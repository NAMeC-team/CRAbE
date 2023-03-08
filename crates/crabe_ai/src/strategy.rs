use crate::action::ActionWrapper;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

pub mod square;

pub trait Strategy {
    fn step(
        &mut self,
        data: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool;
}
