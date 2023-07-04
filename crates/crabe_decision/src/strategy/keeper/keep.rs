use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::manager::game_manager::GameManager;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, Robot, EnemyInfo, AllyInfo};
use crabe_math::shape::Line;
use crabe_math::vectors::{vector_from_angle, self};
use nalgebra::{Point2, clamp};
use std::cmp::min;
use std::f64::consts::PI;
use std::ops::{Add, Mul};

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
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        if let Some(ball) = &world.ball{
            let ball_pos = ball.position_2d();
            let mut shoot_dir = Line::new(ball_pos,Point2::new(-10.,ball.position.y));
            if (robot.pose.position - ball_pos).norm() < 0.4{
                let goal_pos: Point2<f64> = Point2::new(world.geometry.field.length/2., 0.0);
                let robot_pos = robot.pose.position;
                let robot_to_ball = ball_pos - robot_pos;
                let dist_to_ball = robot_to_ball.norm();
                let dir_shooting_line = Line::new(robot_pos, robot_pos.add(vector_from_angle(robot.pose.orientation).mul(100.)));
                let robot_current_dir = vectors::vector_from_angle(robot.pose.orientation);
                let dot_with_ball = robot_current_dir.normalize().dot(&robot_to_ball.normalize());
                if (dist_to_ball < 0.115 && dot_with_ball > 0.9) || robot.has_ball{//TODO replace with IR (robot.has_ball)
                    let kick: Option<Kick> = if dir_shooting_line.intersect(&world.geometry.enemy_goal.front_line) {
                        Some(Kick::StraightKick {  power: 4. }) 
                    }else {None};
                    action_wrapper.push(self.id, MoveTo::new(robot_pos, vectors::angle_to_point(goal_pos, robot_pos), 1., kick, false, true));
                }else if dist_to_ball < 0.8 {
                    action_wrapper.push(self.id, MoveTo::new(ball_pos, vectors::angle_to_point(ball_pos, robot_pos), 1.,  None, false, true));
                }else{
                    action_wrapper.push(self.id, MoveTo::new(ball_pos, vectors::angle_to_point(ball_pos, robot_pos), 0.,  None, false, false));
                };
            }else{
                if ball.velocity.norm() > 0.{
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
                    action_wrapper.push(self.id, MoveTo::new(Point2::new(x, y), vectors::angle_to_point(ball.position.xy(), robot.pose.position ), 0., None, false, false));
                }
            }
        }
        false
    }
}
