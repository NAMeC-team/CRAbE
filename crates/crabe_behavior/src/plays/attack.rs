use crate::plays::{Play, PlayStrategies};
use crate::strategies::Strategy;
use crate::strategies::defense::goal_keeper::GoalKeeper;
use crabe_framework::data::world::World;

#[derive(Debug)]
pub struct Attack;
impl Play for Attack {
    fn generate_strategies(&self, world: &World) -> PlayStrategies {
        PlayStrategies {
            keeper: Box::new(GoalKeeper),
            outfield: vec![],
        }
    }

    fn transition(&self, world: &World) -> Option<Box<dyn Play>> {
        None
    }
}
