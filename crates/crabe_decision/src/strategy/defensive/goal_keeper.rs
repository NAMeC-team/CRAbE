use std::backtrace;

use crate::action::move_to::{self, MoveTo};
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::basics::pass;
use crate::strategy::Strategy;
use crate::utils::{closest_bot_to_point, closest_bots_to_point, object_in_bot_trajectory};
use crabe_framework::data::geometry::Penalty;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{Ball, EnemyInfo, Robot, World};
use crabe_math::{shape::Line, vectors};
use crabe_math::vectors::vector_from_angle;
use crabe_protocol::protobuf::simulation_packet::MoveGlobalVelocity;
use nalgebra::Point2;

/// The GoalKeeper strategy is responsible for keeping the goal safe by moving the robot to the best position to block the ball.
#[derive(Default)]
pub struct GoalKeeper {
    /// The id of the robot to move.
    id: u8,
    ids_to_not_pass: Vec<u8>,
    messages: Vec<MessageData>,
}

impl GoalKeeper {
    /// Creates a new GoalKeeper instance with the desired robot id.
    pub fn new(id: u8, ids_to_not_pass: Vec<u8>) -> Self {
        Self { id, ids_to_not_pass, messages: vec![]}
    }

    /// Calculates the trajectory of the ball based on its velocity.
    /// The trajectory is calculated by extending the ball's position in the direction of its velocity.
    /// If the ball's velocity is too low (less than 0.1), the function returns None.
    fn follow_velocity_trajectory(&self, ball: &Ball, world: &World) -> Option<Point2<f64>> {
        let ball_pos = ball.position_2d();
        let ball_velocity_trajectory = Line::new(ball_pos, ball_pos + ball.velocity.xy().normalize() * 100.);
        if ball.velocity.norm() > 0.01 {
            if let Ok(intersection) = world.geometry.ally_goal.line.intersection_segments(&ball_velocity_trajectory) {
                return Some(intersection);
            }
        }
        None
    }

    /// Calculates the trajectory of the ball based on the enemy's position.
    /// The trajectory is calculated by extending the line from the enemy to the ball's position.
    /// If the trajectory intersects with the goal line, the function returns the intersection point.
    fn follow_enemy_to_ball_trajectory(&self, ball: &Ball, world: &World, enemy: &Robot<EnemyInfo>) -> Option<Point2<f64>> {
        let ball_pos = ball.position_2d();
        let enemy_to_ball = ball_pos - enemy.pose.position;
        let enemy_to_ball_trajectory = Line::new(ball_pos, ball_pos + enemy_to_ball.normalize() * 100.);
        if let Ok(intersection) = world.geometry.ally_goal.line.intersection_segments(&enemy_to_ball_trajectory) {
            return Some(intersection);
        }
        None
    }

    /// Calculates the trajectory of the ball based on the enemy's direction.
    /// The trajectory is calculated by extending the line from the enemy in the direction of its orientation.
    /// If the trajectory intersects with the goal line, the function returns the intersection point.
    fn follow_enemy_direction(&self, world: &World, enemy: &Robot<EnemyInfo>) -> Option<Point2<f64>> {
        let enemy_dir = vector_from_angle(enemy.pose.orientation) * 100.;
        let enemy_dir_trajectory = Line::new(enemy.pose.position, enemy.pose.position + enemy_dir);
        if let Ok(intersection) = world.geometry.ally_goal.line.intersection_segments(&enemy_dir_trajectory) {
            return Some(intersection);
        }
        None
    }
}

impl Strategy for GoalKeeper {

    fn name(&self) -> &'static str {
        return "GoalKeeper";
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
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        // Clean the action wrapper otherwise the previous commands will still have to be runned before the one he will calculate now
        action_wrapper.clear(self.id);

        // Get the GoalKeeper robot, otherwise exit the function
        let robot = match world.allies_bot.get(&self.id) {
            Some(robot) => robot,
            None => return false,
        };
        
        // Set the default position and orientation target
        // - position : the center of the goal
        // - orientation : towards the center of the field
        let mut orientation_target = Point2::new(0., 0.);
        let mut position_target = world.geometry.ally_goal.line.center();
        
        // If the ball is present, the position and orientation have to be updated
        if let Some(ball) = &world.ball{
            let ball_position = ball.position_2d();
            let follow_ball_y_position = Point2::new(world.geometry.ally_goal.line.start.x, ball_position.y);
            let penalty = &world.geometry.ally_penalty;
            orientation_target = ball_position;
            if let Some(intersection) = self.follow_velocity_trajectory(ball, world){
                position_target = intersection;
            } else if ball.velocity.norm() < 0.1 && penalty.is_inside(&ball_position) {
                position_target = ball_position;
                let mut closests_receivers = closest_bots_to_point(world.allies_bot.values().collect(), ball_position);
                closests_receivers.retain(|receiver| receiver.id != self.id && !self.ids_to_not_pass.contains(&receiver.id));
                if closests_receivers.len() == 0{
                    let kick = if robot.distance(&ball_position) < world.geometry.robot_radius + world.geometry.ball_radius -0.01 {
                        Some(Kick::StraightKick { power: 4. })
                    }else{None};
                    action_wrapper.push(robot.id, MoveTo::new(ball_position, vectors::angle_to_point(robot.pose.position, ball_position), 0.0, true, kick, false, false));
                    return false;
                }
                for receiver in closests_receivers.iter(){
                    if object_in_bot_trajectory(world, self.id, receiver.pose.position, false, false, true).len() == 0{
                        let pass_action = pass(robot, receiver, ball, world);
                        action_wrapper.push(self.id, pass_action);
                        return false;
                    } else {
                        action_wrapper.push(robot.id, MoveTo::new(ball_position, vectors::angle_to_point(robot.pose.position, ball_position), 0.0, true, Some(Kick::StraightKick { power: 4. }), false, false));
                        return false;
                    }
                }
            } else if let Some(closest_enemy) = closest_bot_to_point(world.enemies_bot.values().collect(), ball_position){
                if let Some(intersection) = self.follow_enemy_to_ball_trajectory(ball, world, closest_enemy){
                    position_target = intersection;
                } else if let Some(intersection) = self.follow_enemy_direction( world, closest_enemy){
                    position_target = intersection;
                } else{
                    position_target = follow_ball_y_position;
                }
            }else{
                position_target = follow_ball_y_position;
            }
        }

        // Calculate the orientation of the robot towards the orientation target
        let orientation = vectors::angle_to_point(robot.pose.position, orientation_target);

        // clamp the y position of the robot to the goal width so that he's not colliding with the goal walls
        let goal_half_width = world.geometry.ally_goal.width /2.;
        if goal_half_width > world.geometry.robot_radius {
            position_target.y = position_target.y.clamp(-goal_half_width + world.geometry.robot_radius, goal_half_width - world.geometry.robot_radius);
        }

        // Move the robot to the calculated position and orientation
        action_wrapper.push(self.id, MoveTo::new(position_target, orientation, 0., false, None, false, false));
        false
    }

}