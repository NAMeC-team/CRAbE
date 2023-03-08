pub mod move_to;
pub mod sequencer;

use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use std::collections::HashMap;

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

pub enum Actions {
    MoveTo(MoveTo),
    Sequencer(Sequencer),
}

#[derive(Default)]
pub struct ActionWrapper {
    pub actions: HashMap<u16, Actions>,
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
