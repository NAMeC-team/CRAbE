use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crate::utils::object_in_bot_trajectory;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::angle_to_point;
use crate::strategy::basics::move_away;
use nalgebra::Point2;
use std::f64::consts::PI;


/// The LateralAttack struct represents a strategy that commands a robot to move in a LateralAttack shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct LateralAttack {
    /// The id of the robot to move.
    id : u8,
    passer_id: u8,
    messages: Vec<MessageData>,
}

impl LateralAttack {
    /// Creates a new LateralAttack instance with the desired robot id.
    pub fn new(id: u8, passer_id: u8) -> Self {
        Self {id, passer_id, messages: vec![]}
    }
}

impl Strategy for LateralAttack {
    fn name(&self) -> &'static str {
        "LateralAttack"
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

    fn step(
        &mut self,
        world: &World,
        _tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);
        let robot = match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                return false;
            }
        };

        let passer = match world.allies_bot.get(&self.passer_id) {
            Some(r) => r,
            None => {
                return false;
            }
        };

        let mut target : nalgebra::OPoint<f64, nalgebra::Const<2>>;

        if dbg!(passer.pose.position.y.signum()) * (passer.pose.position.x - 1.5).signum()  < 0. {
            target = Point2::new(3.5, 2.);
            if object_in_bot_trajectory(world, self.id, passer.pose.position, false, false, true).len() > 0 {
                target = Point2::new(3.5, 1.5);
            }
            action_wrapper.push(self.id, MoveTo::new(target, angle_to_point(robot.pose.position, passer.pose.position), 0.0, false, None, true));
            return false;
        } else {
            target = Point2::new(3.5, -2.);
            if object_in_bot_trajectory(world, self.id, passer.pose.position, false, false, true).len() > 0 {
                target = Point2::new(3.5, -1.5);
            }
            action_wrapper.push(self.id, MoveTo::new(target, angle_to_point(robot.pose.position, passer.pose.position), 0.0, false, None, true));
            return false;
        } 

    }
}
