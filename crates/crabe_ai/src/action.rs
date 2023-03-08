pub mod move_to;
pub mod sequencer;

use crabe_framework::data::output::{Command, CommandMap};

#[derive(Clone)]
pub enum State {
    Running,
    Failed,
    Done,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Running, Self::Running)
                | (Self::Failed, Self::Failed)
                | (Self::Done, Self::Done)
        )
    }
}

pub trait Action {
    fn name(&self) -> String;
    fn state(&mut self) -> State;

    fn compute_order(&mut self) -> Command;
    fn cancel(&mut self);
}

#[derive(Default)]
pub struct ActionWrapper {
    actions: Vec<Box<dyn Action>>,
}

impl ActionWrapper {
    pub fn push(&mut self, _action: Box<dyn Action>) {
        todo!()
    }

    pub fn set(&mut self, _action: Box<dyn Action>) {
        todo!()
    }

    pub fn compute(&mut self) -> CommandMap {
        todo!()
    }
}
