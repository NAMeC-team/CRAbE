use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crate::message::Message;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

#[derive(Default)]
pub struct GoLeft {
    id: u8,
    messages: Vec<MessageData>,
}

impl GoLeft {
    /// Creates a new GoLeft instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for GoLeft {
    fn name(&self) -> &'static str {
        "GoLeft"
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
        let dest = Point2::new(-1.0, 0.0);
        self.messages.clear();
        action_wrapper.push(
            self.id,
            MoveTo::new(dest, -PI / 4.0, 0.0, false, None, true),
        );
        match world.allies_bot.get(&self.id) {
            Some(bot) => {  
                let bot_position = bot.pose.position;
                let dist = (bot_position - dest).norm();
                if dist < 0.1 {
                    self.messages.push(MessageData::new(Message::WantToGoRight, self.id));
                }
            }
            None => {}
        }
        false
    }
}
