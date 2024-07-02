use crabe_math::shape::Line;
use nalgebra::Point2;
use crabe_framework::data::world::{Robot, World};

/// Get the closest bot to a point.
/// 
/// # Arguments
/// bots - The map of robots.
/// point - The point to which the distance is calculated.
/// 
/// # Returns
/// The closest robot to the point.
/// 
/// # Example
/// On use case, to grab a list of bots you can do for example :
/// world.allies_bot.values().collect()
pub fn closest_bot_to_point<T>(bots: Vec<&Robot<T>>, point: Point2<f64>) -> Option<&Robot<T>> {
    bots.into_iter()
        .min_by(|robot1, robot2| {
            robot1.distance(&point).total_cmp(&robot2.distance(&point))
        })
}

/// Get a list of the closest to the farthest bots to a point
/// 
/// # Arguments
/// bots - The map of robots.
/// point - The point to which the distance is calculated.
/// 
/// # Returns
/// A list of the closest to the farthest robots to the point.
/// 
/// 
/// # Example
/// On use case, to grab a list of bots you can do for example :
/// world.allies_bot.values().collect()
pub fn closest_bots_to_point<T>(bots: Vec<&Robot<T>>, point: Point2<f64>) -> Vec<&Robot<T>> {
    let mut robots_with_distances: Vec<_> = bots
        .into_iter()
        .map(|robot| {
            let distance = robot.distance(&point);
            (distance, robot)
        })
        .collect();

    robots_with_distances.sort_by(|(d1, _), (d2, _)| d1.total_cmp(d2));

    robots_with_distances.into_iter().map(|(_, robot)| robot).collect()
}

/// Check if any bot is in the trajectory (straight line) of a robot to a target point.
/// 
/// # Arguments
/// world - The world data.
/// id - The id of the robot to check the trajectory.
/// target - The target point.
/// 
/// # Returns
/// True if any robot is in the trajectory, false otherwise.
pub fn bot_in_trajectory(world: &World, id: u8, target: Point2<f64>) -> bool{
    let robot = match world.allies_bot.get(&id) {
        None => {
            return false;
        }
        Some(robot) => {
            robot
        }
    };
    let trajectory = Line::new(robot.pose.position, target);
    let closest_dist = world.allies_bot
        .iter().filter(|(current_id, _)| **current_id != id)
        .map(|(id, robot)| (id, trajectory.distance_to_point(&robot.pose.position.xy())))
        .chain(world.enemies_bot.iter().map(|(id, robot)| (id, trajectory.distance_to_point(&robot.pose.position.xy()))))
        .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2))
        .map(|(_, d)| d);
    let delta = 0.005;
    return closest_dist < Some(world.geometry.robot_radius * 2. + delta)
}

/// Check if the ball is in the trajectory (straight line) of a robot to a target point.
///    
/// # Arguments
/// world - The world data.
/// id - The id of the robot to check the trajectory.
/// target - The target point.
/// 
/// # Returns
/// True if the ball is in the trajectory, false otherwise.
pub fn ball_in_trajectory(world: &World, id: u8, target: Point2<f64>) -> bool{
    let robot = match world.allies_bot.get(&id) {
        None => {
            return false;
        }
        Some(robot) => {
            robot
        }
    };
    let trajectory = Line::new(robot.pose.position, target);    
    if let Some(ball) = &world.ball{
        let delta = 0.001;
        return trajectory.distance_to_point(&ball.position_2d()) < world.geometry.robot_radius + world.geometry.ball_radius + delta;
    }
    false
}