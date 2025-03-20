use crate::action::move_to::MoveTo;
use crabe_framework::data::output::Kick;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, World};
use crabe_math::shape::Line;
use crabe_math::vectors::{self, angle_to_point, vector_from_angle};
use crabe_protocol::protobuf::game_controller_packet::Vector2;

pub fn intercept_instant_goal(
    world: &World,
    robot: &Robot<AllyInfo>,
    ball: &Ball,
) -> MoveTo {
    let reverse_ball_velocity = -ball.velocity.normalize().xy();
    let robot_to_goal = (world.geometry.enemy_goal.line.center() - robot.pose.position).normalize();
    // vector in the middle of the reverse ball velocity and the vector from the robot to the goal
    // let middle = (reverse_ball_velocity + robot_to_goal).normalize();
    let middle = world.geometry.enemy_goal.line.center() - ball.velocity.xy() / 2.;



    let ball_position = ball.position_2d();
    let orientation = vectors::angle_to_point(robot.pose.position,ball_position);
    let to_kicker_pos = vector_from_angle(robot.pose.orientation) * world.geometry.robot_radius;
    if ball.velocity.norm() < 0.4 {
        return MoveTo::new_all_params(ball_position, orientation, 0., false, None, true, true);
    }
    let trajectory = Line::new(ball_position, ball_position + ball.velocity.xy().normalize() * 100.);
    let target = trajectory.closest_point_on_segment(&robot.pose.position);
    MoveTo::new().set_target(target - to_kicker_pos).set_orientation(angle_to_point(robot.pose.position, middle)).set_kick(Kick::StraightKick { power: 4. })

}