use std::fmt::Debug;
use std::vec;
// use clap::builder::styling::Color;
// use crate::action::state::State::Running;
use crate::action::ActionWrapper;
use crate::exception::Exception;
use crate::exception::card_exception::CardException;
use crate::manager::Manager;
use crate::strategy::defensive::{DefenseWall, GoalKeeper};
use crate::strategy::offensive::Attacker;
// use crate::strategy::testing::Square;
use crate::strategy::Strategy;
use crate::strategy::rule_actions::{Halt,PrepareStart,PrepareKickOff,PreparePenalty, ExcecutePenalty};
use crate::strategy::formations::{MoveAwayFromBall, };
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::World;
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

    fn run(&mut self, available_bots_id: Vec<u8>) {
        let mut robots_left = available_bots_id.len();
        let mut available_bots_left = available_bots_id.clone();

        if robots_left > 0 {
            let chosen = available_bots_left.pop().expect("mismatch between left robots and left count");
            self.strategies.push(Box::new(Attacker::new(chosen)));
            robots_left -= 1;
        }

        if robots_left > 0 {
            let chosen = available_bots_left.pop().expect("mismatch between left robots and left count");
            self.strategies.push(Box::new(GoalKeeper::new(chosen)));
        }

        self.strategies.push(Box::new(DefenseWall::new(available_bots_left)));
    }

    fn manage_running(&mut self, state: RunningState, available_bots_id: Vec<u8>, world: &World) {
        match state {
            RunningState::KickOff(team_color) => {
                if team_color == world.team_color {
                    self.run(available_bots_id);
                } else {
                    if let Some(x) = world.get_goalkeeper(world.team_color) {
                        self.strategies.push(Box::new(GoalKeeper::new(x)));
                    };
                    let keeper_id = world.get_goalkeeper(world.team_color);
                    let keeper_id = world.get_goalkeeper(world.team_color);

                    let mut ids = vec![];
                    world.allies_bot.iter().for_each(|(id,_)| if *id != KEEPER_ID { ids.push(*id); });
                    self.strategies.push(Box::new(DefenseWall::new(ids)));
                }
            },
            RunningState::Penalty(team_color) => {
                if team_color == world.team_color {
                    self.strategies.push(Box::new(ExcecutePenalty::new(KEEPER_ID)));
                }else {
                    let mut ids = vec![];
                    world.allies_bot.iter().for_each(|(id,_)| if *id != KEEPER_ID { ids.push(*id); });

                    self.strategies.push(Box::new(MoveAwayFromBall::new(ids)));
                    self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
                }

            },
            RunningState::FreeKick(team_color) => {
                self.run(world, tools_data, action_wrapper);
            },
            RunningState::Run =>  { self.run(world, tools_data, action_wrapper); },
            RunningState::CornerKick(_) => {self.run(world, tools_data, action_wrapper); },
            RunningState::GoalKick(_) => { self.run(world, tools_data, action_wrapper); },
        }
    }

    fn manage_halted(&mut self, state: HaltedState, available_bots_id: Vec<u8>, world: &World) {
        match state {

            HaltedState::Halt => { self .strategies.push(Box::new(Halt::new(available_bots_id.clone()))) },
            HaltedState::Timeout(team_color) => { self.strategies.push(Box::new(Halt::new(available_bots_id))); },
        }
    }


    fn manage_stopped(&mut self, state: StoppedState, available_bots_id: &mut Vec<u8>, world: &World) {
        match state {

            StoppedState::PrepareKickoff(team_color) => { self.strategies.push(Box::new(PrepareKickOff::new(world.allies_bot.iter().map(|a| *a.0).collect(), team_color)));},
            StoppedState::PreparePenalty(team_color) => {
            if (team_color == world.team_color){
                self.strategies.push(Box::new(PreparePenalty::new(ATTACKER_ID)));
            }else {
                self.strategies.push(Box::new(MoveAwayFromBall::new(world.allies_bot.iter().map(|a| *a.0).collect())))
            }
            },
            StoppedState::BallPlacement(team_color) => { self .strategies.push(Box::new(Halt::new(world.allies_bot.iter().map(|a| *a.0).collect()))); },
            StoppedState::Stop => {self.run(world, tools_data, action_wrapper);},
            StoppedState::PrepareCornerKick(team_color) => if team_color == world.team_color {
                self.run(world, tools_data, action_wrapper);
            } else {
                let mut ids = vec![];
                world.allies_bot.iter().for_each(|(id,_)| if *id != KEEPER_ID { ids.push(*id); });
                self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
                self.strategies.push(Box::new(MoveAwayFromBall::new(ids)));
            }
            ,
            StoppedState::PrepareGoalKick(team_color) => self.run(world, tools_data, action_wrapper),
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
        action_wrapper.clear_all();
        self.strategies.clear();
        self.exceptions.iter_mut().for_each(|x| x.step(world, tools_data, action_wrapper, &mut self.benched));
        let mut available_bots_id: Vec<u8> = world.allies_bot.iter().map(|(id, _)| *id).collect();
        let mut to_bench = self.benched.len();
        if self.benched.len() >= available_bots_id.len() {
            info!("No bot available !!");
            return;
        }

        available_bots_id = available_bots_id[0..(available_bots_id.len() - to_bench)].to_vec();

        match world.data.ref_orders.state {
            GameState::Halted(state) => self.manage_halted(state, available_bots_id, world),
            GameState::Stopped(state) => self.manage_stopped(state, available_bots_id, world),
            GameState::Running(state) => self.manage_running(state, available_bots_id, world),
        }

        for s in &mut self.strategies {
          s.step(world, tools_data, action_wrapper);
        };
    }
}
