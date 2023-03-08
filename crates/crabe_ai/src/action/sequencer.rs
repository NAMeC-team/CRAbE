use crate::action::state::State;
use crate::action::{Action, Actions};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

pub struct Sequencer {
    state: State,
    pub actions: Vec<Actions>,
}

impl Default for Sequencer {
    fn default() -> Self {
        Self {
            state: State::Running,
            actions: vec![],
        }
    }
}

impl Sequencer {
    pub fn new(action: Actions) -> Self {
        Self {
            state: State::Running,
            actions: vec![action],
        }
    }

    pub fn push(&mut self, action: Actions) {
        self.actions.push(action);
        self.state = match self.state {
            State::Running | State::Done => State::Running,
            State::Failed => State::Failed,
        };
    }

    pub fn name(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.name())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn compute_order(&mut self, id: u8, world: &World, tools: &mut ToolData) -> Command {
        if self.state == State::Failed || self.actions.is_empty() {
            return Command::default();
        }

        let mut iter = self.actions.iter_mut();

        if let Some(action) = iter.next() {
            match action.state() {
                State::Failed => {
                    self.state = State::Failed;
                    return Command::default();
                }
                State::Done => {
                    self.actions.remove(0);
                }
                _ => {}
            }
        }

        if let Some(action) = self.actions.iter_mut().next() {
            action.compute_order(id, world, tools)
        } else {
            self.state = State::Done;
            Command::default()
        }
    }

    pub fn cancel(&mut self) {
        self.actions.iter_mut().for_each(|action| action.cancel());
    }
}
