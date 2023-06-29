use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, Robot, EnemyInfo, AllyInfo};
use crabe_math::shape::Line;
use crabe_math::vectors::vector_from_angle;
use nalgebra::{Point2, clamp};
use std::cmp::min;
use std::f64::consts::PI;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Keep {
    /// The id of the robot to move.
    id: u8,
}

impl Keep {
    /// Creates a new Keep instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id }
    }

    pub fn enemy_shooter(world: &World,) -> Option<Robot<AllyInfo>>{
        let mut closest_bot = None;
        let mut min_dist = f64::INFINITY;
        let ball_pos = match world.ball.clone() {
            None => {
                return None;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        for (_id, bot) in world.allies_bot.clone().into_iter(){
            let to_ball = ball_pos - bot.pose.position;
            let dist_to_ball = to_ball.norm();
            if dist_to_ball < min_dist{
                min_dist = dist_to_ball;
                closest_bot = Some(bot);
            }
        } 
        closest_bot
    }
}

impl Strategy for Keep {
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
            else if let Some(closest_enemy) = Keep::enemy_shooter(world){
                let enemy_dir = closest_enemy.pose.position + vector_from_angle(closest_enemy.pose.orientation) * 1000.;
                shoot_dir.end = enemy_dir.xy();
            }
            if let Some(intersection) = world.geometry.ally_goal.front_line.intersection_line(&shoot_dir) {
                let x = world.geometry.ally_goal.bottom_left_position.x+0.1;
                let y = clamp(intersection.y, world.geometry.ally_goal.bottom_left_position.y, world.geometry.ally_goal.bottom_right_position.y);
                action_wrapper.push(self.id, MoveTo::new(Point2::new(x, y), -PI / 4.0, 0., None));
            }
        }
        false
    }
}
