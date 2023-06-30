use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{RobotMap, World};
use nalgebra::{distance, Matrix, matrix, Matrix2, Matrix2x1, min, OMatrix, Point2, Rotation2, U1, U2, Vector2, Vector3};
use std::ops::{Div};

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct MoveTo {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,
    /// Dribble strength
    dribble: f32,
    /// Avoid the ball
    avoid_ball: bool,
    /// Attraction factor
    k_attraction: f64,
    /// Repulsion factor
    k_repulsion: f64,
    /// Set to true if we should charge the capacitors for kicking when being near target
    chg_near_arrival: bool,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            dribble: other.dribble,
            avoid_ball: other.avoid_ball,
            k_attraction: other.k_attraction,
            k_repulsion: other.k_repulsion,
            chg_near_arrival: other.chg_near_arrival,
        }
    }
}

impl MoveTo {
    /// Creates a new `MoveTo` instance, avoiding any obstacles in the way.
    /// Speed is limited by normalizing the movement vector
    /// Based on this paper : https://www.researchgate.net/publication/313389747_Potential_field_methods_and_their_inherent_approaches_for_path_planning
    ///
    /// # Arguments
    ///
    /// * `target` : The target position on the field to move the robot to.
    /// * `orientation` : The target orientation of the robot.
    /// * `avoid_ball` : Set to true to make the MoveTo avoid the ball as well as the other robots
    /// * `charge_when_near_target` : Set to true to charge the kickers when we're near the target (about 0.3 meter)
    pub fn new(target: Point2<f64>, orientation: f64, dribble: f32, avoid_ball: bool, charge_when_near_target: bool) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            avoid_ball,
            dribble,
            k_attraction: 1.0,
            k_repulsion: 1.0,
            chg_near_arrival: charge_when_near_target,
        }
    }

    /// Computes the attractive force of the goal target to attain
    /// using the formula from the paper.
    ///
    /// # Arguments
    ///
    /// * `q` : The robot's vector position (or coordinates)
    /// * `q_d` : The target's vector position (or coordinates)
    fn attractive_force(&self, q: &Point2<f64>, q_d: &Point2<f64>) -> Vector2<f64> {
        -&self.k_attraction * (q - q_d)
    }

    /// Computes the repulsive force generated by an obstacle,
    /// using the formula from the paper.
    ///
    /// # Arguments
    ///
    /// * `d_0` : Constant, radius of the obstacle
    /// * `d_q` : Euclidean distance between the robot and the obstacle
    /// * `q`   : The robot's vector position (or coordinates)
    /// * `q_c` : The obstacle's vector position (or coordinates)
    fn repulsive_force(&self, d_0: &f64, d_q: &f64, q: &Point2<f64>, q_c: &Point2<f64>) -> Vector2<f64> {
        self.k_repulsion *
        (1.0.div(d_q) - 1.0.div(d_0)) *
        (1.0.div(d_q.powi(2))) *
        ((q-q_c).div(distance(&q, q_c)))
    }

    /// Computes the angular speed required to adjust the robot's orientation to the required orientation
    ///
    /// # Arguments
    ///
    /// * `robot_theta` : The current orientation of the robot
    fn angular_speed(&self, robot_theta: &f64) -> f32 {
        let mut angular_accel_sign: f32 = 1.;

        let angle_diff = self.orientation - robot_theta;
        if angle_diff.abs() < 0.2 {
            angular_accel_sign = 0.;
        }
        else if angle_diff < 0. {
            angular_accel_sign = -1.;
        }

        // apply a factor of 5 to increase
        angular_accel_sign * angle_diff.abs() as f32 * 5.0
    }
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
            const OBSTACLE_RADIUS: f64 = 0.4;

            let d_0 = OBSTACLE_RADIUS;

            // Resulting movement vector
            let mut f = Vector2::new(0.0, 0.0);

            // Coordinates of the robot
            let q = robot.pose.position;
            // Coordinates of the target
            let q_d = self.target;

            // -- Attractive field
            f += self.attractive_force(&q, &q_d);

            // -- Repulsive field

            // Don't compute any repulsion if robot is already near target
            if distance(&robot.pose.position, &self.target) >= 0.15 {
                let mut repulsive_strength_sum = Vector2::new(0.0, 0.0);
                world.allies_bot.iter().for_each(|(id, ally)| {
                    // Our robot id is not an obstacle
                    if robot.id == *id {
                        return;
                    }

                    let d_q = distance(&robot.pose.position, &ally.pose.position);

                    if d_q < d_0 {
                        repulsive_strength_sum += self.repulsive_force(&d_0, &d_q, &q, &ally.pose.position);
                    }
                });

                world.enemies_bot.iter().for_each(|(_, enemy)| {
                    // Distance from our robot and the ally obstacle
                    let d_q = distance(&robot.pose.position, &enemy.pose.position);

                    if d_q < d_0 {
                        repulsive_strength_sum += self.repulsive_force(&d_0, &d_q, &q, &enemy.pose.position);
                    }
                });

                // avoid ball if tasked
                if self.avoid_ball {
                    if let Some(ball) = &world.ball {
                        let ball_position = &ball.position.xy();
                        if distance(ball_position, &robot.pose.position) <= d_0 {
                            let d_q = distance(&robot.pose.position, ball_position);
                            repulsive_strength_sum += self.repulsive_force(&d_0, &d_q, &q, ball_position);
                        }
                    }
                }

                f += dbg!(repulsive_strength_sum);
            }

            // -- Normalizing the strength vector to avoid super Sonic speed
            //    but only if not close to target, otherwise leads to oscillation
            if distance(&q, &q_d) > 1.0 {
                f = f.normalize();
            }

            // -- Compute angle of the resulting vector
            let angular_velocity = self.angular_speed(&robot.pose.orientation);

            // -- Change the basis of the resulting vector to the basis of the robot
            //    I'm not exactly sure why it's `-robot_theta` and not `robot_theta`
            let rob_rotation_basis = Rotation2::new(-&robot.pose.orientation);
            // println!("Before transformation : {}", &f);
            f = rob_rotation_basis * f;
            // println!("After transformation : {}", &f);

            // -- Determine whether we need to charge
            let mut charge = self.chg_near_arrival && distance(&robot.pose.position, &self.target) <= 0.3;

            Command {
                forward_velocity: f.x as f32,
                left_velocity: f.y as f32,
                angular_velocity,
                charge,
                kick: None,
                dribbler: self.dribble,
            }
        } else {
            Command::default()
        }
    }
}
