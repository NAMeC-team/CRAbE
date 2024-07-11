use crate::action::state::State;
use crate::action::Action;
use crate::utils::{robots_to_circles, utils};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crabe_math::shape::{Circle, Line};
use crabe_math::vectors::{rotate_vector, vector_from_angle};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::{PI, TAU};
use std::path;

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct MoveTo {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,
    charge: bool,
    dribbler: f32,
    kicker: Option<Kick>,
    fast: bool,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            fast: other.fast,
        }
    }
}

impl MoveTo {
    /// Creates a new `MoveTo` instance.
    ///
    /// # Arguments
    ///
    /// * `target`: The target position on the field to move the robot to.
    /// * `orientation`: The target orientation of the robot.
    pub fn new(
        target: Point2<f64>,
        orientation: f64,
        dribbler: f32,
        charge: bool,
        kicker: Option<Kick>,
        fast: bool,
    ) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            charge,
            dribbler,
            kicker,
            fast,
        }
    }
}

fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), orientation)
}

fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
    frame.inverse()
}

fn robot_frame(robot: &Robot<AllyInfo>) -> Isometry2<f64> {
    frame(
        robot.pose.position.x,
        robot.pose.position.y,
        robot.pose.orientation,
    )
}

fn angle_difference(alpha1: f64, alpha2: f64) -> f64 {
    let diff = alpha1 - alpha2;
    match diff {
        d if d > PI => d - TAU,
        d if d < -PI => d + TAU,
        d => d,
    }
}

/// The default factor speed for the robot to move towards the target position.
const GOTO_SPEED: f64 = 1.5;
/// The overshooting factor to make the robot get faster to the real target.
const GOTO_SPEED_FAST: f64 = 3.;
/// The default factor speed for the robot to rotate towards the target orientation.
const GOTO_ROTATION: f64 = 1.5;
/// The overshooting factor to make the robot rotate faster to the real target.
const GOTO_ROTATION_FAST: f64 = 3.;

/// The error tolerance for arriving at the target position.
const ERR_TOLERANCE: f64 = 0.1;

fn front_objects_in_trajectory(trajectory: &Line, circles: &Vec<Circle>, segment_width: f64, _tools: &mut ToolData) -> Vec<Circle> {
    //draw all circles in annotations
    // for circle in circles.iter(){
    //     _tools.annotations.add_circle(vec!["circle".to_string(), circle.center.to_string()].join("-"), circle.clone());
    // }
    // _tools.annotations.add_line(vec!["trajectory".to_string()].join("-"), trajectory.clone());
    let objects_in_trajectory = trajectory.circles_on_segment(circles, segment_width);
    // filter dot product and distance 
    objects_in_trajectory
        .into_iter()
        .filter(|object| {
            let trajectory_vector = (trajectory.end - trajectory.start).normalize();
            let object_vector = (object.center - trajectory.start).normalize();
            trajectory_vector.dot(&object_vector) > 0.1 
        })
        .collect()
}

