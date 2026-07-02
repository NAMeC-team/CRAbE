use crate::action::state::State::Running;
use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::testing::Square;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{self, World};

/// Manager handling game states, unnamed atm
pub struct StateBasedManager {
    strategies: HashMap<Gam, Vec<Box<dyn Strategy>>>,
}

impl StateBasedManager {
    pub fn new() -> Self {
        Self {
            strategies: vec![],
        }
    }

    fn manage_running(&mut self, state: RunningState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            RunningState::KickOff(team_color) => todo!(),
            RunningState::Penalty(team_color) => todo!(),
            RunningState::FreeKick(team_color) => todo!(),
            RunningState::Run => todo!(),
            RunningState::CornerKick(team_color) => todo!(),
            RunningState::GoalKick(team_color) => todo!(),
        }
    }

    fn manage_halted(&mut self, state: HaltedState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            HaltedState::Halt => todo!(),
            HaltedState::Timeout(team_color) => todo!(),
        }
    }


    fn manage_stopped(&mut self, state: StoppedState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            StoppedState::PrepareKickoff(team_color) => todo!(),
            StoppedState::PreparePenalty(team_color) => todo!(),
            StoppedState::BallPlacement(team_color) => todo!(),
            StoppedState::Stop => todo!(),
            StoppedState::PrepareCornerKick(team_color) => todo!(),
            StoppedState::PrepareGoalKick(team_color) => todo!(),
        }
    }
}

impl Manager for StateBasedManager {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        match world.data.ref_orders.state {
            GameState::Halted(state) => self.manage_halted(state, world, tools_data, action_wrapper),
            GameState::Stopped(state) => self.manage_stopped(state, world, tools_data, action_wrapper),
            GameState::Running(state) => self.manage_running(state, world, tools_data, action_wrapper),
        }
    }
}
