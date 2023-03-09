pub mod move_to;
pub mod sequencer;
pub mod state;
use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crabe_framework::data::output::{Command, CommandMap};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use enum_dispatch::enum_dispatch;
use state::State;
use std::collections::HashMap;

#[enum_dispatch(Actions)]
pub trait Action {
    fn name(&self) -> String;
    fn state(&mut self) -> State;

    fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command;
    fn cancel(&mut self);
}

#[enum_dispatch]
pub enum Actions {
    MoveTo(MoveTo),
}

#[derive(Default)]
pub struct ActionWrapper {
    pub actions: HashMap<u8, Sequencer>,
}

impl ActionWrapper {
    pub fn push<T: Action + Into<Actions>>(&mut self, id: u8, mut action: T) {
        if let Some(sequencer) = self.actions.get_mut(&id) {
            sequencer.push(action.into());
        } else {
            self.actions
                .insert(id, Sequencer::new(action.into()));
        }
    }

    pub fn clean(&mut self, id: u8) {
        if let Some(sequencer) = self.actions.get_mut(&id) {
            sequencer.clear();
        }
    }

    pub fn compute(&mut self, world: &World, tools: &mut ToolData) -> CommandMap {
        let mut command_map = CommandMap::default();
        self.actions.iter_mut().for_each(|(id, action)| {
            command_map.insert(*id, action.compute_order(*id, world, tools));
        });
        command_map
    }
}
