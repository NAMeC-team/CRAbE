use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

#[derive(Default)]
pub struct Square {
    id: u8,
}

impl Square {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Square {
    fn step(
        &mut self,
        _data: &World,
        _tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, 1.0), -PI / 4.0));
        action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, 1.0), -3.0 * PI / 4.0));
        action_wrapper.push(self.id, MoveTo::new(Point2::new(1.0, -1.0), 3.0 * PI / 4.0));
        action_wrapper.push(self.id, MoveTo::new(Point2::new(-1.0, -1.0), PI / 4.0));
        true
    }
}
