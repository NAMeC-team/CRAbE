use std::vec;

use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::message::Message;
use crate::message::MessageData;
use crate::strategy;
use crate::strategy::testing::{Aligned, GoLeft, GoRight};
use crate::strategy::formations::Stop;
use crate::strategy::defensive::{GoalKeeper, BotMarking, BotContesting};
use crate::strategy::Strategy;
use crabe_framework::data::geometry::Goal;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::*;
use crabe_framework::data::world::World;

/// The `BigBro` struct represents a decision manager that executes strategies BigBroly
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `BigBro` struct.
#[derive(Default)]
pub struct BigBro {
    pub strategies: Vec<Box<dyn Strategy>>,
}

impl BigBro {
    /// Creates a new `BigBro` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            strategies: vec![
                Box::new(Stop::new(vec![2, 3, 4])),
                Box::new(GoalKeeper::new(0)),
                Box::new(GoLeft::new(1)),
            ],
        }
    }

    /// Moves a bot from its current strategy to an existing strategy.
    ///
    /// # Arguments
    /// - `bot_id`: The id of the bot to move.
    /// - `strategy_index`: The index of the strategy (in the strategies list) to move the bot to.
    pub fn move_bot_to_existing_strategy(&mut self, bot_id: u8, strategy_index: usize) {
        self.move_bots_to_existing_strategy(vec![bot_id], strategy_index);
    }

    /// Moves a list of bots from their current strategy to an existing strategy.
    /// 
    /// # Arguments
    /// - `bot_ids`: The list of bot ids to move.
    /// - `strategy_index`: The index of the strategy (in the strategies list) to move the bots to.
    /// 
    /// # Example
    /// ```
    /// use crabe_decision::manager::bigbro::BigBro;
    /// let mut bigbro = BigBro::new();
    /// bigbro.strategies.clear();
    /// bigbro.move_bots_to_existing_strategy(vec![1, 2, 3], 0);
    /// assert_eq!(bigbro.strategies.len(), 0);
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::formations::Stop::new(vec![])));
    /// bigbro.move_bots_to_existing_strategy(vec![1, 2, 3], 0);
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1, 2, 3]);
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::Aligned::new(vec![4])));
    /// bigbro.move_bots_to_existing_strategy(vec![1, 2, 3], 1);
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![4, 1, 2, 3]);
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::GoLeft::new(5)));
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::GoRight::new(0)));
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::formations::Stop::new(vec![2])));
    /// assert_eq!(bigbro.strategies.len(), 4);
    /// bigbro.move_bots_to_existing_strategy(vec![5, 0], 3);
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![2, 5, 0]);
    /// ```
    pub fn move_bots_to_existing_strategy(&mut self, bot_ids: Vec<u8>, strategy_index: usize) {
        if strategy_index >= self.strategies.len(){
            return;
        }
        let mut new_strategy_index = strategy_index;
        for bot_id in bot_ids {
            let mut new_strategy_ids = self.strategies[new_strategy_index].as_ref().get_ids();
            if new_strategy_ids.contains(&bot_id) { // if already in the strategy, don't do anything
                continue;
            };
            // if the bot is already in a strategy, remove it from there
            if let Some(bot_current_strategy_index) = self
                .strategies
                .iter()
                .position(|s| s.get_ids().contains(&bot_id)){
                    let mut current_strategy_ids = self.strategies[bot_current_strategy_index]
                        .as_ref()
                        .get_ids();
                    if current_strategy_ids.len() == 1 {
                        self.strategies.remove(bot_current_strategy_index);
                        if new_strategy_index > bot_current_strategy_index {
                            new_strategy_index = new_strategy_index - 1;
                        }
                    } else {
                        current_strategy_ids.retain(|&id| id != bot_id);
                        self.strategies[bot_current_strategy_index].put_ids(current_strategy_ids);
                    }
            }
            new_strategy_ids.push(bot_id);
            self.strategies[new_strategy_index].put_ids(new_strategy_ids);
        }
    }

    /// Moves a bot from its current strategy to a new strategy.
    /// If the bot is the only one in its current strategy, the strategy is replaced with the new one.
    /// Otherwise, the bot is removed from the current strategy and added to the new one.
    /// 
    /// # Arguments
    /// - `bot_id`: The id of the bot to move.
    /// - `strategy`: The new strategy to move the bot to.
    pub fn move_bot_to_new_strategy(&mut self, bot_id: u8, strategy: Box<dyn Strategy>) {
        if let Some(current_strategy_index) = self
            .strategies
            .iter()
            .position(|s| s.get_ids().contains(&bot_id))
        {
            let mut ids = self.strategies[current_strategy_index].as_ref().get_ids();

            // we should always branch in this if
            // there was an `unwrap()` before and it was safe,
            // but it was transformed into a safer syntax just in case
            if let Some(index_of_bot_in_slot_ids) = ids.iter().position(|x| x == &bot_id) {
                ids.remove(index_of_bot_in_slot_ids);
                if ids.len() == 0 {
                    //if the bot was the alone in this strategy, we can replace it
                    self.strategies[current_strategy_index] = strategy;
                } else {
                    self.strategies[current_strategy_index].put_ids(ids);
                    self.strategies.push(strategy);
                }
            }

        } else {
            self.strategies.push(strategy);
        }
    }


    
    /// Processes the messages received from the strategies and updates the strategies accordingly.
    ///
    /// # Arguments
    /// - `messages`: A list of `MessageData` instances containing the messages received from the strategies.
    pub fn process_messages(&mut self, messages: Vec<MessageData>) {
        messages.iter().for_each(|m| {
            match m.message {
                Message::WantToGoRight => {
                    let strategy = Box::new(GoRight::new(m.id));
                    self.move_bot_to_new_strategy(m.id, strategy);
                }
                Message::WantToGoLeft => {
                    let strategy = Box::new(GoLeft::new(m.id));
                    self.move_bot_to_new_strategy(m.id, strategy);
                }
                Message::WantToBeAligned => {
                    //find strategy index with name "Aligned"
                    if let Some(strategy_index) = self.get_index_strategy_with_name("Aligned") {
                        self.move_bot_to_existing_strategy(m.id, strategy_index);
                    } else {
                        let strategy = Box::new(Aligned::new(vec![m.id]));
                        self.move_bot_to_new_strategy(m.id, strategy);
                    }
                }
                _ => {}
            }
        });
    }

    /// Get the index of a strategy with a given name.
    /// 
    /// # Arguments
    /// - `name`: The name of the strategy.
    /// 
    /// # Returns
    /// The index of the strategy in the strategies list.
    pub fn get_index_strategy_with_name(&self, name: &str) -> Option<usize> {
        self.strategies.iter().position(|s| s.name() == name)
    }

    /// Get the robot current strategy.
    /// 
    /// # Arguments
    /// - `bot_id`: The id of the robot.
    ///     
    /// # Returns
    /// The strategy of the robot.
    pub fn get_bot_current_strategy(&self, bot_id: u8) -> Option<&Box<dyn Strategy>> {
        if let Some(strategy) = self.strategies.iter().find(|s| s.get_ids().contains(&bot_id)){
            return Some(strategy);
        }
        None
    }
    
    /// Put all bots to the Stop strategy.
    pub fn everyone_stop(&mut self) {
        // Ensure there is a stop strategy available, either existing or new.
        let stop_strategy_index = match self.get_index_strategy_with_name("Stop") {
            Some(index) => index,
            None => {
                let stop_strategy = Box::new(Stop::new(vec![]));
                self.strategies.push(stop_strategy);
                self.get_index_strategy_with_name("Stop").expect("Stop strategy should exist after being added")
            }
        };
        // Move all bots to the stop strategy.
        for robot_id in 0..6 {
            if let Some(updated_stop_strategy_index) = self.get_index_strategy_with_name("Stop") {
                self.move_bot_to_existing_strategy(robot_id, updated_stop_strategy_index);
            }
        }
    }
}

