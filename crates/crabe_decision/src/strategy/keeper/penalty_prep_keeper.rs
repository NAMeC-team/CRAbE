use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::shape::Line;
use crabe_math::vectors::vector_from_angle;
use nalgebra::{Point2, clamp};
use std::f64::consts::PI;

/// The penaltyPrepKeeper struct represents a strategy that commands the keeper to set in the penalty formation
/// It is used when there is a penalty for the opponent team
#[derive(Default)]
pub struct PenaltyPrepKeeper {
    /// The id of the robot to move.
    id: u8,
}

impl PenaltyPrepKeeper {
    /// Creates a new penaltyPrepKeeper instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Strategy for PenaltyPrepKeeper {
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
        if let Some(ball) = &world.ball{
            let mut shoot_dir = Line::new(ball.position_2d(),Point2::new(-10.,ball.position.y));
            dbg!(ball.velocity);
            if ball.velocity.norm() > 0.{//TODO : ball velocity is equal to 0
                let ball_dir = ball.position + ball.velocity * 1000.;
                shoot_dir.end = ball_dir.xy();
            }
            else if let Some(closest_enemy) = GameManager::closest_enemy_to_ball(world){
                let enemy_dir = closest_enemy.pose.position + vector_from_angle(closest_enemy.pose.orientation) * 1000.;
                shoot_dir.end = enemy_dir.xy();
            }
            if let Some(intersection) = world.geometry.ally_goal.front_line.intersection_line(&shoot_dir) {
                let x = world.geometry.ally_goal.bottom_left_position.x+0.1;
                let y = clamp(intersection.y, world.geometry.ally_goal.bottom_left_position.y, world.geometry.ally_goal.bottom_right_position.y);
                action_wrapper.push(self.id, MoveTo::new(Point2::new(x, y), -PI / 4.0, 0., None));
            }
        }
        true
    }
}

