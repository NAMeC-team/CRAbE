use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{Const, distance, Dyn, IsDynamic, Isometry2, Matrix, matrix, Matrix2, Matrix2x1, OMatrix, Point2, U1, U2, Vector2, Vector3};
use std::f64::consts::PI;
use std::ops::{Div, Mul};

/// The `MoveTo` struct represents an action that moves the robot to a specific location on the field, with a given target orientation.
#[derive(Clone)]
pub struct MoveTo {
    /// The current state of the action.
    state: State,
    /// The target position to move to.
    target: Point2<f64>,
    /// The target orientation of the robot.
    orientation: f64,
    /// Avoid the ball
    avoid_ball: bool,
}

impl From<&mut MoveTo> for MoveTo {
    fn from(other: &mut MoveTo) -> MoveTo {
        MoveTo {
            state: other.state,
            target: other.target,
            orientation: other.orientation,
            avoid_ball: other.avoid_ball,
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
    /// * `avoid_ball`
    pub fn new(target: Point2<f64>, orientation: f64, avoid_ball: bool) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
            avoid_ball,
        }
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
            const K_A: f64 = 1.0; // Attractive coefficient
            const OBSTACLE_RADIUS: f64 = 0.5;
            const K_B: f64 = 1.0; // Repulsive coefficient

            let mut f = Vector2::new(0.0, 0.0);

            // Attractive field
            // q  : Coordinates of the robot
            // qd : Coordinates of the target
            let q = robot.pose.position;
            let q_d = self.target;

            // implementation of (4) in paper
            let f_att = -K_A * (q - q_d);
            // let f_att = K_A * (q_d - q);
            f += f_att;


            // Repulsive field
            if self.avoid_ball {
                println!("[TODO] : AVOID BALL")
            }

            let d_0 = OBSTACLE_RADIUS;
            let mut repulsive_strength_sum = Vector2::new(0.0, 0.0);//OMatrix::new::<f64, U2, Dyn>();
            world.allies_bot.iter().for_each(|(id, ally)| {
                // Our robot id is not an obstacle
                if robot.id == *id {
                    return;
                }

                // Location of the ally obstacle
                let q_c = &ally.pose.position;
                // Distance from our robot and the ally obstacle
                let d_q = distance(&q, q_c);

                if d_q < OBSTACLE_RADIUS {

                    // implementation of (8) in the paper
                    let repulsive_force = K_B *
                        (1.0.div(d_q) - 1.0.div(d_0))
                        *
                        (1.0.div(d_q.powi(2)))
                        *
                        ((q-q_c).div(distance(&q, q_c)))
                    ;
                    repulsive_strength_sum += repulsive_force;
                }
            });

            world.enemies_bot.iter().for_each(|(_, enemy)| {
                // Location of the ally obstacle
                let q_c = &enemy.pose.position;
                // Distance from our robot and the ally obstacle
                let d_q = distance(&q, q_c);

                if d_q < OBSTACLE_RADIUS {

                    // implementation of (8) in the paper
                    let repulsive_force = K_B *
                        (1.0.div(d_q) - 1.0.div(d_0))
                        *
                        (1.0.div(d_q.powi(2)))
                        *
                        ((q-q_c).div(distance(&q, q_c)))
                        ;
                    repulsive_strength_sum += repulsive_force;
                }

                // let dist_enemy_robot = distance(&robot.pose.position, &enemy.pose.position);
                // dbg!(dist_enemy_robot);
                //
                // if dist_enemy_robot < OBSTACLE_RADIUS {
                //     dbg!(dist_enemy_robot);
                //     let force_magnitude =  K_B / dist_enemy_robot;
                //     let enemy_robot = enemy.pose.position - robot.pose.position;
                //
                //     let repulsive_force = (enemy_robot / dist_enemy_robot) * force_magnitude;
                //     f -= repulsive_force;
                // }

            });

            f += dbg!(repulsive_strength_sum);

            Command {
                forward_velocity: f.x as f32,
                left_velocity: f.y as f32,
                angular_velocity: 0.0,
                charge: false,
                kick: None,
                dribbler: 0.0,
            }
        } else {
            Command::default()
        }
    }
}
