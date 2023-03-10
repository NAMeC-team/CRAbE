use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct MoveTo {
    state: State,
    target: Point2<f64>,
    orientation: f64,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
        }
    }
}

impl MoveTo {
    pub fn new(target: Point2<f64>, orientation: f64) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
        }
    }
}

pub fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), orientation)
}

pub fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
    frame.inverse()
}

pub fn robot_frame(robot: &Robot<AllyInfo>) -> Isometry2<f64> {
    frame(
        robot.pose.position.x,
        robot.pose.position.y,
        robot.pose.orientation,
    )
}

pub fn angle_wrap(alpha: f64) -> f64 {
    (alpha + PI) % (2.0 * PI) - PI
}

const GOTO_SPEED: f64 = 1.5;
const GOTO_ROTATION: f64 = 1.5;
const ERR_TOLERANCE: f64 = 0.115;

impl Action for MoveTo {
    fn name(&self) -> String {
        String::from("MoveTo")
    }

    fn state(&mut self) -> State {
        self.state.clone()
    }

    fn compute_order(&mut self, id: u8, world: &World, _tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&(id as u32)) {
            let ti = frame_inv(robot_frame(robot));
            let target_in_robot = ti * Point2::new(self.target.x, self.target.y);

            let error_orientation = angle_wrap(self.orientation - robot.pose.orientation);
            let error_x = target_in_robot[0];
            let error_y = target_in_robot[1];
            let arrived = Vector3::new(error_x, error_y, error_orientation).norm() < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }

            let order = Vector3::new(
                GOTO_SPEED * error_x,
                GOTO_SPEED * error_y,
                GOTO_ROTATION * error_orientation,
            );

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                angular_velocity: order.z as f32,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        } else {
            Command::default()
        }
    }

    fn cancel(&mut self) {}
}
