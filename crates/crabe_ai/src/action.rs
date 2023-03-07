pub mod move_to;
pub mod sequencer;

use crabe_framework::data::output::Command;

#[derive(Clone)]
pub enum State {
    Running,
    Failed,
    Done,
}

pub trait Action {
    fn name(&self) -> String;
    fn state(&mut self) -> State;

    fn compute_order(&mut self) -> Command;
    fn cancel(&mut self);
}
