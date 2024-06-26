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
pub struct Defensor {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
}

impl Defensor {
    /// Creates a new Defensor instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids, messages: vec![]}
    }
}

impl Strategy for Defensor {
    fn name(&self) -> &'static str {
        "Defensor"
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
        self.ids.iter().for_each(|id| {
            action_wrapper.clear(*id);
        });
        self.messages.clear();
        self.ids.iter().enumerate().for_each(|(i, id)| {
            action_wrapper.push(
                *id,
                MoveTo::new(Point2::new((i as f64) *0.5, 2.0), -PI / 4.0, 0.0, false, None),
            );
        });
        false
    }
}
