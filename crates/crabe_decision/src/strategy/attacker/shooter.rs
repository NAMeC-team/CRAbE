use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::{Point2, Point3, Vector2};
use std::f64::consts::PI;
use std::ops::{Sub, Add, Mul};
use crabe_math::vectors;
use crabe_math::shape::Line;

#[derive(Default)]
pub struct Shooter {
    /// The id of the robot to move.
    id: u8,
    internal_state: ShooterState
}

#[derive(Debug, Default)]
enum ShooterState {
    #[default]
    GoingBehindBall,
    GoingShoot
}
impl Shooter {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, internal_state: ShooterState::GoingBehindBall }
    }
}

impl Strategy for Shooter {
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
        // action_wrapper.clean(self.id);
        // if let Some(ball) = &world.ball {
        //     let target = Point3::new(-world.geometry.field.length/2.,0.,0.);
        //     let mut dir = ball.position.sub(target);
        //     dir = dir.normalize();
        //     dir = dir * 0.2;
        //     match &self.internal_state {
        //         ShooterState::GoingBehindBall => {
        //             action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x + dir.x, ball.position.y + dir.y), PI / 4.0));
        //             self.internal_state = ShooterState::GoingShoot;
        //         },
        //         ShooterState::GoingShoot => {
        //             action_wrapper.push(self.id, MoveTo::new(Point2::new(ball.position.x, ball.position.y), PI / 4.0));
        //         },
        //     }
            
        // }
        // false

        action_wrapper.clean(self.id);
        let goal_pos: Point2<f64> = Point2::new(-world.geometry.field.length/2., 0.0);//[Warning] TODO:for testing we are kicking in our own goal
        let ball_pos = match world.ball.clone() {
            None => {
                return false;
            }
            Some(ball) => {
                ball.position.xy()
            }
        };
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        let robot_pos = robot.pose.position;
        let robot_to_ball = ball_pos - robot_pos;
        let dist_to_ball = robot_to_ball.norm();
        let mut dir_shooting_line = Line::new(robot_pos, robot_pos.add(robot_to_ball.mul(100.)));
        let robot_current_dir = vectors::vector_from_angle(robot.pose.orientation);
        let dot_with_ball = robot_current_dir.normalize().dot(&robot_to_ball.normalize());
        if dist_to_ball < 0.115 && dot_with_ball > 0.95{//TODO replace with IR (robot.has_ball)
            let kick = if dir_shooting_line.intersect(&world.geometry.ally_goal.front_line) {
                Some(Kick::ChipKick { power: 4. }) 
            }else {None};
            action_wrapper.push(self.id, MoveTo::new(robot_pos, vectors::angle_to_point(goal_pos, robot_pos), 1., kick));
        }else if dist_to_ball < 0.8 {
            action_wrapper.push(self.id, MoveTo::new(ball_pos, vectors::angle_to_point(ball_pos, robot_pos), 1., None));
        }else{
            action_wrapper.push(self.id, MoveTo::new(ball_pos, vectors::angle_to_point(ball_pos, robot_pos), 0., None));
        }

        false
    }
}
