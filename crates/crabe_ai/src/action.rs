pub mod move_to;
pub mod sequencer;
pub mod state;

use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use state::State;
use std::collections::HashMap;

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
    pub fn push<T: Action>(&mut self, _action: T) {
        todo!()
    }

    pub fn set<T: Action>(&mut self, _action: T) {
        todo!()
    }

    pub fn compute(&mut self) -> CommandMap {
        todo!()
    }
}
