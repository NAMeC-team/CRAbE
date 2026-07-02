use std::collections::HashMap;

use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::offensive::Attacker;
use crate::strategy::testing::Square;
use crate::strategy::Strategy;
use crabe_framework::data::{tool::ToolData, world::RobotMap};
use crabe_framework::data::world::{AllyInfo, World};
use crate::exception::{Exception, card_exception::CardException};

/// The `Manual` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `Manual` struct.
#[derive(Default)]
pub struct Manual {
    strategies: Vec<Box<dyn Strategy>>,
    exceptions: Vec<Box<dyn Exception>>,
    benched: Vec<u8>,
}

impl Manual {
    /// Creates a new `Manual` instance with the desired strategies to test.
    pub fn new() -> Self {
        let mut strategies: Vec<Box<dyn Strategy>> = vec![];
        for i in 0..6 {
            let strategy: Box<dyn Strategy> = Box::new(Attacker::new(i));
            strategies.push(strategy);
        }

        Self {
            strategies: strategies,
            exceptions: vec![Box::new(CardException::new())],
            benched: vec![],
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
        self.exceptions.iter_mut().for_each(|x| x.step(world, tools_data, action_wrapper, &mut self.benched));
        self.strategies
            .retain_mut(
                |s|
                if s.robots().iter().all(|r| !self.benched.contains(r))  {
                    !s.step(world, tools_data, action_wrapper)
                } else {
                    true
                }

        );
    }
}
