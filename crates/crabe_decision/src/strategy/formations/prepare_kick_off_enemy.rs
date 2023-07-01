use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The PrepareKickOffEnemy struct represents a strategy that commands the team to set in the PrepareKickOffEnemy formation
/// It is used when the team is not in favor of the kick-off
#[derive(Default)]
pub struct PrepareKickOffEnemy {
}

impl PrepareKickOffEnemy {
    /// Creates a new PrepareKickOffEne instance with the desired robot id.
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PrepareKickOffEnemy {
    /// Executes the PrepareKickOffEnemy strategy.
    ///
    /// This strategy commands all the robots to move in position for kick-off when not in favor
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
        action_wrapper.push(5, MoveTo::new(Point2::new(-1.5, 1.1), -PI / 4.0, 0.0,None , false, false));
        action_wrapper.push(1, MoveTo::new(Point2::new(-1.0, -0.0), -PI / 4.0, 0.0,None, false, false));
        action_wrapper.push(2, MoveTo::new(Point2::new(-1.5, -1.1), -PI / 4.0, 0.0,None, false, false));
        action_wrapper.push(3, MoveTo::new(Point2::new(-1.2, 0.7), -PI / 4.0, 0.0,None, false, false));
        action_wrapper.push(4, MoveTo::new(Point2::new(-1.2, -0.7), -PI / 4.0, 0.0,None, false, false));
        action_wrapper.push(0, MoveTo::new(Point2::new(-4.0, -0.0), -PI / 4.0, 0.0,None, false, false));
        true
    }
}

