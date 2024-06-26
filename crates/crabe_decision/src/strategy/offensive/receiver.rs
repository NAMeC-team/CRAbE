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
pub struct Receiver {
    id: u8,
    messages: Vec<MessageData>,
}

impl Receiver {
    /// Creates a new Receiver instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for Receiver {
    fn name(&self) -> &'static str {
        "Receiver"
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
        action_wrapper.clear_all();
        self.messages.clear();
        action_wrapper.push(
            self.id,
            MoveTo::new(Point2::new(1.0, -1.0), -PI / 4.0, 0.0, false, None),
        );
        let bot_position = world.allies_bot.get(&self.id).unwrap().pose.position;
        // if (bot_position - Point2::new(1.0, -1.0)).norm() < 0.1 {
        //     self.messages.push(MessageData::new(Message::SearchingReciever, self.id));
        // }
        false
    }
}
