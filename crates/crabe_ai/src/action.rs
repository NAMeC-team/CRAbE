pub mod move_to;

use crabe_framework::data::output::Command;

#[derive(Clone)]
pub enum State {
    Running,
    Failed,
    Done,
}

pub trait Action {
    fn name() -> &'static str;
    fn state(&mut self) -> State;

    fn compute_order(&mut self) -> Command;
    fn cancel(&mut self);
}
