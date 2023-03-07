use crate::action::{Action, State};

use crabe_framework::data::output::Command;

pub struct Sequencer {
    pub actions: Vec<Box<dyn Action>>,
}

impl Action for Sequencer {
    fn name(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.name())
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn state(&mut self) -> State {
        todo!()
    }

    fn compute_order(&mut self) -> Command {
        todo!()
    }

    fn cancel(&mut self) {
        todo!()
    }
}
