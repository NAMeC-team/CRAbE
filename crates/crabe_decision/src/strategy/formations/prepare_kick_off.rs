use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::basics::comeback;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;


/// Strategy prep
#[derive(Default)]
pub struct PrepareKickOff {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
}

impl PrepareKickOff {
    /// Creates a new PrepareKickOff instance
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
            messages: vec![],
        }
    }
}

impl Strategy for PrepareKickOff {
    fn name(&self) -> &'static str {
        "PrepareKickOff"
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
        for id in &self.ids {
            action_wrapper.clear(*id);
            if world.allies_bot.len() >= self.ids.len(){
                let robot = &world.allies_bot[id];
                if robot.pose.position.x > -0.6{
                    action_wrapper.push(*id, comeback(
                        &world.allies_bot[id],
                        world,
                    ));
                }
            }
        }

        false
    }
}
