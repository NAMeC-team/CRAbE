pub mod move_to;
pub mod sequencer;
pub mod state;
use enum_dispatch::enum_dispatch;
use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use state::State;
use std::collections::HashMap;
use std::ops::DerefMut;

#[enum_dispatch(Actions)]
pub trait Action {
    fn name(&self) -> String;
    fn state(&mut self) -> State;

    fn compute_order(&mut self) -> Command;
    fn cancel(&mut self);
}

#[enum_dispatch]
pub enum Actions {
    Sequencer(Sequencer),
}

#[derive(Default)]
pub struct ActionWrapper {
    pub actions: HashMap<u16, Actions>,
}

impl ActionWrapper {
    pub fn push<T: Action>(&mut self, _action: T) {
        let sequencer: Sequencer = Default::default();
        let mut actions: Actions= sequencer.into();
        actions.cancel();

        let actions2 = &mut actions;
        actions2.cancel();
        todo!()
    }

    pub fn set<T: Action>(&mut self, _action: T) {
        todo!()
    }

    pub fn compute(&mut self) -> CommandMap {
        let mut command_map = CommandMap::default();
        self.actions.values_mut().for_each(|action| {command_map.insert(0, action.compute_order());});
        command_map
    }
}
