use std::f64::consts::PI;

use crabe_framework::data::{tool::ToolData, world::{AllyInfo, Robot, World}};
use crabe_math::{shape::{Circle, Line}, vectors::rotate_vector};
use nalgebra::Point2;

use crate::utils::robots_to_circles;

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
    if robot.distance(target) > 0.5 {
        let allies_bot_without_actual_robot = world.allies_bot.values().filter(|r: &&Robot<AllyInfo>| r.id != robot.id).collect();
        let allies_objects = robots_to_circles(world, allies_bot_without_actual_robot);
        let enemies_objects = robots_to_circles(world, world.enemies_bot.values().collect());
        let objects = [&allies_objects[..], &enemies_objects[..]].concat();
        let trajectory = Line::new(robot.pose.position, *target);
        if front_objects_in_trajectory(&trajectory, &objects, world.geometry.robot_radius+0.05).len() > 0{
            let (_, mut tmp_path) = r_star(&objects, world.geometry.robot_radius+0.05, robot.pose.position, target, 8);
            tmp_path.push(robot.pose.position);
            // path is reversed tmp_path
            let path: Vec<Point2<f64>> = tmp_path.into_iter().rev().collect();
            for i in 0..path.len()-1{
                _tools.annotations.add_line(vec!["path".to_string(), i.to_string()].join("-"), Line::new(path[i], path[i+1]));
            }
            // add path to annotations 
            let smooth_path = smooth_path(&path, &objects, world.geometry.robot_radius+0.05, _tools);            
            if smooth_path.len() > 1{
                for i in 0..smooth_path.len()-1{
                    _tools.annotations.add_line(vec!["path".to_string(), i.to_string()].join("-"), Line::new(smooth_path[i], smooth_path[i+1]));
                }   
                let robot_to_new_target = smooth_path[1] - robot.pose.position;
                return robot.pose.position + robot_to_new_target;
            }
        }
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
    if (left_target - target).norm() < 0.5 || (right_target - target).norm() < 0.5{
        return ((target - start).norm(), vec![*target]);
    }else{
        let (length_left_r_star,mut path_left_r_star) = r_star( objects, segment_width, left_target, target, itt_nb - 1);
        let (length_right_r_star,mut path_right_r_star) = r_star(objects, segment_width, right_target, target, itt_nb - 1);
        let dist_from_target_left: f64 = (target - path_left_r_star[0]).norm();
        let dist_from_target_right: f64 = (target - path_right_r_star[0]).norm();
        if dist_from_target_left < 0.01 && dist_from_target_right < 0.01{
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
    let mut angle = 0.0;
    let mut new_target = target.clone();
    let mut free = false;
    while !free && angle < PI && angle > -PI{
        let dir = rotate_vector((target - start).normalize(), angle);
        new_target = start + dir.normalize() * 0.5;
        let trajectory = Line::new(*start, new_target);
        let objects_on_trajectory = front_objects_in_trajectory(&trajectory, &objects, segment_width);
        if objects_on_trajectory.len() == 0{
            free = true;
        } else {
            if positive_rotation{
                angle += 0.1;
            } else {
                angle -= 0.1;
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
fn smooth_path(path: &Vec<Point2<f64>>, objects: &Vec<Circle>, segment_width: f64, _tools: &mut ToolData) -> Vec<Point2<f64>>{
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

