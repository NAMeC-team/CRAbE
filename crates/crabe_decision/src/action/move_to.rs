use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{distance, Isometry2, Point2, Vector2, Vector3};
use std::f64::consts::PI;

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
            const ATTRACTIVE_COEFFICIENT: f64 = 1.0;
            const OBSTACLE_RADIUS: f64 = 0.5;
            const REPULSIVE_COEFFICIENT: f64 = 1.0;

            // let distance_goal = 1;
            // let distance_obstacle = 2;

            let mut result = Vector2::new(0.0, 0.0);

            // Attractive field
            // let attract_force = self.target - robot.pose.position;
            let dist_target_robot = distance(&self.target, &robot.pose.position);

            let force_magnitude = dist_target_robot * ATTRACTIVE_COEFFICIENT;
            let target_robot = self.target - robot.pose.position;

            let attractive_force = (target_robot / dist_target_robot) * force_magnitude;

            result += attractive_force;

            // Repulsive field

            if self.avoid_ball {
                println!("[TODO] : AVOID BALL")
            }

            world.allies_bot.iter().for_each(|(id, ally)| {
                // Our robot id is not an obstacle
                if robot.id == *id {
                    return;
                }

                let dist_ally_robot = distance(&robot.pose.position, &ally.pose.position);

                if dist_ally_robot < OBSTACLE_RADIUS {
                    let force_magnitude = dist_ally_robot * REPULSIVE_COEFFICIENT;
                    let ally_robot = ally.pose.position - robot.pose.position;

                    let repulsive_force = (ally_robot / dist_ally_robot) * force_magnitude;
                    result -= repulsive_force;
                }
            });

            world.enemies_bot.iter().for_each(|(_, enemy)| {
                let dist_enemy_robot = distance(&robot.pose.position, &enemy.pose.position);
                dbg!(dist_enemy_robot);

                if dist_enemy_robot < OBSTACLE_RADIUS {
                    dbg!(dist_enemy_robot);
                    let force_magnitude = dist_enemy_robot * REPULSIVE_COEFFICIENT;
                    let enemy_robot = enemy.pose.position - robot.pose.position;

                    let repulsive_force = (enemy_robot / dist_enemy_robot) * force_magnitude;
                    result -= repulsive_force;
                }
            });

            result *= 1.0;

            Command {
                forward_velocity: result.x as f32,
                left_velocity: result.y as f32,
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
