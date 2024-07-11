use std::f64::consts::PI;

use crabe_framework::data::{tool::ToolData, world::{AllyInfo, Robot, World}};
use crabe_math::{shape::{Circle, Line}, vectors::rotate_vector};
use nalgebra::Point2;

use crate::utils::robots_to_circles;

const NO_AVOIDANCE_DIST : f64 = 0.4;        // distance to the target to start avoiding obstacles
const EXPLORATION_STEPS_LENGTH : f64 = 0.5; // length of the step to explore the field
const EXPLORATION_STOP_DIST : f64 = 0.4;    // distance to the target to stop the exploration
const EXPLORATION_ANGLE : f64 = 0.1;        // angle between two exploration steps
const EXPLORATION_ITTERATION : usize = 8;   // number of iterations for the exploration (O(n*2) so be careful)
const AVOIDANCE_MARGIN : f64 = 0.05;        // margin to avoid obstacles (added to the bot radius)
const BALL_AVOIDANCE_MARGIN : f64 = 0.03;   // margin to avoid ball (added to the bot radius)
const OVERSHOOTING_DIST : f64 = 0.5;        // overshooting dist to the new target, use to regulate speed while avoiding obstacles

/// Return a point avoiding obstacles
/// 
/// # Arguments
/// - target: The target point
/// - robot: The robot
/// - world: The world
/// - _tools: The tools to add annotations
/// 
/// # Returns
/// The new point to move to
pub fn obstacle_avoidance(target: &Point2<f64>, robot: &Robot<AllyInfo>, world: &World, _tools: &mut ToolData) -> Point2<f64>{
    if robot.distance(target) <= NO_AVOIDANCE_DIST {
        return target.clone();
    }
    let allies_bot_without_actual_robot = world.allies_bot.values().filter(|r: &&Robot<AllyInfo>| r.id != robot.id).collect();
    let allies_objects = robots_to_circles(world, allies_bot_without_actual_robot);
    let enemies_objects = robots_to_circles(world, world.enemies_bot.values().collect());
    let mut objects = [&allies_objects[..], &enemies_objects[..]].concat();
    if let Some(ball) = &world.ball{
        let ball_circle = Circle::new(ball.position_2d(), world.geometry.ball_radius + BALL_AVOIDANCE_MARGIN);
        objects.push(ball_circle);
    }
    // list of objects to avoid
    let avoidance_width = world.geometry.robot_radius+AVOIDANCE_MARGIN;

    let trajectory = Line::new(robot.pose.position, *target);
    // if there is no object in front of the robot on the trajectory, we can go straight to the target
    if front_objects_in_trajectory(&trajectory, &objects, avoidance_width).len() == 0{
        return target.clone();
    }
    let (_, mut path) = r_star(&objects, avoidance_width, robot.pose.position, target, EXPLORATION_ITTERATION);
    path.push(robot.pose.position);
    // reverse to have bot in first position
    let path: Vec<Point2<f64>> = path.into_iter().rev().collect();
    let smooth_path = smooth_path(&path, &objects, avoidance_width);            

    if smooth_path.len() > 1{
        // add path to annotations 
        for i in 0..smooth_path.len()-1{
            _tools.annotations.add_line(vec!["path".to_string(), i.to_string()].join("-"), Line::new(smooth_path[i], smooth_path[i+1]));
        }
        let dir = smooth_path[1] - robot.pose.position;
        if dir.norm() < OVERSHOOTING_DIST{
            let overshooted_dir = dir.normalize() * OVERSHOOTING_DIST;
            return robot.pose.position + overshooted_dir;
        }
        return robot.pose.position + dir;
    }

    target.clone()
}

/// Get the objects in front of the robot on a trajectory
/// 
/// # Arguments
/// - trajectory: The trajectory to consider
/// - circles: The objects on the field
/// - segment_width: The width of the segment to consider collision with objects
/// 
/// # Returns
/// The objects in front of the robot on the trajectory
fn front_objects_in_trajectory(trajectory: &Line, circles: &Vec<Circle>, segment_width: f64) -> Vec<Circle> {
    let objects_in_trajectory = trajectory.circles_on_segment(circles, segment_width);
    objects_in_trajectory
        .into_iter()
        .filter(|object| {
            let trajectory_vector = (trajectory.end - trajectory.start).normalize();
            let object_vector = (object.center - trajectory.start).normalize();
            trajectory_vector.dot(&object_vector) > 0.1 // filtering on dot product to get only objects in front of the robot
        })
        .collect()
}

