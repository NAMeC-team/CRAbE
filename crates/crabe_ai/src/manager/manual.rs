use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::square::Square;
use crate::strategy::Strategy;
use crabe_framework::data::output::{Command, CommandMap};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

#[derive(Default)]
pub struct Manual {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Manual {
    fn new() -> Self {
        Self {
            strategies: vec![Box::new(Square::default())],
        }
    }
}

impl Manager for Manual {
    fn step(
        &mut self,
        data: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        self.strategies.retain_mut(|s| !s.step(data, tools_data, action_wrapper));
    }
}
