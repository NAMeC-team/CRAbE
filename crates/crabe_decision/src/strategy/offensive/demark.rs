use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;
use crate::utils::closest_bot_to_point;
use crate::utils::navigation::get_first_angle_free_trajectory;

use crabe_math::shape::{Circle, Line};

const MIN_DISTANCE_TO_ROBOT_ENEMY: f64 = 0.05;

/// The Demark struct represents a strategy that commands a robot to move in a Demark shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Demark {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
}

impl Demark {
    /// Creates a new Demark instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for Demark {
    fn name(&self) -> &'static str {
        "Demark"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
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

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);

        let ball = match &world.ball {
            Some(ball) => ball,
            None => return false,
        };
        let ball_pos = &ball.position_2d();

        let robot = match world.allies_bot.get(&self.id) {
            Some(robot) => robot,
            None => return false,
        };
        let robot_pos = &robot.pose;

        // We take the closest enemy to the ball and we calculate the direction of the shot by just looking at his orientation
        let ball_handler =  &match closest_bot_to_point(world.allies_bot.values().collect(), *ball_pos){
            Some(ball_handler) => ball_handler,
            None => {
                return false;
            }
        };
        let ball_handler_pos = &ball_handler.pose;

        let mut cercles: Vec<Circle> = vec![];

        world.allies_bot
        .iter()
        .filter(|(ally_id, _)|*ally_id != &self.id && *ally_id != &ball_handler.id)
        .for_each(|(ally_id, ally)|{
            let c = Circle::new(ally.pose.position, world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY);
            cercles.push(c);
        });

        world.enemies_bot
        .iter()
        .for_each(|(enemy_id, enemy)|{
            let c = Circle::new(enemy.pose.position, world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY);
            cercles.push(c);    
        });

        let target_handler_positive = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY, 
            &ball_handler_pos.position, 
            &robot_pos.position,
            true,
            robot.distance(&ball_handler_pos.position)
        );

        if target_handler_positive.0 == 0.0 {
            return false;
        }

        let line_handler_positive = Line::new(ball_handler_pos.position, target_handler_positive.1);
        

        let target_goal_positive = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY, 
            &world.geometry.enemy_goal.line.center(),
            &ball_handler_pos.position, 
            false,
            robot.distance(&ball_handler_pos.position)
        );


        let line_goal_negative = Line::new(world.geometry.enemy_goal.line.center(), target_goal_positive.1);
        
        let target1 = match line_handler_positive.intersection_lines(&line_goal_negative){
            Ok(point) => point,
            Err(e) => ball_handler_pos.position
        };

        let target_handler_negative = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY, 
            &ball_handler_pos.position,
            &robot_pos.position, 
            false,
            robot.distance(&ball_handler_pos.position)
        );

        let line_handler_negative = Line::new(ball_handler_pos.position, target_handler_negative.1);
        


        let target_goal_positive = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY, 
            &world.geometry.enemy_goal.line.center(),
            &ball_handler_pos.position, 
            true,
            robot.distance(&ball_handler_pos.position)
        );
        
        let line_goal_positive = Line::new(world.geometry.enemy_goal.line.center(), target_goal_positive.1);
        

        let target2 = match line_handler_negative.intersection_lines(&line_goal_positive){
            Ok(point) => point,
            Err(e) => ball_handler_pos.position
        };

        
        if robot.distance(&target1) < robot.distance(&target2){
            let orientation = vectors::angle_to_point(target1, ball_handler_pos.position);
            action_wrapper.push(self.id, MoveTo::new(target1, orientation, 0.0, false, None, true));
        } else {
            let orientation = vectors::angle_to_point(target2, ball_handler_pos.position);
            action_wrapper.push(self.id, MoveTo::new(target2,orientation, 0.0, false, None, true));
        }


        false
    }
}
