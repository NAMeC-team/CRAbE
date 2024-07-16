use crabe_math::shape::{Circle, Line};
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

pub fn robots_to_circles<T>(world: &World, robots: Vec<&Robot<T>>) -> Vec<Circle> {
    robots
        .into_iter()
        .map(|robot| Circle::new(robot.pose.position, world.geometry.robot_radius))
        .collect()
}

/// Return the closest object (ball or robot) on the trajectory (straight line) of a robot to a target point if there is one.
/// 
/// # Arguments
/// world - The world data.
/// id - The id of the robot to check the trajectory.
/// target - The target point.
/// 
/// # Returns
/// The object in the trajectory if there is one, None otherwise.
pub fn object_in_bot_trajectory(world: &World, id: u8, target: Point2<f64>, ball: bool, ally: bool, enemies: bool) -> Vec<Circle>{
    let robot = match world.allies_bot.get(&id) {
        None => {
            return vec![];
        }
        Some(robot) => {
            robot
        }
    };
    let trajectory = Line::new(robot.pose.position, target);
    let mut objects = vec![];
    if ball{
        if let Some(ball) = &world.ball{
            objects.push(Circle::new(ball.position_2d(), world.geometry.ball_radius));
        }
    }
    if ally{
        for (_, robot) in world.allies_bot.iter(){
            if robot.id == id{
                continue;
            }
            objects.push(Circle::new(robot.pose.position, world.geometry.robot_radius));
        }
    }
    if enemies{
        for (_, robot) in world.enemies_bot.iter(){
            objects.push(Circle::new(robot.pose.position, world.geometry.robot_radius));
        }
    }
    let mut objects_in_trajectory = vec![];
    for object in objects{
        if trajectory.distance_to_point(&object.center) < world.geometry.robot_radius + object.radius{
            objects_in_trajectory.push(object);
        }
    }
    objects_in_trajectory
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


// fn team_possessing_ball(ball: &Ball, allies: &Robot<AllyInfo>, enemies: &Robot<EnemyInfo>) -> Team {
//     let ball_pos = if 
// }

/// Filter robots by ids.
/// 
/// # Arguments
/// - `robots`: The list of robots to filter.
/// - `ids`: The list of ids to keep.
/// 
/// # Returns
/// A list of robots that are in the ids list.
pub fn filter_robots_in_ids<'a, T>(robots: Vec<&'a Robot<T>>, ids: &Vec<u8>) -> Vec<&'a Robot<T>> {
    robots.into_iter().filter(|r| ids.contains(&r.id)).collect()
}

/// Remove robots with id in the ids list.
/// 
/// # Arguments
/// - `robots`: The list of robots to filter.
/// - `ids`: The list of ids to keep.
/// 
/// # Returns
/// A list of robots that are not in the ids list.
pub fn filter_robots_not_in_ids<'a, T>(robots: Vec<&'a Robot<T>>, ids: &Vec<u8>) -> Vec<&'a Robot<T>> {
    robots.into_iter().filter(|r| !ids.contains(&r.id)).collect()
}
