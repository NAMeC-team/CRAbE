use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;
use crate::utils::closest_bot_to_point;
use crate::utils::navigation::get_first_angle_free_trajectory;

use crabe_math::shape::Circle;

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

        let cercles = vec![];

        world.allies_bot.iter().filter(|ally_id, _|ally_id != self.id && ally_id != ball_handler.id).for_each(|ally_id, ally|{
            let c = Circle::new(ally.pose.position, world.geometry.robot_radius);
            tools_data.annotations.add_circle(("Ally_",id), circle);
            cercles.push(c);
        });

        world.enemies_bot.iter().for_each(|enemy_id, enemy|{
            let c = Circle::new(enemy.pose.position, world.geometry.robot_radius);
            tools_data.annotations.add_circle(("Enemy_",id), circle);
            cercles.push(c);
        });

        let target = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius, 
            &robot_pos.position, 
            &ball_handler_pos.position,
            true
        );
        let target_2 = get_first_angle_free_trajectory(
            &cercles, 
            world.geometry.robot_radius, 
            &robot_pos.position, 
            &ball_handler_pos.position,
            false
        );
        if target.0 < target_2.0{
            let orientation = vectors::angle_to_point(target.1, ball_handler_pos.position);
            action_wrapper.push(self.id, MoveTo::new(target.1, orientation, 0.0, false, None, false));
        } else {
            let orientation = vectors::angle_to_point(target_2.1, ball_handler_pos.position);
            action_wrapper.push(self.id, MoveTo::new(target_2.1,orientation, 0.0, false, None, false));
        }



        false
    }
}
