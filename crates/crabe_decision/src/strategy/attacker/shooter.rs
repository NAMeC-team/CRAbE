use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Point2};
use std::ops::{Add, Mul};
use crabe_math::vectors::{self, vector_from_angle};
use crabe_math::shape::Line;

pub struct Shooter {
    /// The id of the robot to move.
    id: u8,
    state: ShooterState
}
impl Shooter {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, state:ShooterState::PlaceForShoot}
    }
}
enum ShooterState{
    PlaceForShoot,
    Shoot
}
impl Strategy for Shooter {
    /// Executes the Shooter strategy.
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
        let goal_pos: Point2<f64> = world.geometry.enemy_goal.center_front_position;
        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let robot_pos = robot.pose.position;
        let robot_to_ball = ball_pos - robot_pos;
        let dir_shooting_line: Line = Line::new(robot_pos, robot_pos.add(vector_from_angle(robot.pose.orientation).mul(100.)));
        let dir_shooting_line_ball: Line = Line::new(robot_pos, robot_pos.add((ball_pos - robot_pos).mul(100.)));
        let ball_to_goal = goal_pos - ball_pos;
        let behind_ball_pos = ball_pos + ball_to_goal.normalize() * -0.3;
        let ball_avoidance: bool = robot_to_ball.normalize().dot(&(goal_pos-ball_pos).normalize()) < 0.;
        let aligne_with_goal_target: bool = dir_shooting_line.intersect(&world.geometry.enemy_goal.front_line);
        let aligne_to_shoot: bool = dir_shooting_line_ball.intersect(&world.geometry.enemy_goal.front_line);
        let robot_current_dir = vectors::vector_from_angle(robot.pose.orientation);
        let dot_with_ball = robot_current_dir.normalize().dot(&robot_to_ball.normalize());
        let aligne_oriented_to_opponent_side = dir_shooting_line.intersection(&world.geometry.enemy_goal.front_line).is_some();
        match self.state {
            ShooterState::PlaceForShoot => {
                if ((aligne_to_shoot && aligne_with_goal_target) || 
                    (robot_pos.x < 0. && aligne_oriented_to_opponent_side)) && 
                    (((behind_ball_pos - robot_pos).norm() <= 0.1 || dot_with_ball > 0.93)){
                    self.state = ShooterState::Shoot
                }
                action_wrapper.push(self.id, MoveTo::new(behind_ball_pos, vectors::angle_to_point(goal_pos, robot_pos), 0., None, ball_avoidance, false));
            },
            ShooterState::Shoot => {
                let dist_to_ball = robot_to_ball.norm();
                let kick: Option<Kick> = if dist_to_ball < 0.125 && dot_with_ball > 0.88 && 
                    (aligne_to_shoot || aligne_oriented_to_opponent_side) {
                    Some(Kick::StraightKick {  power: 4. }) 
                }else {None};
                action_wrapper.push(self.id, MoveTo::new(ball_pos + (ball_pos - robot_pos), vectors::angle_to_point(goal_pos, robot_pos), 1.,  kick, false, true));
                if ball_avoidance || dist_to_ball > 0.4{
                    self.state = ShooterState::PlaceForShoot;
                }
            }
        };
        false
    }
    fn name(&self) -> &'static str {
        return "Shooter";
    }
}

