use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;

pub struct MoveTo {
    state: State,
}

impl Action for MoveTo {
    fn name(&self) -> String {
        String::from("MoveTo")
    }

    fn state(&mut self) -> State {
        self.state.clone()
    }

    fn compute_order(&mut self) -> Command {
        self.state = State::Done;
        Command {
            forward_velocity: 1.0,
            left_velocity: 0.0,
            angular_velocity: 0.0,
            charge: false,
            kick: None,
            dribbler: 0.0,
        }
    }

    fn cancel(&mut self) {}
}
