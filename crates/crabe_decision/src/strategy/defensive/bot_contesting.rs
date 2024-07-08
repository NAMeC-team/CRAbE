use crate::{
    action::{move_to::MoveTo, ActionWrapper}, message::MessageData, strategy::Strategy, utils::closest_bot_to_point
};
use crabe_framework::data::{
    output::{Kick::StraightKick},
    tool::ToolData,
    world::World,
};
use nalgebra::Point2;
use std::{time::Instant, vec};

const ROBOT_RADIUS:f64 = 0.09;
const BALL_RADIUS:f64 = 0.02;
const DISTANCE_TO_BALL:f64 = ROBOT_RADIUS + BALL_RADIUS + 0.06;
const ANGULAR_DIFFERENCE:f64 = 1.0;
/// The BotContesting struct represents a strategy that commands a robot to move in a BotContesting shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct BotContesting {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    time: Instant,

}

fn look_at_target(robot: Point2<f64>, target: Point2<f64>) -> f64 {
    let diff_x = target.x - robot.x;
    let diff_y = target.y - robot.y;
    diff_y.atan2(diff_x)
}

impl BotContesting {
    /// Creates a new BotContesting instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { 
            id,
            messages: vec![],
            time: Instant::now(),    
        }
    }
}

impl Strategy for BotContesting {
    fn name(&self) -> &'static str {
        "BotContesting"
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

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
        
    ) -> bool {
        action_wrapper.clear(self.id);
        let ball: Point2<f64> = match &world.ball {
            Some(b) => b,
            None => {
                eprintln!("Cannot find ball");
                return false;
            }
        }.position_2d();

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false;
            }
        }.pose;
        // We take the closest enemy to the ball and we calculate the direction of the shot by just looking at his orientation
        let enemy =  &match closest_bot_to_point(world.enemies_bot.values().collect(), ball){
            Some(closest_enemy) => closest_enemy,
            None => {
                eprintln!("Cannot get enemy");
                return false;
            }
        }.pose;

        if (ball - enemy.position).norm() > 0.2 {
            eprintln!("Ball is too far to enemy");
            return false;
        }

        let vector = enemy.position - ball;
        let norm = vector.norm();
        let coef = DISTANCE_TO_BALL/norm;
        let target = enemy.position - Point2::new(vector.x, vector.y)*coef;
        let angle = look_at_target(robot.position, enemy.position);

        
        
        if self.time.elapsed().as_millis()%2 == 0{
            action_wrapper.push(self.id,  MoveTo::new(Point2::new(target.x, target.y), angle+ANGULAR_DIFFERENCE, 0.0 , false , Some(StraightKick { power: 0.0 }), false ));
        } else {
             action_wrapper.push(self.id,  MoveTo::new(Point2::new(target.x, target.y), angle-ANGULAR_DIFFERENCE , 0.0 , false , Some(StraightKick { power: 0.0 }), false ));  
        }
        
        
        
        return false;

    }
    
    
    
}
