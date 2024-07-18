use std::f64::consts::PI;

use crabe_math::{shape::{Circle, Line}, vectors::rotate_vector};
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


const MARGIN_SHOOTING_WINDOW: f64 = 0.01;

/// Get the obstruct goal zone from an enemy
/// 
/// # Arguments
/// - `shoot_start_position`: The position of the object that want to go into the goal
/// - `enemy_position`: The position of the enemy
/// - `world`: The current state of the game world
/// 
/// # Returns
/// A `Line` representing the obstruct goal zone
fn get_obstruct_goal_zone_from_enemy(shoot_start_position: &Point2<f64>, enemy_position: &Point2<f64>, world: &World) -> Option<Line> {
    let start_pos_to_enemy = enemy_position - shoot_start_position;
    let perp = rotate_vector(start_pos_to_enemy.normalize(), PI/2.) * (world.geometry.robot_radius + world.geometry.ball_radius + MARGIN_SHOOTING_WINDOW);
    let ray_left_side = (enemy_position + perp) - shoot_start_position;
    let ray_right_side = (enemy_position - perp) - shoot_start_position;
    let line_ray_left_side = Line::new(*shoot_start_position, shoot_start_position + ray_left_side * 1000.);
    let line_ray_right_side = Line::new(*shoot_start_position, shoot_start_position + ray_right_side * 1000.);
    let intersection_left = line_ray_left_side.intersection_segment_line(&world.geometry.enemy_goal.line);
    let intersection_right = line_ray_right_side.intersection_segment_line(&world.geometry.enemy_goal.line);
    match (intersection_left, intersection_right) {
        (Ok(left), Ok(right)) => Some(Line::new(left, right)),
        (Ok(left), _) => Some(Line::new(left, world.geometry.enemy_goal.line.end)),
        (_, Ok(right)) => Some(Line::new(world.geometry.enemy_goal.line.start, right)),
        _ => None,
    }
}

/// Get the open shoot window for the attacker
/// 
/// # Arguments
/// - `shoot_start_position`: The position of the object that want to go into the goal
/// - `world`: The current state of the game world
/// 
/// # Returns
/// A vector of `Line` representing the open shoot windows
pub fn get_open_shoot_window(shoot_start_position: &Point2<f64>, world: &World) -> Vec<Line> {
    let mut available_targets: Vec<Line> = vec![world.geometry.enemy_goal.line];
    for enemy in world.enemies_bot.values() {
        if let Some(line) = get_obstruct_goal_zone_from_enemy(shoot_start_position, &enemy.pose.position.xy(), world){
            let mut new_targets: Vec<Line> = vec![];
            for target_line in available_targets {
                let targets = target_line.cut_off_segment(&line);
                new_targets.extend(targets);
            }
            available_targets = new_targets;
        }
    }
    return available_targets;
}

/// Get the robot with the best shooting window (space where the ball can go into the goal)
/// 
/// # Arguments
/// - `robots`: The list of robots to check
/// - `world`: The current state of the game world
/// 
/// # Returns
/// The robot with the best shooting window
pub fn get_best_shooting_window_bot<'a, T>(robots: &'a Vec<&Robot<T>>, world: &World) -> Option<&'a Robot<T>> {
    // grab allies in the enemy side 
    let mut best_robot: Option<&Robot<T>> = None;
    let mut max_window_length = 0.;
    for robot in robots {
        if object_in_bot_trajectory(world, robot.id, robot.pose.position, false, false, true).len() > 0 {
            continue;
        }
        let shoot_windows = get_open_shoot_window(&robot.pose.position, world);
        let total_length = shoot_windows.iter().fold(0., |acc, line| acc + line.norm());
        if total_length <= max_window_length {
            continue;
        }
        max_window_length = total_length;
        best_robot = Some(robot);
    }
    best_robot
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

/// Get the id of the enemy goalkeeper.
/// 
/// # Arguments
/// world - The world data.
/// 
/// # Returns
/// The id of the enemy goalkeeper.
/// If there's no enemy on the field, return 7
pub fn get_enemy_keeper_id(world: &World) -> u8 {
    if let Some(enemy_infos) = &world.data.enemy.info {
        enemy_infos.goalkeeper as u8
    } else if let Some(enemy_keeper) = closest_bot_to_point(
        world.enemies_bot.values().collect(),
        world.geometry.enemy_goal.line.center()
    ) {
        enemy_keeper.id
    } else {
        // There's no enemies on the field
        7
    }

}