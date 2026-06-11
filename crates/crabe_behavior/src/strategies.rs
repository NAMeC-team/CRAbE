use crate::behaviors::Node;
use crate::behaviors::blackboard::RobotIntentWriters;
use crate::behaviors::blackboard::{State, StateSlots};
use crabe_framework::data::world::{AllyInfo, Robot, World};
use std::fmt::Debug;

pub mod defense;
mod hold_position;
mod utils;

pub trait Strategy: Send + Debug {
    fn cost(&self, robot: &Robot<AllyInfo>, world: &World) -> f64;
    fn init_state(&self) -> State {
        Default::default()
    }
    fn build_tree(&self, state: &StateSlots, intents: &RobotIntentWriters) -> Node;
}
