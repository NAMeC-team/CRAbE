use crate::{
    action::{move_to::MoveTo, ActionWrapper}, message::MessageData, strategy::Strategy
};
use crabe_framework::data::{
    output::Kick::StraightKick,
    tool::ToolData,
    world::World,
};
use nalgebra::{Matrix, Point2};

use crabe_math::{shape::Line, vectors::angle_to_point};


/// The BotMarking struct represents a strategy that commands a robot to move in a BotMarking shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct BotMarking {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    enemy_id: u8,
}

impl BotMarking {
    /// Creates a new BotMarking instance with the desired robot id.
    pub fn new(id: u8, enemy_id: u8) -> Self {
        Self { 
            id,
            messages: vec![],
            enemy_id,
        }
    }
}

impl Strategy for BotMarking {
    fn name(&self) -> &'static str {
        "BotMarking"
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
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                return false;
            }
        };

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                return false;
            }
        };
        let robot_pos = &robot.pose;

        let enemy = &match world.enemies_bot.get(&self.enemy_id) {
            Some(r) => r,
            None => {
                return false;
            }
        };
        let enemy_pos = &enemy.pose;

        let mut dribbler = 0.0;
        let ball_pos = ball.position_2d();
        if robot.distance(&ball_pos) < 1. {
            dribbler = 1.0;
        }

        //ANGLE TO BALL
        let angle = angle_to_point(robot_pos.position, ball_pos);

        // VELOCITY CATCH
        let ball_velocity_trajectory = Line::new(ball_pos, ball_pos + ball.velocity.xy().normalize() * 100.);
        if ball.velocity.norm() > 0.1 && ball_velocity_trajectory.distance_to_point(&enemy_pos.position) < 1. {
            let target = ball_velocity_trajectory.closest_point_on_segment(&robot_pos.position);
            action_wrapper.push(self.id,  MoveTo::new(Point2::new(target.x, target.y), angle , dribbler , false , Some(StraightKick { power: 0.0 }), true, true));
        } else {

            // VECTEUR BALL -> ENEMY
            let enemy_to_ball = ball_pos - enemy_pos.position;
            let enemy_ball_distance = enemy_to_ball.norm();
            let coef_distance_to_enemy: f64 = world.geometry.robot_radius + 0.2/enemy_ball_distance;
            let target = enemy_pos.position -  Point2::new(enemy_to_ball.x, enemy_to_ball.y)*(-coef_distance_to_enemy);
            action_wrapper.push(self.id,  MoveTo::new(Point2::new(target.x, target.y), angle , dribbler , false , Some(StraightKick { power: 0.0 }), false, true));
        }

        
        
        return false;

    }
    
}
