use crate::behaviors::blackboard::{RobotIntentWriters, StateSlots};
use crate::managers::Manager;
use crate::strategies::Strategy;
use crate::strategies::defense::goal_keeper::GoalKeeper;
use crabe_framework::data::annotation::AnnotationStore;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::world::World;

#[derive(Debug)]
pub struct BehaviorEngine {
    manager: Box<dyn Manager>,
}

impl BehaviorEngine {
    pub fn new(manager: Box<dyn Manager>) -> Self {
        Self { manager }
    }
    pub fn decide(&mut self, world: &World, annotations: &mut AnnotationStore) -> CommandMap {
        self.manager.decide(world, annotations)
    }
}
