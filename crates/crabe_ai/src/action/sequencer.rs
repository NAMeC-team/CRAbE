use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;

pub struct Sequencer {
    state: State,
    pub actions: Vec<Box<dyn Action>>,
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
    pub fn push(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
        self.state = match self.state {
            State::Running | State::Done => State::Running,
            State::Failed => State::Failed,
        };
    }
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
        self.state.clone()
    }

    fn compute_order(&mut self) -> Command {
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
            action.compute_order()
        } else {
            self.state = State::Done;
            Command::default()
        }
    }

    fn cancel(&mut self) {
        self.actions.iter_mut().for_each(|action| action.cancel());
    }
}
