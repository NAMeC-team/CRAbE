use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

#[derive(Clone)]
pub struct RawOrder {
    state: State,
    command: Command,
}

impl From<&mut RawOrder> for RawOrder {
    fn from(other: &mut RawOrder) -> RawOrder {
        RawOrder {
            state: other.state,
            command: other.command,
        }
    }
}

impl RawOrder {
    pub fn new(command: Command) -> Self {
        Self {
            state: State::Running,
            command,
        }
    }
}

impl Action for RawOrder {
    fn name(&self) -> String {
        String::from("RawOrder")
    }

    fn state(&mut self) -> State {
        self.state
    }

    fn compute_order(&mut self, _id: u8, _world: &World, _tools: &mut ToolData) -> Command {
        self.state = State::Done;
        self.command
    }

    fn cancel(&mut self) {}
}
