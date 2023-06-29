use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The penaltyPrepKeeper struct represents a strategy that commands the keeper to set in the penalty formation
/// It is used when there is a penalty for the opponent team
#[derive(Default)]
pub struct penaltyPrepKeeper {
    /// The id of the robot to move.
    id: u8,
}

impl penaltyPrepKeeper {
    /// Creates a new penaltyPrepKeeper instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for penaltyPrepKeeper {
    /// Executes the penaltyPrepKeeper strategy.
    ///
    /// This strategy commands all the robots to move in position for
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
        action_wrapper.push(0, MoveTo::new(Point2::new(-0.25, 2.5), -PI / 4.0, 0.0,None));
        action_wrapper.push(1, MoveTo::new(Point2::new(-1.5, -1.5), -PI / 4.0, 0.0,None));
        action_wrapper.push(2, MoveTo::new(Point2::new(-0.25, -2.5), -PI / 4.0, 0.0,None));
        action_wrapper.push(3, MoveTo::new(Point2::new(-0.2, 0.0), -PI / 4.0, 0.0,None));
        action_wrapper.push(4, MoveTo::new(Point2::new(-1.5, 1.5), -PI / 4.0, 0.0,None));
        action_wrapper.push(5, MoveTo::new(Point2::new(-4.0, -0.0), -PI / 4.0, 0.0,None));
        true
    }
}

