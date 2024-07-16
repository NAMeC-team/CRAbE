use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::strategy::basics::move_away;
use nalgebra::Point2;
use std::f64::consts::PI;

const DISTANCE_TO_BALL:f64 = 0.8;

/// The MoveAwayFromBall struct represents a strategy that commands a robot to move in a MoveAwayFromBall shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct MoveAwayFromBall {
    /// The id of the robot to move.
    ids : Vec<u8>,
    messages: Vec<MessageData>,
}

impl MoveAwayFromBall {
    /// Creates a new MoveAwayFromBall instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self {ids, messages: vec![]}
    }
}

impl Strategy for MoveAwayFromBall {
    fn name(&self) -> &'static str {
        "MoveAwayFromBall"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        vec![]
        
    }
    fn put_ids(&mut self, ids: Vec<u8>) { }

    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                return false;
            }
        };
        world.allies_bot.iter()
            .filter(|(ally_id, _)| self.ids.contains(ally_id))
            .for_each(|(ally_id, ally_info)| {
                action_wrapper.clear(*ally_id);
                let robot_pos = ally_info.pose.position;
                let move_to = match move_away(robot_pos, ball.position_2d(),DISTANCE_TO_BALL){
                    Some(m) => {
                        action_wrapper.push(
                            *ally_id,
                            m,
                        );
                    },
                    None => {},
                };
                
        });
        false
    }
}
