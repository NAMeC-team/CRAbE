use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::constants::{PIVOT_ID, ATTACKER1_ID, ATTACKER2_ID};
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Point2};
use std::ops::{Add, Mul};
use crabe_math::vectors::{self, vector_from_angle};
use crabe_math::shape::Line;

#[derive(Default)]
pub struct Waiter {
    /// The id of the robot to move.
    id: u8
}
impl Waiter {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id}
    }
}

impl Strategy for Waiter {
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
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };     
        let mut y = 0.;
        if dbg!(self.id) == ATTACKER1_ID{
            y = -2.;
        }else if self.id == ATTACKER2_ID{
            y = 2.;
        }
        let x = ball_pos.x;
        action_wrapper.push(self.id, MoveTo::new(Point2::new(x,y), vectors::angle_to_point(ball_pos, robot.pose.position), 0., None, false, true));
        false
    }

    fn name(&self) -> &'static str{
        return "Passer";
    }
}