/// R* algorithm to find the shortest path to the target
///     
/// # Arguments
/// - objects: The objects on the field
/// - segment_width: The width of the segment to consider collision with objects
/// - start: The start point
/// - target: The target point
/// - itt_nb: The number of iterations
/// 
/// # Returns
/// The length of the path and the path
fn r_star(objects:&Vec<Circle>, segment_width: f64, start: Point2<f64>, target: &Point2<f64>, itt_nb: usize) -> (f64, Vec<Point2<f64>>){
    if itt_nb == 0{
        return (0.0, vec![start]);
    }
    let (_, left_target) = get_first_angle_free_trajectory(objects, segment_width, &start, target, false);
    let (_, right_target) = get_first_angle_free_trajectory(objects, segment_width, &start, target, true);
    if (left_target - target).norm() < EXPLORATION_STOP_DIST || (right_target - target).norm() < EXPLORATION_STOP_DIST{ // if the target is close enough we stop the exploration
        return ((target - start).norm(), vec![*target]);
    } else {
        let (length_left_r_star,mut path_left_r_star) = r_star( objects, segment_width, left_target, target, itt_nb - 1);
        let (length_right_r_star,mut path_right_r_star) = r_star(objects, segment_width, right_target, target, itt_nb - 1);

        let dist_from_target_left: f64 = (target - path_left_r_star[0]).norm();
        let dist_from_target_right: f64 = (target - path_right_r_star[0]).norm();

        if dist_from_target_left < 0.01 && dist_from_target_right < 0.01{ // check if they are at the target (0.01 is the tolerance for the distance to the target)
            if length_left_r_star < length_right_r_star {
                path_left_r_star.push(left_target);
                return (length_left_r_star + (left_target - start).norm(), path_left_r_star);
            }else{
                path_right_r_star.push(right_target);
                return (length_right_r_star + (right_target - start).norm(), path_right_r_star);
            }
        }else if dist_from_target_left < 0.01{
            path_left_r_star.push(left_target);
            return (length_left_r_star + (left_target - start).norm(), path_left_r_star);
        }else{
            path_right_r_star.push(right_target);
            return (length_right_r_star + (right_target - start).norm(), path_right_r_star);
        }
    }
}

/// Get the first angle that is free to move to the target (Either on the right side or on the left side)
/// 
/// # Arguments
/// - objects: The objects on the field
/// - segment_width: The width of the segment to consider collision with objects
/// - start: The start point
/// - target: The target point
/// - positive_rotation: The direction of the rotation
/// 
/// # Returns
/// The angle and the new target point on the available direction
fn get_first_angle_free_trajectory(objects:&Vec<Circle>, segment_width: f64, start: &Point2<f64>, target: &Point2<f64>, positive_rotation: bool) -> (f64, Point2<f64>){
    let mut angle = 0.;
    let mut new_target = target.clone();
    let mut free = false;
    while !free && angle < PI && angle > -PI{
        let dir = rotate_vector((target - start).normalize(), angle);
        new_target = start + dir.normalize() * EXPLORATION_STEPS_LENGTH;
        let trajectory = Line::new(*start, new_target);
        let objects_on_trajectory = front_objects_in_trajectory(&trajectory, &objects, segment_width);
        if objects_on_trajectory.len() == 0{
            free = true;
        } else {
            if positive_rotation{
                angle += EXPLORATION_ANGLE;
            } else {
                angle -= EXPLORATION_ANGLE;
            }
        }
    }
    (angle, new_target)
}

/// Smooth the path by removing points that are not necessary (i.e. in case he can cross the path without colliding with an object we shorten the path)
/// 
/// # Arguments
/// - path: The path to smooth
/// - objects: The objects on the field
/// - segment_width: The width of the segment to consider collision with objects
/// - _tools: The tools to add annotations
/// 
/// # Returns
/// The smoothed path
fn smooth_path(path: &Vec<Point2<f64>>, objects: &Vec<Circle>, segment_width: f64) -> Vec<Point2<f64>>{
    if path.len() <= 2{
        return path.clone();
    }
    let mut new_path = vec![path[0]];
    let mut i = 0;
    while i < path.len(){
        let mut j = i + 1;
        while j < path.len() && front_objects_in_trajectory(&Line::new(path[i], path[j]), objects, segment_width).len() == 0{
            j += 1;
        }
        new_path.push(path[j-1]);
        i = j;
    }
    new_path
}

