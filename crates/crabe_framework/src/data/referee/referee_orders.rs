use serde::Serialize;
use crate::data::referee::event::GameEvent;
use crate::data::world::game_state::{GameState, HaltedState};

/// Retains information sent by the game controller
/// to both teams, about the current game state,
/// the maximum speed allowed
#[derive(Serialize, Clone, Debug)]
pub struct RefereeOrders {
    /// Current game state
    state: GameState,
    /// Latest event that occurred on the field
    event: Option<GameEvent>,
    /// Maximum speed limit authorized for all robots
    /// Unit is in meters per second (m/s)
    speed_limit: f32,
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

    /// Creates a new instance, that defines the speed limits
    /// depending on the current game state provided
    pub fn new(game_state: GameState, game_event: Option<GameEvent>) -> Self {
        Self {
            state: game_state,
            event: game_event,
            speed_limit: Self::get_speed_limit_during(game_state),
        }
    }
}

impl Default for RefereeOrders {
    fn default() -> Self {
        Self {
            state: GameState::Halted(HaltedState::GameNotStarted),
            event: None,
            speed_limit: 0.0,
        }
    }
}