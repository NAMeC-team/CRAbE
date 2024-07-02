use std::ops::Mul;
use std::ops::Add;
use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::Strategy;
use crate::utils::closest_bot_to_point;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World};
use crabe_math::shape::Line;
use crabe_math::vectors::{vector_from_angle, self};
use nalgebra::{Point2, clamp};

/// The GoalKeeper strategy is responsible for keeping the goal safe by moving the robot to the best position to block the ball.
#[derive(Default)]
pub struct GoalKeeper {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
}

impl GoalKeeper {
    /// Creates a new GoalKeeper instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
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
            let ball_pos = ball.position_2d();
            orientation_target = ball_pos;
            let mut enemy_shoot_dir = Line::new(ball_pos,Point2::new(-10.,ball.position.y));
        
            // We take the closest enemy to the ball and we calculate the direction of the shot by just looking at his orientation
            if let Some(closest_enemy) = closest_bot_to_point(world.enemies_bot.values().collect(), ball_pos){
                let enemy_dir = vector_from_angle(closest_enemy.pose.orientation) * 10.;
                enemy_shoot_dir.end = ball_pos + enemy_dir;
            }

            // If the shoot direction intersect with the goal line, we move the robot to the intersection point
            if let Ok(intersection) = world.geometry.ally_goal.line.intersection_segments(&enemy_shoot_dir) {
                position_target = intersection;
            } else { // Otherwise we move the robot to the closest point of the goal line to the ball (resulting in following the y axis of the ball)
                position_target = world.geometry.ally_goal.line.closest_point_on_segment(&ball_pos);
            }
        }

        // Calculate the orientation of the robot towards the orientation target
        let orientation = vectors::angle_to_point(robot.pose.position, orientation_target);

        // Move the robot to the calculated position and orientation
        action_wrapper.push(self.id, MoveTo::new(position_target, orientation, 0., false, None));
        false
    }

}