fn r_star(_tools: &mut ToolData, objects:&Vec<Circle>, segment_width: f64, start: Point2<f64>, target: Point2<f64>, itt_nb: usize) -> (f64, Vec<Point2<f64>>){
    if itt_nb == 0{
        return (0.0, vec![start]);
    }
    let (left_angle, left_target) = get_first_angle_free_trajectory(objects, segment_width, start, target, false, _tools);
    let (right_angle, right_target) = get_first_angle_free_trajectory(objects, segment_width, start, target, true, _tools);
    if (left_target - target).norm() < 0.5 || (right_target - target).norm() < 0.5{
        // _tools.annotations.add_line(vec!["r_star_left".to_string(), start.to_string()].join("-"), Line::new(start, target));
        return ((target - start).norm(), vec![target]);
    }else{
        let (length_left_r_star,mut path_left_r_star) = r_star(_tools, objects, segment_width, left_target, target, itt_nb - 1);
        let (length_right_r_star,mut path_right_r_star) = r_star(_tools, objects, segment_width, right_target, target, itt_nb - 1);
        // _tools.annotations.add_line(vec!["r_star_left".to_string(), start.to_string()].join("-"), Line::new(start, left_target));
        // _tools.annotations.add_line(vec!["r_star_right".to_string(), start.to_string()].join("-"), Line::new(start, right_target));
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

fn get_first_angle_free_trajectory(objects:&Vec<Circle>, segment_width: f64, start: Point2<f64>, target: Point2<f64>, pos_rotation: bool, _tools: &mut ToolData) -> (f64, Point2<f64>){
    let mut angle = 0.0;
    let mut target = target;
    let mut free = false;
    while !free && angle < PI{
        let dir = rotate_vector((target - start).normalize(), angle);
        target = start + dir.normalize() * 0.5;
        let trajectory = Line::new(start, target);
        let objects_on_trajectory = front_objects_in_trajectory(&trajectory, &objects, segment_width, _tools);
        if objects_on_trajectory.len() == 0{
            free = true;
        } else {
            if pos_rotation{
                angle += 0.1;
            } else {
                angle -= 0.1;
            }
        }
    }
    (angle, target)
}

fn smooth_path(path: &Vec<Point2<f64>>, objects: &Vec<Circle>, segment_width: f64, _tools: &mut ToolData) -> Vec<Point2<f64>>{
    if path.len() <= 2{
        return path.clone();
    }
    let mut new_path = vec![path[0]];
    let mut i = 0;
    while i < path.len(){
        let mut j = i + 1;
        while j < path.len() && front_objects_in_trajectory(&Line::new(path[i], path[j]), objects, segment_width, _tools).len() == 0{
            println!("j {:?}", j);
            j += 1;
        }
        new_path.push(path[j-1]);
        i = j;
    }
    new_path
}

impl Action for MoveTo {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("MoveTo")
    }

    /// Returns the state of the action.
    fn state(&mut self) -> State {
        self.state
    }

    /// Computes the orders to be sent to the robot and returns a `Command` instance.
    /// If the robot arrives at the target position and orientation, the action is considered done.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the robot for which the orders are computed.
    /// * `world`: The current state of the world.
    /// * `tools`: A collection of external tools used by the action, such as a viewer.
    fn compute_order(&mut self, id: u8, world: &World, _tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&id) {
            let mut target = self.target;
            if robot.distance(&self.target) > 0.5 {
                let allies_bot_without_actual_robot = world.allies_bot.values().filter(|r: &&Robot<AllyInfo>| r.id != robot.id).collect();
                let allies_objects = robots_to_circles(world, allies_bot_without_actual_robot);
                let enemies_objects = robots_to_circles(world, world.enemies_bot.values().collect());
                let objects = [&allies_objects[..], &enemies_objects[..]].concat();
                let trajectory = Line::new(robot.pose.position, self.target);
                if front_objects_in_trajectory(&trajectory, &objects, world.geometry.robot_radius+0.05, _tools).len() > 0{
                    let (_, mut tmp_path) = r_star(_tools, &objects, world.geometry.robot_radius+0.05, robot.pose.position, self.target,8);
                    tmp_path.push(robot.pose.position);
                    // path is reversed tmp_path
                    let path: Vec<Point2<f64>> = tmp_path.into_iter().rev().collect();
                    for i in 0..path.len()-1{
                        _tools.annotations.add_line(vec!["path".to_string(), i.to_string()].join("-"), Line::new(path[i], path[i+1]));
                    }
                    println!("path {:?}", path);
                    // add path to annotations 
                    let smooth_path = smooth_path(&path, &objects, world.geometry.robot_radius+0.05, _tools);            
                    println!("smooth_path {:?}", smooth_path);
                    if smooth_path.len() > 1{
                        for i in 0..smooth_path.len()-1{
                            _tools.annotations.add_line(vec!["path".to_string(), i.to_string()].join("-"), Line::new(smooth_path[i], smooth_path[i+1]));
                        }   
                        let robot_to_new_target = smooth_path[1] - robot.pose.position;
                        target = robot.pose.position + robot_to_new_target;
                    }
                }
            }
            let ti = frame_inv(robot_frame(robot));
            let target_in_robot = ti * Point2::new(target.x, target.y);
            _tools.annotations.add_circle(vec!["target".to_string(), id.to_string()].join("-"),Circle::new(target, 0.1));
            let error_orientation = angle_difference(self.orientation, robot.pose.orientation);
            let error_x = target_in_robot[0];
            let error_y = target_in_robot[1];
            let arrived = Vector3::new(error_x, error_y, error_orientation).norm() < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }

            let order = 
            if self.fast {
                Vector3::new(
                GOTO_SPEED_FAST * error_x,
                GOTO_SPEED_FAST * error_y,
                GOTO_ROTATION_FAST * error_orientation,
                )
            } else {
                Vector3::new(
                GOTO_SPEED * error_x,
                GOTO_SPEED * error_y,
                GOTO_ROTATION * error_orientation,
                )
            };
            // Command::default()

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                angular_velocity: order.z as f32,
                charge: self.charge,
                kick: self.kicker,
                dribbler: self.dribbler,
                fast: self.fast,
            }
        } else {
            Command::default()
        }
    }
}
