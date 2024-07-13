use crate::action::move_to::MoveTo;
use crabe_framework::data::output::Kick;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, World};
use crabe_math::shape::Line;
use crabe_math::vectors;

const GO_BEHIND_BALL_DIST: f64 = 0.3;

/// Pass the ball to the receiver
/// (before kicking he makes sure to be aligned with the receiver)
/// 
/// # Arguments
/// - `robot` : The robot that will pass the ball
/// - `receiver` : The robot that will receive the ball
/// - `ball` : The ball
/// - `world` : The current world state
/// 
/// # Returns
/// A `MoveTo` action that will make the robot pass the ball to the receiver
pub fn pass(
    robot: &Robot<AllyInfo>,
    receiver: &Robot<AllyInfo>,
    ball: &Ball,
    world: &World,
) -> MoveTo {
    let robot_position = robot.pose.position;
    let robot_direction = vectors::vector_from_angle(robot.pose.orientation);
    let ball_position = ball.position_2d();
    let robot_to_ball = ball_position - robot_position;
    let dot_with_ball = robot_direction.normalize().dot(&robot_to_ball.normalize());
    let dist_to_ball: f64 = robot_to_ball.norm();

    // Calculate the position behind the ball to prepare the pass
    let behind_ball_position = ball_position + (ball_position - receiver.pose.position).normalize() * GO_BEHIND_BALL_DIST; 
            
    // Check if the pass trajectory will arrive near the ally
    let robot_passing_trajectory = Line::new(robot_position, robot_position + robot_to_ball * 100.);
    let passing_trajectory_will_land = match robot_passing_trajectory.orthogonal_projection_point_on_segment(&receiver.pose.position) {
        Ok(closest_point) => {
            let dist_to_receiver = (receiver.pose.position - closest_point).norm();
            dist_to_receiver < 0.3
        },
        Err(_) => false,
    };

    if passing_trajectory_will_land && dot_with_ball > 0.95{
        let kick: Option<Kick> = if dist_to_ball < (world.geometry.robot_radius + world.geometry.ball_radius + 0.002) { 
            Some(Kick::StraightKick {  power: 4. }) 
        }else {None};
        return MoveTo::new(ball_position, vectors::angle_to_point(robot_position,receiver.pose.position), 1.,  true, kick, true);
    }
    MoveTo::new(behind_ball_position, vectors::angle_to_point(robot_position, receiver.pose.position), 0., false, None, true)
}