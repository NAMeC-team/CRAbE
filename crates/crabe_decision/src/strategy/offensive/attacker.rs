use std::f64::consts::PI;
use crate::strategy::basics::shoot;
use crate::strategy::basics::intercept;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::{shape::Line, vectors::rotate_vector};
use nalgebra::Point2;

const MARGIN_SHOOTING_WINDOW: f64 = 0.01;

/// The Attacker strategy is responsible for moving the robot to the ball and then try scoring a goal
pub struct Attacker {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
}

/// Get the obstruct goal zone from an enemy
/// 
/// # Arguments
/// - `shoot_start_position`: The position of the object that want to go into the goal
/// - `enemy_position`: The position of the enemy
/// - `world`: The current state of the game world
/// 
/// # Returns
/// A `Line` representing the obstruct goal zone
fn get_obstruct_goal_zone_from_enemy(shoot_start_position: &Point2<f64>, enemy_position: &Point2<f64>, world: &World) -> Option<Line> {
    let start_pos_to_enemy = enemy_position - shoot_start_position;
    let perp = rotate_vector(start_pos_to_enemy.normalize(), PI/2.) * (world.geometry.robot_radius + world.geometry.ball_radius + MARGIN_SHOOTING_WINDOW);
    let ray_left_side = (enemy_position + perp) - shoot_start_position;
    let ray_right_side = (enemy_position - perp) - shoot_start_position;
    let line_ray_left_side = Line::new(*shoot_start_position, shoot_start_position + ray_left_side * 1000.);
    let line_ray_right_side = Line::new(*shoot_start_position, shoot_start_position + ray_right_side * 1000.);
    let intersection_left = world.geometry.enemy_goal.line.intersection_segments(&line_ray_left_side);
    let intersection_right = world.geometry.enemy_goal.line.intersection_segments(&line_ray_right_side);
    match (intersection_left, intersection_right) {
        (Ok(left), Ok(right)) => Some(Line::new(left, right)),
        (Ok(left), _) => Some(Line::new(left, world.geometry.enemy_goal.line.end)),
        (_, Ok(right)) => Some(Line::new(world.geometry.enemy_goal.line.start, right)),
        _ => None,
    }
}

/// Get the open shoot window for the attacker
/// 
/// # Arguments
/// - `shoot_start_position`: The position of the object that want to go into the goal
/// - `world`: The current state of the game world
/// 
/// # Returns
/// A vector of `Line` representing the open shoot windows
pub fn get_open_shoot_window(shoot_start_position: &Point2<f64>, world: &World) -> Vec<Line> {
    let mut available_targets: Vec<Line> = vec![world.geometry.enemy_goal.line];
    for enemy in world.enemies_bot.values() {
        if let Some(line) = get_obstruct_goal_zone_from_enemy(shoot_start_position, &enemy.pose.position.xy(), world){
            let mut new_targets: Vec<Line> = vec![];
            for target_line in available_targets {
                let targets = target_line.cut_off_segment(&line);
                new_targets.extend(targets);
            }
            available_targets = new_targets;
        }
    }
    return available_targets;
}
impl Attacker {
    /// Creates a new Attacker instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for Attacker {

    fn name(&self) -> &'static str {
        return "Attacker";
    }
    
    fn get_messages(&self) -> &Vec<MessageData>  {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        vec![self.id]
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        if ids.len() == 1{
            self.id = ids[0];
        }
    }

    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        // Clean the action wrapper otherwise the previous commands will still have to be runned before the one he will calculate now
        action_wrapper.clear(self.id);
        // Get the Attacker robot, otherwise exit the function
        let robot = match world.allies_bot.get(&self.id) {
            Some(robot) => robot,
            None => return false,
        };
        let robot_position = robot.pose.position;
        
        // Get the ball position, otherwise exit the function
        let ball = match &world.ball {
            Some(ball) => ball,
            None => return false,
        };
        
        // If the ball is moving in the direction of our goal, intercept it
        if ball.velocity.norm() > 1. && ball.velocity.x < 0. {
            action_wrapper.push(self.id, intercept(
                &robot,
                &ball,
            ));
            return false;
        }

        let shoot_windows = get_open_shoot_window(&ball.position_2d(), world);
        for line in &shoot_windows{
            tools_data.annotations.add_line(line.start.to_string(), *line);
        }

        let biggest_shoot_window = shoot_windows.iter().reduce(|curr, x: &Line| if curr.norm() > x.norm() {curr} else {x});
        let target_shooting_position = match biggest_shoot_window{
            Some(shoot_window) => shoot_window.center(),
            None => world.geometry.enemy_goal.line.center(), // If there is no shoot window, shoot in the middle of the goal
        };
        tools_data.annotations.add_point("Target".to_string(), target_shooting_position);
        action_wrapper.push(self.id, shoot(robot, &ball, &target_shooting_position, world));
        false
    }

}