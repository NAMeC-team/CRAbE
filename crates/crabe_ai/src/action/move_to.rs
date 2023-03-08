use crate::action::state::State;
use crate::action::Action;
use crabe_framework::data::output::Command;
use nalgebra::Point2;

pub struct MoveTo {
    state: State,
    target: Point2<f64>,
    orientation: f64,
}

impl MoveTo {
    pub fn new(target: Point2<f64>, orientation: f64) -> Self {
        Self {
            state: State::Running,
            target,
            orientation,
        }
    }
}

impl Action for MoveTo {
    fn name(&self) -> String {
        String::from("MoveTo")
    }

    fn state(&mut self) -> State {
        self.state.clone()
    }

    fn compute_order(&mut self, id: u8) -> Command {
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
