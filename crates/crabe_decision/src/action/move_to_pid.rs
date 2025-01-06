use std::f64::consts::{FRAC_PI_8, PI};
use std::time::{Instant};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crate::action::Action;
use crate::action::state::State;

/// Proportional factor of the PID controller
const K_P: f64 = 2.5;
/// Integral factor of the PID controller
const K_I: f64 = 0.9;
/// Derivative factor of the PID controller
const K_D: f64 = 1.;

/// Number of errors to keep track of when computing
/// the integrative value of the PID controller
const PID_NUM_ERRORS: usize = 100;

/// Maximum tolerance for error to be non-zero
/// If the error is inferior to this number, error will be considered 0.
/// The same constants are used to determine whether this action is finished or not.
const TARGET_ATTAINED_TOL: f64 = 0.05;
const THETA_ATTAINED_TOL: f64 = FRAC_PI_8 / 8.;  // in radian !

#[derive(Debug, Clone)]
struct PIDErr {
    err: Vector3<f64>,
    timestamp: Option<Instant>,
}

impl Default for PIDErr {
    fn default() -> Self {
        Self {
            err: Vector3::new(0., 0., 0.),
            timestamp: None,
        }
    }
}

impl PIDErr {
    pub fn new(err: Vector3<f64>, timestamp: Option<Instant>) -> Self { Self { err, timestamp } }
}

/// Stores the number of errors for the PID controller
/// used in this movement command
#[derive(Clone)]
struct PIDErrCounter {
    /// Stores the errors in position and angle (rounded to 2 pi), and the time at which it was stored
    /// xy store the position error, and z is used to store the angle error
    errors: Vec<PIDErr>,
    /// Maximum number of errors to keep
    /// This is also the size of the errors array
    max_size: usize,
    /// Current error index. We use it to overwrite the errors over time instead of
    /// shifting the array one by one
    err_index: usize
}

impl Default for PIDErrCounter {
    fn default() -> Self {
        PIDErrCounter {
            errors: vec![PIDErr::default(); PID_NUM_ERRORS],
            max_size: PID_NUM_ERRORS,
            err_index: 0,
        }
    }
}

impl PIDErrCounter {
    /// Get the previous error computed, which is older than the current error
    fn previous(&self) -> &PIDErr { &self.errors[self.previous_error_idx()] }

    /// Get the most recent error that has been computed
    fn current(&self) -> &PIDErr { &self.errors[self.err_index] }

    /// Get the index of the previous error computed in this structure's internal storage
    fn previous_error_idx(&self) -> usize { ((self.err_index as i16 - 1).rem_euclid(self.max_size as i16)) as usize }

    /// Get the index of the next error in this structure's internal storage
    fn next_error_idx(&self) -> usize { (self.err_index + 1) % self.max_size }

    /// Approximately compute the integral term of all the error terms we have saved, from oldest to most recent
    /// In short, this is the integrative term of the PID controller
    ///
    /// Here, we do not assume that the time period between two error computations is equal, but that it rather
    /// fluctuates a little in real life operations (we are directly dependent of the vision data we receive).
    /// This allows us to be a little more precise in our approximation
    fn sum(&mut self) -> Vector3<f64> {
        let mut error_sum = Vector3::new(0., 0., 0.);
        for _ in 0..PID_NUM_ERRORS {
            let cur = self.current();
            let prev = self.previous();

            let mut delta = 1.;

            if let Some(cur_timestamp) = cur.timestamp {
                delta = match prev.timestamp {
                    Some(prev_timestamp) => cur_timestamp.duration_since(prev_timestamp).as_secs_f64(),
                    None => 0.,
                }
            }

            // approximation of integral term using trapezoidal rule (midpoint)
            error_sum += delta * ((1./2.) * (cur.err + prev.err));

            self.err_index = self.previous_error_idx();
        }

        error_sum
    }

    /// Replaces the error at the current index with
    /// the one passed in parameter.
    fn save(&mut self, err_value: Vector3<f64>) {
        // vec was filled with 0. on init
        // this unwrap() shouldn't panic (in theory)
        let old_err = self.errors.get_mut(self.err_index).unwrap();
        *old_err = PIDErr::new(err_value, Some(Instant::now()));
    }

