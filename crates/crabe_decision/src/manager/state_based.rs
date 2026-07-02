use std::vec;

use crate::action::state::State::Running;
use crate::action::ActionWrapper;
use crate::exception::Exception;
use crate::exception::card_exception::CardException;
use crate::manager::Manager;
use crate::strategy::defensive::{DefenseWall, GoalKeeper};
use crate::strategy::offensive::Attacker;
use crate::strategy::testing::Square;
use crate::strategy::Strategy;
use crate::utils::{ATTACKER_ID, KEEPER_ID};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{self, World};
use log::info;

/// Manager handling game states, unnamed atm
pub struct StateBasedManager {
    strategies: Vec<Box<dyn Strategy>>,
    exceptions: Vec<Box<dyn Exception>>,
    benched: Vec<u8>,
}

impl StateBasedManager {
    pub fn new() -> Self {
        Self {
            strategies: vec![],
            exceptions: vec![Box::new(CardException::new())],
            benched: vec![],
        }
    }

    fn manage_running(&mut self, state: RunningState, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            RunningState::KickOff(team_color) => info!("Kickoff todo"),
            RunningState::Penalty(team_color) => info!("Penalty todo"),
            RunningState::FreeKick(team_color) => info!("FreeKick todo"),
            RunningState::Run =>  {
                let mut robots_left = 6 - self.benched.len();

                if !self.benched.contains(&KEEPER_ID) {
                    self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
                    robots_left -= 1;
                }

                if !self.benched.contains(&ATTACKER_ID) {
                    self.strategies.push(Box::new(Attacker::new(ATTACKER_ID)));
                    robots_left -= 1;
                }

                if (robots_left > 0) {
                    let mut wall_ids: Vec<u8> = vec![];
                    let mut id: u8 = 1;
                    while robots_left > 0 {
                        if !self.benched.contains(&id) {
                            robots_left -= 1;
                            wall_ids.push(id);
                        }

                        id += 1;
                    }

                    self.strategies.push(Box::new(DefenseWall::new(wall_ids)));
                }

            },
            RunningState::CornerKick(team_color) => info!("CornerKick todo"),
            RunningState::GoalKick(team_color) => info!("GoalKick todo"),
        }
    }

    fn manage_halted(&mut self, state: HaltedState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            HaltedState::Halt => info!("Halt todo"),
            HaltedState::Timeout(team_color) => info!("Timeout todo"),
        }
    }


    fn manage_stopped(&mut self, state: StoppedState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            StoppedState::PrepareKickoff(team_color) => info!("PrepareKickoff todo"),
            StoppedState::PreparePenalty(team_color) => info!("PreparePenalty todo"),
            StoppedState::BallPlacement(team_color) => info!("BallPlacement todo"),
            StoppedState::Stop => info!("Stop todo"),
            StoppedState::PrepareCornerKick(team_color) => info!("PrepareCornerKick todo"),
            StoppedState::PrepareGoalKick(team_color) => info!("PrepareGoalKick todo"),
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
        self.strategies.clear();
        self.exceptions.iter_mut().for_each(|x| x.step(world, tools_data, action_wrapper, &mut self.benched));
        match world.data.ref_orders.state {
            GameState::Halted(state) => self.manage_halted(state, world, tools_data, action_wrapper),
            GameState::Stopped(state) => self.manage_stopped(state, world, tools_data, action_wrapper),
            GameState::Running(state) => self.manage_running(state, world, tools_data, action_wrapper),
        }

        for s in &mut self.strategies {
            s.step(world, tools_data, action_wrapper);
        };
    }
}
