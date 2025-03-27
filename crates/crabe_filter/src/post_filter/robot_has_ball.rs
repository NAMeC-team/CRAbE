use crate::data::FilterData;
use crabe_framework::data::world::{Ball, Robot, World};
use crabe_math::vectors::vector_from_angle;

use crate::post_filter::PostFilter;

pub struct RobotHasBallFilter;


fn check_robot_has_ball<T>(robot: &Robot<T>, ball: &Ball, robot_ball_radius_sum: f64) -> bool {
    let distance = robot.distance(&ball.position_2d()) <= robot_ball_radius_sum;
    let forward_vector = vector_from_angle(robot.pose.orientation);
    let robot_to_ball = ball.position_2d() - robot.pose.position;
    let facing_ball = forward_vector.normalize().dot(&robot_to_ball.normalize());
    distance && facing_ball > 0.9
}


impl PostFilter for RobotHasBallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let robot_ball_radius_sum = world.geometry.robot_radius + world.geometry.ball_radius;
        if let Some(ball) = &world.ball {
            for (robot_id, _) in filter_data.enemies.iter() {
                if let Some(robot) = world.enemies_bot.get_mut(robot_id) {
                    robot.has_ball = check_robot_has_ball(robot, ball, robot_ball_radius_sum);
                }
            }
            for (robot_id, _) in filter_data.allies.iter() {
                if let Some(robot) = world.allies_bot.get_mut(robot_id) {
                    robot.has_ball = check_robot_has_ball(robot, ball, robot_ball_radius_sum);
                }
            }
        }else{
            for (robot_id, _) in filter_data.enemies.iter() {
                if let Some(robot) = world.enemies_bot.get_mut(robot_id) {
                    robot.has_ball = false;
                }
            }
        }
    }
}