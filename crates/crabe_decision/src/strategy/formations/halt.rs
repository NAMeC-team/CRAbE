use crate::action::order_raw::RawOrder;
use crate::action::ActionWrapper;

use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_framework::data::output::Command;


/// Strategy to stop the robots (sending Command with 0 movements)
#[derive(Default)]
pub struct Halt {
    ids: Vec<u8>,
}

impl Halt {
    /// Creates a new Halt instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
        }
    }
}

impl Strategy for Halt {
    fn name(&self) -> &'static str {
        "Halt"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        //self.messages.clear();
        self.ids.iter().enumerate().for_each(|(_, id)| {
            action_wrapper.clear(*id);
            action_wrapper.push(*id, RawOrder::new(
                Command {
                    forward_velocity: 0.0,
                    left_velocity: 0.0,
                    angular_velocity: 0.0,
                    charge: false,
                    kick: None,
                    dribbler: 0.0,
                }
            ));
        });
        false
    }
}