    /// Computes the error sum between the previous and
    /// the current error.
    /// Similar to the derivative term of the movement of the robot.
    /// This must be called after computing the current position error for the robot.
    /// Uses finite differences method
    fn deriv_prev_curr(&self, prev: &PIDErr, cur: &PIDErr) -> Vector3<f64> {
        let mut time_delta = 0.016;

        // if one of the values is None, we consider no derivative can be estimated
        // this only happens like once during the whole program, so whatever
        if let Some(prev_timestamp) = prev.timestamp {
            if let Some(cur_timestamp) = cur.timestamp {
                time_delta = cur_timestamp.duration_since(prev_timestamp).as_secs_f64();
            }
        } else {
            return Vector3::new(0., 0., 0.);
        }

        // time_delta could be 0. (idk why). we prevent this
        if time_delta == 0. {
            // TODO: Identify why time_delta can be 0.
            // oh and this is the value of our refresh rate by the way
            // this is because we force ourselves to have a strict refresh rate. See `crates/crabe/src/main.rs`
            time_delta = 0.016;
        }
        (cur.err - prev.err) / time_delta
    }
}

#[derive(Clone)]
pub struct MoveToPID {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,
    charge: bool,
    dribbler: f32,
    kicker: Option<Kick>,
    avoidance: bool,
    /// Accumulation of the errors computer over time
    error_tracker: PIDErrCounter,
}

impl From<&mut MoveToPID> for MoveToPID {
    fn from(other: &mut MoveToPID) -> MoveToPID {
        MoveToPID {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            avoidance: other.avoidance,
            error_tracker: other.error_tracker.clone(),
        }
    }
}

impl MoveToPID {
    pub fn new(target: Point2<f64>, orientation: f64, charge: bool, dribbler: f32, kicker: Option<Kick>, avoidance: bool) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            charge,
            dribbler,
            kicker,
            avoidance,
            error_tracker: PIDErrCounter::default(),
        }
    }

    /// Returns the current coordinate basis of the robot
    fn robot_basis(&self, robot: &Robot<AllyInfo>) -> Isometry2<f64> {
        Isometry2::new(
            Vector2::new(robot.pose.position.x, robot.pose.position.y),
            robot.pose.orientation
        )
    }

    /// Compute the error between the current position
    /// and the target to attain.
    fn error_to_target(&self,
                       robot: &Robot<AllyInfo>,
                       target_position: Point2<f64>,
                       target_orientation: f64) -> Vector3<f64> {
        let mut computed_err = Vector3::new(0., 0., 0.);

        // change target into basis of robot to compute err
        let robot_basis = self.robot_basis(robot).inverse();
        let pos_err = robot_basis * target_position;

        // TODO: error angle should be higher when far from target, and very small when close to it
        let dist_to_target: f64 = nalgebra::distance(&robot.pose.position, &target_position);
        let theta_diff = target_orientation - robot.pose.orientation;
        let err_theta = self.angle_wrap(theta_diff + (dist_to_target + 0.9) * theta_diff);

        // consider error is 0. if is it superior to max tolerance
        computed_err.x = if pos_err.x.abs() > TARGET_ATTAINED_TOL { pos_err.x } else { 0. };
        computed_err.y = if pos_err.y.abs() > TARGET_ATTAINED_TOL { pos_err.y } else { 0. };
        computed_err.z = if err_theta.abs() > THETA_ATTAINED_TOL { err_theta } else { 0. };

        computed_err
    }

    fn angle_wrap(&self, alpha: f64) -> f64 { (alpha + PI) % (2.0 * PI) - PI }
}

impl Action for MoveToPID {
    fn name(&self) -> String {
        String::from("MoveToPID")
    }

    fn state(&mut self) -> State { self.state }

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&id) {
            // take current error in account for next command
            let current_error = self.error_to_target(robot, self.target, self.orientation);
            self.error_tracker.save(current_error);

            // Stop the robot if it has attained its destination
            if current_error.norm() <= TARGET_ATTAINED_TOL + THETA_ATTAINED_TOL {
                self.state = State::Done;
                return Command::default();
            }

            // compute in order, the factors of the PID
            let p = K_P * self.error_tracker.current().err;
            let i = K_I * self.error_tracker.sum();
            let d = K_D * self.error_tracker.deriv_prev_curr(self.error_tracker.previous(), self.error_tracker.current());

            dbg!(&i);
            let vec_command: Vector3<f64> = p + i + d;
            dbg!(&vec_command.z);

            // after computing the PID control values, we increase the index of the current error by one
            // this is necessary, otherwise we'll always replace the value at index 0
            // it has to be at the end of the computations, otherwise you're gonna compute orders
            // for an error of 0. (the default value with which it was initialized)
            self.error_tracker.err_index = self.error_tracker.next_error_idx();
            
            Command {
                // assuming that the precision lost by casting can be ignored/neglected
                forward_velocity: vec_command.x as f32,
                left_velocity: vec_command.y as f32,
                angular_velocity: vec_command.z as f32,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        } else {
            Command::default()
        }

    }
}