impl Manager for BigBro {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        match world.data.ref_orders.state {
            GameState::Halted(halted_state) => match halted_state {
                HaltedState::GameNotStarted => println!("game not started"),
                HaltedState::Halt => self.everyone_stop(),
                HaltedState::Timeout(team) => println!("timeout by {:?}", team),
            }
            GameState::Stopped(stopped_state) => match stopped_state {
                StoppedState::Stop => println!("stop"),
                StoppedState::PrepareKickoff(team) => println!("prepare kick off {:?}",team),
                StoppedState::PreparePenalty(team) => println!("prepare penalty {:?}",team),
                StoppedState::BallPlacement(team) => println!("ball placement {:?}",team),
                StoppedState::PrepareForGameStart => println!("prepare for game start"),
                StoppedState::BallLeftFieldTouchLine(_) => println!("ball left field touch line"),
                StoppedState::CornerKick(_) => println!("corner kick"),
                StoppedState::GoalKick(_) => println!("goal kick"),
                StoppedState::AimlessKick(_) => println!("aimless kick"),
                StoppedState::NoProgressInGame => println!("no progress in game"),
                StoppedState::PrepareFreekick(_) => println!("prepare freekick"),
                StoppedState::FoulStop => println!("foul stop"),
            },
            GameState::Running(running_state) => match running_state {
                RunningState::KickOff(team) => println!("kickoff for {:#?}", team),
                RunningState::Penalty(team) => println!("penalty for {:#?}", team),
                RunningState::FreeKick(team) => println!("free kick for {:#?}", team),
                RunningState::Run => println!("run"),
            }
        }
        
        // mailbox to grab the messages
        // (we can't iter the strategies and modify them at the same time so we need to collect the messages first and then process them)
        let mut messages: Vec<MessageData> = vec![];

        // grab all the messages from the strategies
        self.strategies.iter().for_each(|s| {
            messages.extend(s.get_messages().clone());
        });

        // process the messages
        self.process_messages(messages);

        // execute the strategies
        self.strategies.iter_mut().for_each(|s| {
            s.step(world, tools_data, action_wrapper);
        });
    }
}
