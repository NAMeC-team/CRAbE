use serde::Serialize;
use crate::data::referee::event::GameEvent;
use crate::data::world::game_state::{GameState, HaltedState};

/// Retains information sent by the game controller
/// to both teams, about the current game state,
/// the maximum speed allowed
#[derive(Serialize, Clone, Debug)]
pub struct RefereeOrders {
    /// Current game state
    pub state: GameState,
    /// Latest event that occurred on the field
    pub event: Option<GameEvent>,
    /// Maximum speed limit authorized for all robots
    /// Unit is in meters per second (m/s)
    pub speed_limit: f32,
    /// Minimum distance to stay away from the ball
    /// There might not be any distance required, for example
    /// during a normal running state
    pub min_dist_from_ball: Option<f32>,
}

const MAX_SPEED_HALTED: f32 = 0.;
const MAX_SPEED_STOPPED: f32 = 1.5;
//TODO: use MAX_LINEAR constant ? (can't because of circular dependency
// between crabe_framework and crabe_guard)
const MAX_SPEED_RUNNING: f32 = 6.; // Arbitrary value, not defined by the rulebook

impl RefereeOrders {
    /// Get the maximum speed authorized during a given game state
    /// There are no specific speed limits for certain events,
    /// such as a penalty.
    /// Speed limits are only defined for three main types of game states
    pub fn get_speed_limit_during(game_state: GameState) -> f32 {
        match game_state {
            GameState::Halted(_) => MAX_SPEED_HALTED,
            GameState::Stopped(_) => MAX_SPEED_STOPPED,
            GameState::Running(_) => MAX_SPEED_RUNNING,
        }
    }
    
    /// Get the minimum distance our robots have to stay away
    /// from the ball during a given state
    pub fn get_min_dist_from_ball_during(game_state: GameState) -> Option<f32> {
        match game_state {
            GameState::Halted(_) => None,
            GameState::Stopped(_) => Some(1.5),
            GameState::Running(_) => None,
        }
    }

    /// Creates a new instance, that defines the speed limits
    /// depending on the current game state provided
    pub fn new(game_state: GameState, game_event: Option<GameEvent>) -> Self {
        Self {
            state: game_state,
            event: game_event,
            speed_limit: Self::get_speed_limit_during(game_state),
            min_dist_from_ball: None,
        }
    }

    /// Updates the struct with the new information provided
    /// Convenience function to avoid having to create/drop similar objects
    pub fn update(&mut self, game_state: GameState, game_event: Option<&GameEvent>) {
        self.state = game_state;
        self.speed_limit = Self::get_speed_limit_during(game_state);

        // dev note : this one is a bit weird
        // it's either this or putting lifetimes onto structs (notably on the `World` struct)
        self.event = match game_event {
            None => None,
            Some(ge_ref) => { Some(ge_ref.clone()) }
        };
    }
}

impl Default for RefereeOrders {
    fn default() -> Self {
        Self {
            state: GameState::Halted(HaltedState::GameNotStarted),
            event: None,
            speed_limit: MAX_SPEED_HALTED,
            min_dist_from_ball: None,
        }
    }
}