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
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{self, World};

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
            RunningState::KickOff(team_color) => todo!(),
            RunningState::Penalty(team_color) => todo!(),
            RunningState::FreeKick(team_color) => todo!(),
            RunningState::Run =>  {
                self.exceptions.iter_mut().for_each(|x| x.step(world, tools_data, action_wrapper, &mut self.benched));
                let mut strategies: Vec<Box<dyn Strategy>> = vec![];

                let robots_left = 6 - self.benched.len();

                if robots_left > 0 {
                    strategies.push(Box::new(GoalKeeper::new(0)));
                }

                if robots_left > 1 {
                    strategies.push(Box::new(Attacker::new(1)));
                }

                if robots_left > 2 {
                    let mut wall_ids: Vec<u8> = vec![];
                    for id in 2..(robots_left + 1) {
                        wall_ids.push(id as u8);
                    }
                    strategies.push(Box::new(DefenseWall::new(wall_ids)));
                }

                for free_robot in 0..(7 - self.benched.len()) {
                    let s = &mut self.strategies[free_robot];
                    if s.robots().iter().all(|r| !self.benched.contains(r))  {
                        s.step(world, tools_data, action_wrapper);
                    }
                };
            },
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
