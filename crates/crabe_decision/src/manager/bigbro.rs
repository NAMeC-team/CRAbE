use std::vec;
use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::message::AttackerMessage;
use crate::message::Message;
use crate::message::MessageData;
use crate::strategy::offensive::Attacker;
use crate::strategy::offensive::Receiver;
use crate::strategy::testing::{Aligned, GoLeft, GoRight};
use crate::strategy::Strategy;
use crate::utils::everyone_halt;
use crate::utils::everyone_stop;
use crate::utils::everyone_stop_except_keeper;
use crate::utils::prepare_kick_off;
use crate::utils::prepare_start;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::game_state::*;
use crabe_framework::data::world::World;
use crate::utils::bigbro_decisions::run_state;

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
            strategies: vec![],
        }
    }

    /// Removes a bot from all strategies.
    ///     
    /// # Arguments
    /// - `bot_id`: The id of the bot to remove.
    /// 
    /// # Example
    /// ```
    /// use crabe_decision::manager::bigbro::BigBro;
    /// let mut bigbro = BigBro::new();
    /// bigbro.strategies.clear();
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::formations::Stop::new(vec![1, 2, 3])));
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::Aligned::new(vec![4])));
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::GoLeft::new(5)));
    /// bigbro.strategies.push(Box::new(crabe_decision::strategy::testing::GoRight::new(0)));
    /// assert_eq!(bigbro.strategies.len(), 4);
    /// bigbro.remove_bot_from_strategies(0);
    /// assert_eq!(bigbro.strategies.len(), 3);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1, 2, 3]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![4]);
    /// assert_eq!(bigbro.strategies[2].as_ref().get_ids(), vec![5]);
    /// bigbro.remove_bot_from_strategies(5);
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1, 2, 3]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![4]);
    /// bigbro.remove_bot_from_strategies(2);
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1, 3]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![4]);
    /// ```
    pub fn remove_bot_from_strategies(&mut self, bot_id: u8) {
        for strategy in self.strategies.iter_mut() {
            let mut ids = strategy.get_ids();
            ids.retain(|&id| id != bot_id);
            strategy.put_ids(ids);
        }
        self.strategies.retain(|s| {
            let ids = s.get_ids();
            !(ids.len() == 1 && ids[0] == bot_id) && !ids.is_empty()
        });
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
    /// 
    /// # Example
    /// ```
    /// use crabe_decision::manager::bigbro::BigBro;
    /// let mut bigbro = BigBro::new();
    /// bigbro.strategies.clear();
    /// bigbro.move_bot_to_new_strategy(1, Box::new(crabe_decision::strategy::formations::Stop::new(vec![])));
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1]);
    /// bigbro.move_bot_to_new_strategy(2, Box::new(crabe_decision::strategy::testing::Aligned::new(vec![2])));
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![2]);
    /// bigbro.move_bot_to_new_strategy(1, Box::new(crabe_decision::strategy::testing::GoLeft::new(1)));
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![2]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![1]);
    /// ```
    pub fn move_bot_to_new_strategy(&mut self, bot_id: u8, strategy: Box<dyn Strategy>) {
        self.move_bots_to_new_strategy(vec![bot_id], strategy);
    }

    /// Moves a list of bots from their current strategy to a new strategy.
    /// 
    /// # Arguments
    /// - `bot_ids`: The list of bot ids to move.
    /// - `strategy`: The new strategy to move the bots to.
    /// 
    /// # Example
    /// ```
    /// use crabe_decision::manager::bigbro::BigBro;
    /// let mut bigbro = BigBro::new();
    /// bigbro.strategies.clear();
    /// bigbro.move_bots_to_new_strategy(vec![1, 2, 3], Box::new(crabe_decision::strategy::formations::Stop::new(vec![])));
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![1, 2, 3]);
    /// bigbro.move_bots_to_new_strategy(vec![1, 2, 3], Box::new(crabe_decision::strategy::testing::Aligned::new(vec![4])));
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![4, 1, 2, 3]);
    /// bigbro.move_bots_to_new_strategy(vec![1, 2, 3], Box::new(crabe_decision::strategy::testing::GoLeft::new(5)));
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![4, 1, 2, 3]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![5]); // strategy can only have one robot
    /// bigbro.move_bots_to_new_strategy(vec![5, 0], Box::new(crabe_decision::strategy::formations::Stop::new(vec![2])));
    /// assert_eq!(bigbro.strategies.len(), 2);
    /// assert_eq!(bigbro.strategies[0].as_ref().get_ids(), vec![4, 1, 3]);
    /// assert_eq!(bigbro.strategies[1].as_ref().get_ids(), vec![2, 5, 0]);
    /// bigbro.move_bots_to_new_strategy(vec![0, 1, 2, 3, 4, 5],Box::new(crabe_decision::strategy::formations::Stop::new(vec![])));
    /// assert_eq!(bigbro.strategies.len(), 1);
    /// ```
    pub fn move_bots_to_new_strategy(&mut self, bot_ids: Vec<u8>, mut strategy: Box<dyn Strategy>) {
        for bot_id in bot_ids {
            // if the bot is not in the new strategy, add it
            if !strategy.get_ids().contains(&bot_id) {
                let mut new_strategy_ids = strategy.get_ids();
                new_strategy_ids.push(bot_id);
                strategy.put_ids(new_strategy_ids);
            }
        }
        for bot_id in strategy.get_ids() {
            self.remove_bot_from_strategies(bot_id);
        }
        self.strategies.push(strategy);
    }

    
    /// Processes the messages received from the strategies and updates the strategies accordingly.
    ///
    /// # Arguments
    /// - `messages`: A list of `MessageData` instances containing the messages received from the strategies.
    pub fn process_messages(&mut self, messages: Vec<MessageData>) {
        messages.iter().for_each(|m| {
            match &m.message {
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
                Message::AttackerMessage(attacker_message) => {
                    match &attacker_message {
                        AttackerMessage::WantToPassBallTo(receiver_id, passing_trajectory) => {
                            if let Some(receiver_current_strategy) = self.get_bot_current_strategy(*receiver_id) {
                                if receiver_current_strategy.name() == "Receiver" {
                                    return;
                                }
                            } 
                            let receiver_strategy = Box::new(Receiver::new(*receiver_id, m.id, *passing_trajectory));
                            self.move_bot_to_new_strategy(*receiver_id, receiver_strategy);
                        }
                        AttackerMessage::NoNeedReceiver => {
                            if let Some(receiver_current_strategy_index) = self.get_index_strategy_with_name("Receiver"){
                                self.strategies.remove(receiver_current_strategy_index);
                            }
                        }
                        AttackerMessage::BallPassed(receiver_id) => {
                            if let Some(receiver_current_strategy_index) = self.get_index_strategy_with_name("Receiver"){
                                self.strategies.remove(receiver_current_strategy_index);
                            }
                            if let Some(attacker_current_strategy_index) = self.get_index_strategy_with_name("Attacker"){
                                self.strategies.remove(attacker_current_strategy_index);
                            }
                            let strategy = Box::new(Attacker::new(*receiver_id));
                            self.move_bot_to_new_strategy(*receiver_id, strategy);
                        }
                    }
                }
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
                HaltedState::GameNotStarted => prepare_start(self, world),
                HaltedState::Halt => everyone_halt(self, world),
                HaltedState::Timeout(_team) => everyone_halt(self, world),
            }
            GameState::Stopped(stopped_state) => match stopped_state {
                StoppedState::Stop => everyone_stop(self, world),
                StoppedState::PrepareKickoff(team) => prepare_kick_off(self, world, team),
                StoppedState::PreparePenalty(_team) =>  everyone_stop_except_keeper(self, world),
                StoppedState::BallPlacement(_team) =>  everyone_halt(self, world),
                StoppedState::PrepareForGameStart => prepare_start(self, world),
                StoppedState::BallLeftFieldTouchLine(_) =>   everyone_halt(self, world),
                StoppedState::CornerKick(team) => if team == world.team_color{
                    run_state(self, world, tools_data);
                }else{
                    everyone_stop_except_keeper(self, world);
                },
                StoppedState::GoalKick(_team) => run_state(self, world, tools_data),
                StoppedState::AimlessKick(_) => everyone_halt(self, world),
                StoppedState::NoProgressInGame => run_state(self, world, tools_data),
                StoppedState::PrepareFreekick(_) => everyone_stop_except_keeper(self, world),
                StoppedState::FoulStop => run_state(self, world, tools_data),
            },
            GameState::Running(running_state) => match running_state {
                RunningState::KickOff(team) => if team == world.team_color{
                    run_state(self, world, tools_data);
                }else{
                    prepare_kick_off(self, world, team);
                },
                RunningState::Penalty(team) => if team == world.team_color{
                    run_state(self, world, tools_data);
                }else{
                    everyone_stop_except_keeper(self, world);
                },
                RunningState::FreeKick(team) => if team == world.team_color{
                    run_state(self, world, tools_data);
                }else{
                    everyone_stop_except_keeper(self, world);
                },
                RunningState::Run => run_state(self, world, tools_data),
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
