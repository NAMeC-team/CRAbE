use crate::action::state::State;
use crate::action::Action;
use crate::utils::{obstacle_avoidance, penalty_zone_prevention, KEEPER_ID};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};

/// The `GoTo` struct represents an action that moves the robot to a specific location on the field without moving orientation.
#[derive(Clone)]
pub struct GoTo {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    charge: bool,
    dribbler: f32,
    kicker: Option<Kick>,
    fast: bool,
}

impl From<&mut GoTo> for GoTo {
    fn from(other: &mut GoTo) -> GoTo {
        GoTo {
            state: other.state,
            target: other.target,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            fast: other.fast,
        }
    }
}

impl GoTo {
    /// Creates a new `GoTo` instance.
    ///
    /// # Arguments
    ///
    /// * `target`: The target position on the field to move the robot to.
    /// * `orientation`: The target orientation of the robot.
    pub fn new(
        target: Point2<f64>,
        dribbler: f32,
        charge: bool,
        kicker: Option<Kick>,
        fast: bool,
    ) -> Self {
        Self {
            state: State::Running,
            target,
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


/// The default factor speed for the robot to move towards the target position.
const GOTO_SPEED: f64 = 1.5;
/// The overshooting factor to make the robot get faster to the real target.
const GOTO_SPEED_FAST: f64 = 3.;

/// The error tolerance for arriving at the target position.
const ERR_TOLERANCE: f64 = 0.1;

impl Action for GoTo {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("GoTo")
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
            if id != KEEPER_ID{
                self.target = penalty_zone_prevention(&robot.pose.position, &self.target, world)
            }
            self.target = obstacle_avoidance(&self.target, robot, world, _tools);
            let ti = frame_inv(robot_frame(robot));
            let target_in_robot = ti * Point2::new(self.target.x, self.target.y);
            
            let error_x = target_in_robot[0];
            let error_y = target_in_robot[1];
            let arrived = Vector2::new(error_x, error_y).norm() < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }

            let order = 
            if self.fast {
                Vector2::new(
                GOTO_SPEED_FAST * error_x,
                GOTO_SPEED_FAST * error_y,
                )
            } else {
                Vector2::new(
                GOTO_SPEED * error_x,
                GOTO_SPEED * error_y,
                )
            };

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                charge: self.charge,
                kick: self.kicker,
                dribbler: self.dribbler,
                ..Default::default()
            }
        } else {
            Command::default()
        }
    }
}
