pub mod game_state_handler;

use std::time::Instant;
use crate::data::referee::{Referee, RefereeCommand};
use crate::data::world::game_state::GameState;
use crate::data::world::{TeamColor, World};

/// This trait defines how each game state should be handled
/// for each possible game state that can be issued by the
/// referee. Because the commands issued by the referee are limited,
/// it is up to us to determine what state we are playing in.
/// This is the grey part of the game, where you have to understand
/// what's happening as per the rulebook.
pub trait GameStateBranch {
    /// Process a single referee command, that changes the state
    /// of the game, and returns the new state of the match
    ///
    /// Thus, to determine the state we are currently in,
    /// you can access the data issued by the referee,
    /// and the data of the previous state we were in
    ///
    /// Parameters :
    /// - `world`                     | Information on the world
    /// - `referee`                   | The current data issued from the referee
    /// - `timer`                     | Used for specific events. Set to None if it is not in use
    /// - `previous_state_data`       | Stores information about the latest
    ///                             valid state we encountered. It is a capture
    ///                             of the *previous* state
    ///
    /// Returns
    /// - The game state in which we are in
    fn process_state(&self,
                     world: &World,
                     referee: &Referee,
                     timer_opt: &mut Option<Instant>,
                     previous_state_data: &StateData) -> GameState;
}

/// This struct contains the strict minimum required
/// to help us determine what will be the next valid
/// state of the match, depending on the commands issued
/// from the referee
pub struct StateData {
    /// Whether the first kickoff already occurred or not
    pub kicked_off_once: bool,
    /// The previous referee command, different from the current one
    pub prev_ref_cmd: RefereeCommand,
    /// Last saved score of the ally team
    pub ally_score: u32,
    /// Last saved score of the enemy team
    pub enemy_score: u32,
}

impl Default for StateData {
    fn default() -> Self {
        Self {
            kicked_off_once: false,
            prev_ref_cmd: RefereeCommand::Halt,
            ally_score: 0,
            enemy_score: 0,
        }
    }
}