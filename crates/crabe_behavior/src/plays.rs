pub mod attack;
pub mod prepare_kickoff;

use crate::strategies::Strategy;
use crabe_framework::data::world::World;
use std::fmt::Debug;

pub trait Play: Debug + Send {
    fn generate_strategies(&self, world: &World) -> PlayStrategies;
    fn transition(&self, world: &World) -> Option<Box<dyn Play>>;
}

#[derive(Debug)]
pub struct PlayStrategies {
    pub keeper: Box<dyn Strategy>,
    pub outfield: Vec<Box<dyn Strategy>>,
}
