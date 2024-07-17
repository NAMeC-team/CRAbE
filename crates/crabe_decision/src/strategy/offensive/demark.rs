use std::any::Any;
use crate::action::{move_to::MoveTo, orient_to::OrientTo};
use crate::action::ActionWrapper;
use crate::strategy::basics::intercept;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crabe_math::vectors;
use nalgebra::Point2;
use std::f64::consts::PI;
use std::fmt::format;
use crate::utils::closest_bot_to_point;
use crate::utils::navigation::get_first_angle_free_trajectory;

use crabe_math::shape::{Circle, Line};

const MIN_DISTANCE_TO_ROBOT_ENEMY: f64 = 0.025;
const EXPLORATION_ANGLE: f64 = 0.01;

/// The Demark struct represents a strategy that commands a robot to move in a Demark shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Demark {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    positive_side: Option<bool>, // NONE IF you don't care , false négative , true positive side
}

impl Demark {
    /// Creates a new Demark instance with the desired robot id.
    pub fn new(id: u8,positive_side:Option<bool>) -> Self {
        Self { id, messages: vec![],positive_side}
    }

    fn get_target_to_demark(&self,action_wrapper: &mut ActionWrapper,side:&bool,ball_handler:&&Robot<AllyInfo>,cercles: &Vec<Circle>,world:&World,robot:&Robot<AllyInfo>) -> Option<Point2<f64>>{

        let ball_handler_pos = &ball_handler.pose;
        let robot_pos= &robot.pose;

        let target_handler_positive = get_first_angle_free_trajectory(
            cercles,
            world.geometry.ball_radius+MIN_DISTANCE_TO_ROBOT_ENEMY,
            &ball_handler_pos.position,
            &robot_pos.position,
            *side,
            robot.distance(&ball_handler_pos.position),
            EXPLORATION_ANGLE
        );

        if target_handler_positive.0 == 0.0 {
            let orientation = vectors::angle_to_point(robot_pos.position, ball_handler_pos.position);
            action_wrapper.push(self.id, OrientTo::new(orientation, 0.0, false, None, true));
            return None;
        }

        let line_handler_positive = Line::new(ball_handler_pos.position, target_handler_positive.1);


        let target_goal_positive = get_first_angle_free_trajectory(
            &cercles,
            world.geometry.ball_radius+MIN_DISTANCE_TO_ROBOT_ENEMY,
            &world.geometry.enemy_goal.line.center(),
            &ball_handler_pos.position,
            !side,
            robot.distance(&world.geometry.enemy_goal.line.center()),
            EXPLORATION_ANGLE
        );


        let line_goal_negative = Line::new(world.geometry.enemy_goal.line.center(), target_goal_positive.1);


        let target1 = match line_handler_positive.intersection_lines(&line_goal_negative){
            Ok(point) => point,
            Err(e) => ball_handler_pos.position
        };
        return Some(target1);

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
        

        // We take the closest enemy to the ball and we calculate the direction of the shot by just looking at his orientation
        let ball_handler =  &match closest_bot_to_point(world.allies_bot.values().collect(), *ball_pos){
            Some(ball_handler) => ball_handler,
            None => {
                return false;
            }
        };

        let enemy_keeper =  &match closest_bot_to_point(world.enemies_bot.values().collect(), world.geometry.enemy_goal.line.center()){
            Some(ball_handler) => ball_handler,
            None => {
                return false;
            }
        };
        

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
            .filter(|(enemy_id,enemy)|*enemy_id != &enemy_keeper.id)
        .for_each(|(enemy_id, enemy)|{
            let c = Circle::new(enemy.pose.position, world.geometry.robot_radius+MIN_DISTANCE_TO_ROBOT_ENEMY);
            cercles.push(c);    
        });

        let ball_trajectory = Line::new(*ball_pos, ball_pos + ball.velocity.xy().normalize() * 100.);
        let try_to_shoot = match world.geometry.enemy_goal.line.intersection_segment_line(&ball_trajectory){
            Ok(res) => true,
            Err(e) => false
        };
        if robot.distance(ball_pos) < 1. && ball_handler.id == self.id && !(try_to_shoot && ball.velocity.norm() > 0.4){
            action_wrapper.push(self.id, intercept(robot, ball));
            return false
        }



        match self.positive_side {
            Some(side) => {

                let target1 = match self.get_target_to_demark(action_wrapper,&side, ball_handler, &cercles, world, robot){
                    Some(p) => p,
                    None => return false
                };
                let orientation = vectors::angle_to_point(target1, ball_handler.pose.position);
                action_wrapper.push(self.id, MoveTo::new(target1, orientation, 0.0, false, None, true));
            },
            None => {
                let target1 = match self.get_target_to_demark(action_wrapper,&true, ball_handler, &cercles, world, robot,){
                    Some(p) => p,
                    None => return false
                };
                let target2 = match self.get_target_to_demark(action_wrapper,&false, ball_handler, &cercles, world, robot,){
                    Some(p) => p,
                    None => return false
                };
        
                if robot.distance(&target1) < robot.distance(&target2){
                    let orientation = vectors::angle_to_point(target1, ball_handler.pose.position);
                    action_wrapper.push(self.id, MoveTo::new(target1, orientation, 0.0, false, None, true));
                } else {
                    let orientation = vectors::angle_to_point(target2, ball_handler.pose.position);
                    action_wrapper.push(self.id, MoveTo::new(target2,orientation, 0.0, false, None, true));
                }
            }
        }



        false
    }
}
