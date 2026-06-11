use crate::behaviors::blackboard::{Blackboard, IntentWriter, RobotIntentWriters, StateSlots};
use crate::behaviors::{Behavior, BehaviorFrame, Context, Node, Status};
use crate::strategies::Strategy;
use crabe_framework::data::annotation::AnnotationStore;
use crabe_framework::data::output::{Command, CommandPatch};
use crabe_framework::data::world::{Pose, World};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Executor {
    blackboard: Blackboard,
    strategy: Box<dyn Strategy>,
    cached_root: Node,
}

pub struct Environment<'a> {
    pub world: &'a World,
    pub annotations: &'a mut AnnotationStore,
}

impl Executor {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        let state_slots = StateSlots::new();
        let intent_writers = RobotIntentWriters::new();
        Self {
            blackboard: Default::default(),
            cached_root: strategy.build_tree(&state_slots, &intent_writers),
            strategy,
        }
    }

    pub fn strategy(&self) -> &Box<dyn Strategy> {
        &self.strategy
    }

    pub fn tick(&mut self, environment: &mut Environment, robot_id: u8) -> BehaviorFrame {
        let mut context = Context {
            robot_id: &robot_id,
            world: environment.world,
            annotations: environment.annotations,
            blackboard: &mut self.blackboard,
        };

        let state = self.cached_root.tick(&mut context);
        let command = (&self.blackboard.intents).into();
        self.blackboard.intents.reset();
        BehaviorFrame { command, state }
    }
}
