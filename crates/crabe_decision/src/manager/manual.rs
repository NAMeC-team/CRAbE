use nalgebra::{Point2, Translation2};
use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::testing::{Square, TestVisionMoveTo};
use crate::strategy::formations::MoveAwayFromBall;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::shape::{Circle, Line};
use crate::action::opt_pass::{OptimalPassMoveTo, SearchBounds};

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct Manual {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Manual {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            strategies: vec![Box::new(Square::new(0))],
        }
    }
}

impl Manager for Manual {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        self.strategies
            .retain_mut(|s| !s.step(world, tools_data, action_wrapper));

        // wip for testing
        if let Some(ally) = world.allies_bot.get(&0) {
            let from = ally.pose.position;
            let opmt = OptimalPassMoveTo::new(
                world,
                from,
                SearchBounds { min: 0., max: 3., step: 0.025 },
                SearchBounds { min: -2., max: 2., step: 0.025 },
            );
            let mut id = 0;

            let dummy_obj = Point2::new(4.5, 0.);
            opmt.graph.iter().for_each(|node| {
                let v = (dummy_obj - node.p).normalize() * 0.5;
                let t = Translation2::from(v);

                tools_data.annotations.add_line(
                    id.to_string(), Line { start: node.p, end: t * node.p }
                );
                id += 1;
            });
        }
    }
}
