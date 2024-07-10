use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use std::f64::consts::{PI, TAU};

/// The `OrientTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct OrientTo {
    /// The current state of the action.
    state: State,
    /// The target orientation of the robot.
    orientation: f64,
    charge: bool,
    dribbler: f32,
    kicker: Option<Kick>,
    fast: bool,
}

impl From<&mut OrientTo> for OrientTo {
    fn from(other: &mut OrientTo) -> OrientTo {
        OrientTo {
            state: other.state,
            orientation: other.orientation,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            fast: other.fast,
        }
    }
}

impl OrientTo {
    /// Creates a new `OrientTo` instance.
    ///
    /// # Arguments
    ///
    /// * `orientation`: The target orientation of the robot.
    pub fn new(
        orientation: f64,
        dribbler: f32,
        charge: bool,
        kicker: Option<Kick>,
        fast: bool,
    ) -> Self {
        Self {
            state: State::Running,
            orientation,
            charge,
            dribbler,
            kicker,
            fast,
        }
    }
}



fn angle_difference(alpha1: f64, alpha2: f64) -> f64 {
    let diff = alpha1 - alpha2;
    match diff {
        d if d > PI => d - TAU,
        d if d < -PI => d + TAU,
        d => d,
    }
}

/// The default factor speed for the robot to rotate towards the target orientation.
const GOTO_ROTATION: f64 = 1.5;
/// The overshooting factor to make the robot rotate faster to the real target.
const GOTO_ROTATION_FAST: f64 = 3.;

/// The error tolerance for arriving at the target position.
const ERR_TOLERANCE: f64 = 0.1;

impl Action for OrientTo {
    /// Returns the name of the action.
    fn name(&self) -> String {
        String::from("OrientTo")
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


            let error_orientation = angle_difference(self.orientation, robot.pose.orientation);
            let arrived = error_orientation < ERR_TOLERANCE;
            if arrived {
                self.state = State::Done;
            }

            let order:f64 = 
            if self.fast {
                GOTO_ROTATION_FAST * error_orientation
            } else {
                GOTO_ROTATION * error_orientation
            };

            Command {
                angular_velocity: order as f32,
                charge: self.charge,
                kick: self.kicker,
                dribbler: self.dribbler,
                fast: self.fast,
                ..Default::default()
            }
        } else {
            Command::default()
        }
    }
}
