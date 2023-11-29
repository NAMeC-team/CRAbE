use std::f64::consts::{FRAC_PI_8, PI};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crate::action::Action;
use crate::action::state::State;

/// Proportional factor of the PID controller
const K_P: f64 = 1.;
/// Integral factor of the PID controller
const K_I: f64 = 1.;
/// Derivative factor of the PID controller
const K_D: f64 = 1.;

/// Number of errors to keep track of when computing
/// the integrative value of the PID controller
const PID_NUM_ERRORS: usize = 10;

/// Maximum tolerance for error to be non-zero
/// If the error is inferior to this number, error will be considered 0.
/// The same constants are used to determine whether this action is finished or not.
const TARGET_ATTAINED_TOL: f64 = 0.01;
const THETA_ATTAINED_TOL: f64 = FRAC_PI_8 / 2.;  // in radian !

/// Stores the number of errors for the PID controller
/// used in this movement command
#[derive(Clone)]
struct PIDErrCounter {
    /// Stores the errors in position and angle (rounded to 2 pi)
    /// xy store the position error, and z is used to store the angle error
    errors: Vec<Vector3<f64>>,
    /// Maximum number of errors to keep
    /// This is also the size of the errors array
    max_size: usize,
    /// Current error index. We use it to overwrite the errors over time instead of
    /// shifting the array one by one
    err_index: usize
}

impl PIDErrCounter {
    fn current(&self) -> Vector3<f64> { self.errors[self.err_index] }
    fn sum(&self) -> Vector3<f64> {
        self.errors.iter().sum()
    }

    /// Replaces the error at the current index with
    /// the one passed in parameter.
    fn save(&mut self, err_value: Vector3<f64>) {
        // vec was filled with 0. on init
        // this unwrap() shouldn't panic (in theory)
        let old_err = self.errors.get_mut(self.err_index).unwrap();
        *old_err = err_value;
        self.err_index = (self.err_index + 1) % self.max_size;
    }

    /// Computes the error sum between the previous and
    /// the current error.
    /// Similar to the derivative term of the movement of the robot.
    /// This must be called after computing the current position error for the robot
    fn deriv_prev_curr(&self) -> Vector3<f64> {
        let idx_prev = ((self.err_index as i16 - 1).rem_euclid(self.max_size as i16)) as usize ;
        self.errors[idx_prev] - self.errors[self.err_index]
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
    /// Accumulation of the errors computer over time
    error_tracker: PIDErrCounter,
}

impl MoveToPID {
    pub fn new(target: Point2<f64>, orientation: f64) -> Self {
        let mut zero_vec: Vec<Vector3<f64>> = Vec::new();
        zero_vec.fill(Vector3::new(0., 0., 0.));

        Self {
            state: State::Running,
            target,
            orientation,
            error_tracker: PIDErrCounter {
                errors: vec![Vector3::new(0., 0., 0.); PID_NUM_ERRORS],
                max_size: PID_NUM_ERRORS,
                err_index: 0,
            }
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
        let err_theta= target_orientation - robot.pose.orientation;

        // dbg!(&pos_err.y);

        // consider error is 0. if is it superior to max tolerance
        computed_err.x = if pos_err.x.abs() > TARGET_ATTAINED_TOL { pos_err.x } else { 0. };
        computed_err.y = if pos_err.y.abs() > TARGET_ATTAINED_TOL { pos_err.y } else { 0. };
        // computed_err.z = if err_theta.abs() > THETA_ATTAINED_TOL { err_theta } else { 0. };

        // dbg!(&computed_err.y);

        computed_err
    }

    fn angle_wrap(&self, alpha: f64) -> f64 { (alpha + PI) % (2.0 * PI) - PI }
}

impl Action for MoveToPID {
    fn name(&self) -> String {
        String::from("MoveToPID")
    }

    fn state(&mut self) -> State { self.state }

    fn compute_order(&mut self, id: u8, world: &World, _tools: &mut ToolData) -> Command {
        if let Some(robot) = world.allies_bot.get(&id) {
            // take current error in account for next command
            let current_error = self.error_to_target(robot, self.target, self.orientation);
            self.error_tracker.save(current_error);

            if current_error.norm() == 0. {
                return Command::default();
            }

            // compute in order, the factors of the PID
            let p =  K_P * self.error_tracker.current();
            let i = K_I * self.error_tracker.sum();
            let d = K_D * self.error_tracker.deriv_prev_curr();

            let vec_command: Vector3<f64> = p + i + d;

            Command {
                // assuming that the precision lost by casting can be ignored/neglected
                forward_velocity: vec_command.x as f32,
                left_velocity: vec_command.y as f32,
                angular_velocity: 0. as f32,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        } else {
            Command::default()
        }
    }
}