use std::fmt::Debug;
use std::vec;
use clap::builder::styling::Color;
use crabe_protocol::protobuf::game_controller_packet::game_event::Type::Goal;
use crate::action::state::State::Running;
use crate::action::{self, ActionWrapper};
use crate::exception::Exception;
use crate::exception::card_exception::CardException;
use crate::manager::Manager;
use crate::strategy::defensive::{DefenseWall, GoalKeeper};
use crate::strategy::offensive::Attacker;
// use crate::strategy::testing::Square;
use crate::strategy::Strategy;
use crate::utils::{ATTACKER_ID, KEEPER_ID};
use crate::strategy::rule_actions::{Halt,PrepareStart,PrepareKickOff,PreparePenalty, ExcecutePenalty};
use crate::strategy::formations::{MoveAwayFromBall, };
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

    fn run(&mut self, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        let mut robots_left = world.allies_bot.iter().count() - self.benched.len();
        let mut chosen_bots = vec![];

        if !self.benched.contains(&ATTACKER_ID) && robots_left > 0 {
            self.strategies.push(Box::new(Attacker::new(ATTACKER_ID)));
            robots_left -= 1;
            chosen_bots.push(ATTACKER_ID);
        }

        if !self.benched.contains(&KEEPER_ID) && robots_left > 0 {
            self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
            robots_left -= 1;
            chosen_bots.push(KEEPER_ID);
        }

        if (robots_left > 0) {
            let mut wall_ids: Vec<u8> = vec![];
            let mut id: u8 = 1;
            while robots_left > 0 {
                //id not in chosen bots or benched
                if !(self.benched.contains(&id) || chosen_bots.contains(&id)){
                    robots_left -= 1;
                    wall_ids.push(id);
                }

                if id > 6 {
                    info!("Maximum id achieved in running state");
                    break;
                }
                id += 1;
            }

            self.strategies.push(Box::new(DefenseWall::new(wall_ids)));
        }
    }

    fn manage_running(&mut self, state: RunningState, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {
            RunningState::KickOff(team_color) => {
                if team_color == world.team_color {
                    self.run(world, tools_data, action_wrapper);
                } else {
                    let mut ids = vec![];
                    world.allies_bot.iter().for_each(|(id,_)| if *id != KEEPER_ID { ids.push(*id); });
                    self.strategies.push(Box::new(DefenseWall::new(ids)));
                    self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
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
            RunningState::FreeKick(team_color) =>
            if team_color == world.team_color {
                self.run(world, tools_data, action_wrapper);
            } else {
                let mut ids = vec![];
                world.allies_bot.iter().for_each(|(id,_)| if *id != KEEPER_ID { ids.push(*id); });
                self.strategies.push(Box::new(GoalKeeper::new(KEEPER_ID)));
                // self.strategies.push(Box::new(DefenseWall::new(ids)));
            },
            RunningState::Run =>  { self.run(world, tools_data, action_wrapper); },
            RunningState::CornerKick(_) => {self.run(world, tools_data, action_wrapper); },
            RunningState::GoalKick(_) => { self.run(world, tools_data, action_wrapper); },
        }
    }

    fn manage_halted(&mut self, state: HaltedState, world: &World, tool_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
        match state {

            HaltedState::Halt => { self .strategies.push(Box::new(Halt::new(world.allies_bot.iter().map(|a| *a.0).collect())));},
            HaltedState::Timeout(team_color) => { self.strategies.push(Box::new(Halt::new(world.allies_bot.iter().map(|a| *a.0).collect()))); },
        }
    }


    fn manage_stopped(&mut self, state: StoppedState, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) {
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
            StoppedState::Stop => {self.strategies.push(Box::new(MoveAwayFromBall::new(world.allies_bot.iter().map(|a| *a.0).collect())));},
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
