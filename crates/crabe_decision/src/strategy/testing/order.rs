use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;
use crabe_framework::data::output::Command;
use crate::action::order_raw::RawOrder;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Order {
    /// The id of the robot to move.
    id: u8,
}

impl Order {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for Order {
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        let cmd = Command {
            forward_velocity: 1.0,
            left_velocity: 0.0,
            angular_velocity: 0.0,
            charge: false,
            kick: None,
            dribbler: 0.0,
        };
        action_wrapper.push(self.id, RawOrder::new(cmd));
        false
    }
}
