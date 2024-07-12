use crate::action::state::State;
use crate::action::Action;
use crate::utils::navigation::obstacle_avoidance;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crabe_math::shape::{Circle, Line};
use crabe_math::vectors::{rotate_vector, vector_from_angle};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::{PI, TAU};

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
            let target = obstacle_avoidance(&self.target, robot, world, _tools);
            
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
