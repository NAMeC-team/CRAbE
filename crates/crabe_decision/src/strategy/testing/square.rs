use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Square {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
}

impl Square {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for Square {
    fn name(&self) -> &'static str {
        "Square"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        vec![self.id]
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        if ids.len() == 1{
            self.id = ids[0];
        }
    }
    /// Executes the Square strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a square shape in a
    /// counter-clockwise direction.
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.push(
            self.id,
            MoveTo::new(Point2::new(-1.0, 1.0), -PI / 4., 0.0, false, None, true),
        );
        action_wrapper.push(
            self.id,
            MoveTo::new(Point2::new(1.0, 1.0), -3.* PI / 4., 0.0, false, None, true),
        );
        action_wrapper.push(
            self.id,
            MoveTo::new(Point2::new(1.0, -1.0), 3.* PI / 4., 0.0, false, None, true),
        );
        action_wrapper.push(
            self.id,
            MoveTo::new(Point2::new(-1.0, -1.0), PI / 4., 0.0, false, None, true),
        );
        true
    }
}
