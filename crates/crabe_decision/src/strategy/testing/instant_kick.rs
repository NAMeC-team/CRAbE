use crate::action::move_to_builder::MoveToBuilder;
use crate::action::ActionWrapper;
use crate::strategy::basics::{intercept_instant_goal, pass};
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::angle_to_point;
use nalgebra::Point2;

pub struct InstantKick {
    id_attacker: u8,
    id_receiver: u8,
}

impl InstantKick {
    /// Creates a new InstantKick instance with the desired robot id.
    pub fn new(id_attacker: u8, id_receiver: u8) -> Self {
        Self {
            id_attacker,
            id_receiver,
        }
    }
}

impl Strategy for InstantKick {
    fn name(&self) -> &'static str {
        "InstantKick"
    }
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        //self.messages.clear();
        action_wrapper.clear(self.id_attacker);
        action_wrapper.clear(self.id_receiver);
        let ball = if let Some(ball) = &world.ball {
            ball
        } else {
            return false;
        };
        let attacker = if let Some(attacker) = world.allies_bot.get(&self.id_attacker) {
            attacker
        } else {
            return false;
        };
        let receiver = if let Some(receiver) = world.allies_bot.get(&self.id_receiver) {
            receiver
        } else {
            return false;
        };
        let receiver_target_pos_y = world.geometry.enemy_penalty.front_line.start.y+0.5;
        let receiver_target_pos_x = world.geometry.enemy_penalty.front_line.start.x;
        let receiver_target_pos = Point2::new(receiver_target_pos_x, receiver_target_pos_y);
        let goal_center = world.geometry.enemy_goal.line.center();

        //vector from goal to attacker
        let goal_to_attacker = attacker.pose.position - goal_center;
        let middle_goal_to_attacker = goal_center + goal_to_attacker.normalize() * 0.5;
        action_wrapper.push(
            self.id_attacker,
            pass(attacker, receiver, ball, world),
        );
        if ball.velocity.norm() > 0.3{
            action_wrapper.push(
                self.id_receiver,
                intercept_instant_goal(world, receiver, ball)
            );
        }else{
            action_wrapper.push(
                self.id_receiver,
                MoveToBuilder::new().set_target(receiver_target_pos).set_orientation(angle_to_point(receiver.pose.position, ball.position_2d())).build()
            );
        }


        false
    }
}