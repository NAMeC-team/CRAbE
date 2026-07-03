use crate::behaviors::actions::move_to::MoveTo;
use crate::behaviors::actions::write_const::WriteConst;
use crate::behaviors::blackboard::{Blackboard, RobotIntentWriters, State, StateSlots};
use crate::behaviors::{Node, action, seq};
use crate::strategies::Strategy;
use crabe_framework::data::world::{AllyInfo, Pose, Robot, World};
use nalgebra::distance;

#[derive(Debug)]
pub struct HoldPosition {
    target: Pose,
}

impl Strategy for HoldPosition {
    fn cost(&self, robot: &Robot<AllyInfo>, _world: &World) -> f64 {
        distance(&robot.position(), &self.target.position)
    }

    fn init_state(&self) -> State {
        State {
            target: self.target.clone(),
            ..Default::default()
        }
    }

    fn build_tree(&self, state: &StateSlots, intents: &RobotIntentWriters) -> Node {
        seq(vec![action(MoveTo::new(
            state.target.input(),
            intents.movement,
        ))])
    }
}
