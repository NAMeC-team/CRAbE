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
pub struct Aligned {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
}

impl Aligned {
    /// Creates a new Aligned instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids, messages: vec![]}
    }
}

impl Strategy for Aligned {
    fn name(&self) -> &'static str {
        "Aligned"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
        &self.messages
    }   
    fn get_ids(&self) -> Vec<u8> {
        self.ids.clone()
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        self.ids = ids;
    }
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        self.messages.clear();
        self.ids.iter().for_each(|id| {
            action_wrapper.clear(*id);
            let offset = 0.15 * (self.ids.len() as f64);
            let dest = Point2::new((*id as f64) * 0.3 - offset, 2.0);
            action_wrapper.push(
                *id,
                MoveTo::new(dest, -PI / 4.0, 0.0, false, None),
            );
            if *id == 1 {
                match world.allies_bot.get(id) {
                    Some(bot) => {  
                        let bot_position = bot.pose.position;
                        let dist = (bot_position - dest).norm();
                        if dist < 0.1 {
                            self.messages.push(MessageData::new(Message::WantToGoLeft, 1));
                            println!("message length: {:?}", self.messages.len());
                        }
                    }
                    None => {}
                }
            }
        });
        false
    }
}
