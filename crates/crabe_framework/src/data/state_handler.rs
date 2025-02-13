pub mod game_state_handler;

use std::time::Instant;
use nalgebra::Point2;
use crate::data::referee::{Referee, RefereeCommand};
use crate::data::world::game_state::{GameState, HaltedState};
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
    /// - `time_based_refresh`        | Set to true if we need to refresh the state constantly
    ///                                 (for example, a free kick state is time based, we need to update
    ///                                 the state again to check if we switch to another state or not)
    /// - `previous_state_data`       | Stores information about the latest
    ///                                 valid state we encountered. It is a capture
    ///                                 of the *previous* state. Responsible to modify
    ///                                 this with updated state data, if required (used to remember
    ///                                 if first kickoff occurred)
    ///
    /// Returns
    /// - The game state in which we are in
    fn process_state(&self,
                     world: &World,
                     referee: &Referee,
                     time_based_refresh: &mut bool,
                     previous_state_data: &mut GameStateData) -> GameState;
}

/// This struct contains the strict minimum required
/// to help us determine what will be the next valid
/// state of the match, depending on the commands issued
/// from the referee
pub struct GameStateData {
    /// Whether the first kickoff already occurred or not
    pub kicked_off_once: bool,
    /// The previous referee command, different from the current one
    pub prev_ref_cmd: RefereeCommand,
    /// The most recent referee command issued
    pub last_ref_cmd: RefereeCommand,
    /// Latest designated position provided by the referee
    pub last_designated_pos: Option<Point2<f64>>,
    /// Last saved score of the ally team
    pub ally_score: u32,
    /// Last saved score of the enemy team
    pub enemy_score: u32,
}

impl Default for GameStateData {
    fn default() -> Self {
        Self {
            kicked_off_once: false,
            prev_ref_cmd: RefereeCommand::Halt,
            last_ref_cmd: RefereeCommand::Halt,
            last_designated_pos: Some(Point2::new(0., 0.)),
            ally_score: 0,
            enemy_score: 0,
        }
    }
}