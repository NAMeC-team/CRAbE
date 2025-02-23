use crate::action::state::State;
use crate::action::Action;
use crate::utils::navigation::obstacle_avoidance;
use crate::utils::{penalty_zone_prevention, KEEPER_ID};
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
    pub state: State,
    /// The target position to move to.
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    /// The target orientation of the robot.
    pub orientation: Option<f64>,
    pub charge: bool,
    pub dribbler: f32,
    pub kicker: Option<Kick>,
    pub fast: bool,
    pub avoidance: bool,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target_x: other.target_x,
            target_y: other.target_y,
            orientation: other.orientation,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            fast: other.fast,
            avoidance: other.avoidance
        }
    }
}

impl MoveTo {
    /// Creates a new `MoveTo` instance where the default target position and orientation are set to the current position and orientation of the robot.
    /// You can then use the builder pattern to modify parameters.
    pub fn new() -> Self {
        Self {
            state: State::Running,
            target_x: None,
            target_y: None,
            orientation: None,
            charge: false,
            dribbler: 0.,
            kicker: None,
            fast: true,
            avoidance: true
        }
    }

    /// Creates a new `MoveTo` instance by specifying all the params manually (DO NOT USE THIS FUNCTION, the only purpose is to handle previously existing code)
    ///
    /// # Arguments
    ///
    /// * `target`: The target position on the field to move the robot to.
    /// * `orientation`: The target orientation of the robot.
    pub fn new_all_params(
        target: Point2<f64>,
        orientation: f64,
        dribbler: f32,
        charge: bool,
        kicker: Option<Kick>,
        fast: bool,
        avoidance: bool,
    ) -> Self {
        Self {
            state: State::Running,
            target_x: Some(target.x),
            target_y: Some(target.y),
            orientation: Some(orientation),
            charge,
            dribbler,
            kicker,
            fast,
            avoidance
        }
    }

    /// Desactivate obstacle avoidance.
    pub fn no_avoidance(mut self) -> Self {
        self.avoidance = false;
        self
    }

    /// Makes the robot move slower towards the target position and orientation.
    pub fn slowly(mut self) -> Self {
        self.fast = false;
        self
    }

    /// Activate the charging mode of the robot.
    pub fn charging(mut self) -> Self {
        self.charge = true;
        self
    }

    /// Set the kicker to be used by the robot.
    /// 
    /// # Arguments
    /// * `kicker`: The kicker to be used by the robot.
    pub fn set_kick(mut self, kicker: Kick) -> Self {
        self.kicker = Some(kicker);
        self
    }

    /// Set the dribbler speed of the robot.
    /// 
    /// # Arguments
    /// * `dribbler`: The dribbler speed of the robot.
    pub fn set_dribbler(mut self, dribbler: f32) -> Self {
        self.dribbler = dribbler;
        self
    }

    /// Set the x target position of the robot.
    /// 
    /// # Arguments
    /// * `x`: The x target position of the robot.
    pub fn set_x(mut self, x: f64) -> Self {
        self.target_x = Some(x);
        self
    }

    /// Set the y target position of the robot. 
    /// 
    /// # Arguments
    /// * `y`: The y target position of the robot.
    pub fn set_y(mut self, y: f64) -> Self {
        self.target_y = Some(y);
        self
    }

    /// Set the target position of the robot.
    /// 
    /// # Arguments
    /// * `target`: A 2d point representing the target position of the robot.
    pub fn set_target(mut self, target: Point2<f64>) -> Self {
        self.target_x = Some(target.x);
        self.target_y = Some(target.y);
        self
    }

    /// Set the target orientation of the robot.
    /// 
    /// # Arguments
    /// * `orientation`: The target angle of the robot.
    pub fn set_orientation(mut self, orientation: f64) -> Self {
        self.orientation = Some(orientation);
        self
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
            let mut order = Vector3::new(0., 0., 0.);
            let ti = frame_inv(robot_frame(robot));

            // set the target position and orientation to the current position and orientation of the robot if they are not set
            self.target_x.get_or_insert_with(|| robot.pose.position.x);
            self.target_y.get_or_insert_with(|| robot.pose.position.y);
            self.orientation.get_or_insert_with(|| robot.pose.orientation);

            // calculate position command 
            let mut target = Point2::new(self.target_x.unwrap(), self.target_y.unwrap());
            if id != KEEPER_ID{
                target = penalty_zone_prevention(&robot.pose.position, &target, world);
            }
            if self.avoidance{
                target = obstacle_avoidance(&target, robot, world, _tools);
            }
            _tools.annotations.add_circle(vec!["target".to_string(), id.to_string()].join("-"),Circle::new(target, 0.1));
            let target_in_robot = ti * Point2::new(target.x, target.y);
            order.x = target_in_robot[0];
            order.y = target_in_robot[1];

            // calculate orientation command
            let orientation = self.orientation.unwrap();
            order.z = angle_difference(orientation, robot.pose.orientation);

            // if the robot is close to the target orientation, it will stop this moveto
            let arrived = order.norm() < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }

            // mutliply by the speed factors
            let speed = if self.fast { GOTO_SPEED_FAST } else { GOTO_SPEED };
            let rotation = if self.fast { GOTO_ROTATION_FAST } else { GOTO_ROTATION };
            order.x = speed * order.x;
            order.y = speed * order.y;
            order.z = rotation * order.z;
            // Command::default()

            Command {
                forward_velocity: order.x as f32,
                left_velocity: order.y as f32,
                angular_velocity: order.z as f32,
                charge: self.charge,
                kick: self.kicker,
                dribbler: self.dribbler,
            }
        } else {
            Command::default()
        }
    }
}

