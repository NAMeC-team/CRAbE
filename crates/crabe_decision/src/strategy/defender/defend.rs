use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The penaltyPrepKeeper struct represents a strategy that commands the keeper to set in the penalty formation
/// It is used when there is a penalty for the opponent team
#[derive(Default)]
pub struct Defend {
    /// The id of the robot to move.
    id: u8,
    left: bool,//defense the ball with 2 bot
}

impl Defend {
    /// Creates a new penaltyPrepKeeper instance with the desired robot id.
    pub fn new(id: u8, left: bool) -> Self {
        Self { id , left}
    }
}

impl Strategy for Defend {
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
        action_wrapper.clean(self.id);
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        let ball_pos = match world.ball.clone() {
            None => {
                action_wrapper.push(self.id, MoveTo::new(robot.pose.position, robot.pose.orientation, 0., None));
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let to_ball_angle = vectors::angle_to_point(ball_pos, robot.pose.position);
        let follow_ball = Point2::new(-3., ball_pos.y);
        action_wrapper.push(self.id, MoveTo::new(follow_ball, to_ball_angle, 0., None));
        false
    }
}

