use std::vec;

use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::message::Message;
use crate::message::MessageData;
use crate::strategy::testing::Aligned;
use crate::strategy::testing::GoLeft;
use crate::strategy::testing::GoRight;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

/// The `BigBro` struct represents a decision manager that executes strategies BigBroly
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `BigBro` struct.
#[derive(Default)]
pub struct BigBro {
    strategies: Vec<Box<dyn Strategy>>,
}

impl BigBro {
    /// Creates a new `BigBro` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            strategies: vec![
                Box::new(GoLeft::new(1)),
                Box::new(Aligned::new(vec![2, 3, 4])),
            ],
        }
    }

    /// Moves a bot from its current strategy to an existing strategy.
    ///
    /// # Arguments
    /// - `bot_id`: The id of the bot to move.
    /// - `strategy_index`: The index of the strategy (in the strategies list) to move the bot to.
    pub fn move_bot_to_existing_strategy(&mut self, bot_id: u8, strategy_index: usize) {
        let bot_current_strategy_index = self
            .strategies
            .iter()
            .position(|s| s.get_ids().contains(&bot_id))
            .unwrap();
        let mut new_strategy_ids = self.strategies[strategy_index].as_ref().get_ids();
        new_strategy_ids.push(bot_id);
        self.strategies[strategy_index].put_ids(new_strategy_ids);

        let mut current_strategy_ids = self.strategies[bot_current_strategy_index]
            .as_ref()
            .get_ids();
        if current_strategy_ids.len() == 1 {
            self.strategies.remove(bot_current_strategy_index);
        } else {
            current_strategy_ids.retain(|&id| id != bot_id);
            self.strategies[bot_current_strategy_index].put_ids(current_strategy_ids);
        }
    }

    pub fn move_bot_to_new_strategy(&mut self, bot_id: u8, strategy: Box<dyn Strategy>) {
        let current_strategy_index = self
            .strategies
            .iter()
            .position(|s| s.get_ids().contains(&bot_id))
            .unwrap();
        let mut ids = self.strategies[current_strategy_index].as_ref().get_ids();
        let index_of_bot_in_slot_ids = ids.iter().position(|x| x == &bot_id).unwrap();
        ids.remove(index_of_bot_in_slot_ids);
        if ids.len() == 0 {
            //if the bot was the alone in this strategy, we can replace it
            self.strategies[current_strategy_index] = strategy;
        } else {
            self.strategies[current_strategy_index].put_ids(ids);
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
                    let strategy_index = self
                        .strategies
                        .iter()
                        .position(|s| s.name() == "Aligned")
                        .unwrap();
                    self.move_bot_to_existing_strategy(m.id, strategy_index);
                }
                _ => {}
            }
        });
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
