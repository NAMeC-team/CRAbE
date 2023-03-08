pub mod move_to;
pub mod sequencer;
pub mod state;
use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use enum_dispatch::enum_dispatch;
use state::State;
use std::collections::HashMap;
use std::ops::DerefMut;

#[enum_dispatch(Actions)]
pub trait Action {
    fn name(&self) -> String;
    fn state(&mut self) -> State;

    fn compute_order(&mut self, id: u8) -> Command;
    fn cancel(&mut self);
}

#[enum_dispatch]
pub enum Actions {
    MoveTo(MoveTo),
    Sequencer(Sequencer),
}

#[derive(Default)]
pub struct ActionWrapper {
    pub actions: HashMap<u8, Actions>,
}

impl ActionWrapper {
    pub fn push<T: Action>(&mut self, id: u8, _action: T) {
        todo!()
    }

    pub fn set<T: Action>(&mut self, _action: T) {
        todo!()
    }

    pub fn compute(&mut self) -> CommandMap {
        let mut command_map = CommandMap::default();
        self.actions.iter_mut().for_each(|(id, action)| {
            command_map.insert(*id, action.compute_order(*id));
        });
        command_map
    }
}
