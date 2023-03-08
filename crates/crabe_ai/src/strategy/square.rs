use crate::action::move_to::MoveTo;
use crate::action::sequencer::Sequencer;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use nalgebra::Point2;
use std::f64::consts::PI;

#[derive(Default)]
pub struct Square {
    id: u16,
}

impl Square {
    fn new(id: u16) -> Self {
        Self { id }
    }
}

impl Strategy for Square {
    fn step(
        &mut self,
        data: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        let mut sequencer = Sequencer::default();
        sequencer.push(Box::new(MoveTo::new(Point2::new(-1.0, 1.0), PI / 4.0)));
        sequencer.push(Box::new(MoveTo::new(
            Point2::new(1.0, 1.0),
            -3.0 * PI / 4.0,
        )));
        sequencer.push(Box::new(MoveTo::new(
            Point2::new(1.0, -1.0),
            3.0 * PI / 4.0,
        )));
        sequencer.push(Box::new(MoveTo::new(Point2::new(-1.0, -1.0), PI / 4.0)));
        true
    }
}
