use chrono::{DateTime, Duration, Utc};
use crabe_framework::data::event::GameEvent;
use crabe_framework::data::output::Command;
use crabe_framework::data::world::{Team, TeamColor};
use nalgebra::Point2;

pub enum MatchType {
    /// Not set
    UnknownMatch,
    /// Match is part of the group phase
    GroupPhase,
    /// Match is part of the elimination phase
    EliminationPhase,
    /// Friendly match, not part of a tournament
    Friendly,
}

/// The `Referee` struct contains various information about the referee of a match.
pub struct Referee {
    /// A random UUID of the referee that is kept constant while running.
    pub source_identifier: Option<String>,
    /// Meta information about the current match that helps to process the logs after a competition.
    pub match_type: Option<MatchType>,
    /// The UNIX timestamp when the packet was sent, in microseconds.
    pub packet_timestamp: DateTime<Utc>,
    /// Represent the current stage of the match.
    pub stage: Stage,
    /// The number of microseconds left in the stage.
    /// Some stage have this value :
    /// - NORMAL_FIRST_HALF, NORMAL_HALF_TIME, NORMAL_SECOND_HALF
    /// - EXTRA_TIME_BREAK EXTRA_FIRST_HALF EXTRA_HALF_TIME EXTRA_SECOND_HALF
    /// - PENALTY_SHOOTOUT_BREAK
    /// If the stage runs over its specified time, this value becomes negative.
    pub stage_time_left: Option<Duration>,
    /// Represent the current command sent.
    pub command: RefereeCommand,
    /// The number of commands issued since startup (mod 2^32).
    pub command_counter: u32,
    /// The UNIX timestamp when the command was issued, in microseconds.
    pub command_timestamp: DateTime<Utc>,
    pub ally: Team,
    pub enemy: Team,
    pub designated_position: Point2<f64>,
    pub positive_half: Option<TeamColor>,
    pub next_command: Option<Command>,
    pub game_events: Vec<GameEvent>,
    pub game_event_proposals: Vec<GameEventProposalGroup>,
    pub current_action_time_remaining: Option<Duration>,
}

pub struct GameEventProposalGroup {
    pub game_event: Vec<GameEvent>,
    pub accepted: Option<bool>,
}
/// `Stage` represents different stages of a game.
pub enum Stage {
    /// Indicates that the first half is about to start.
    NormalFirstHalfPre,
    /// Indicates the first half of the normal game, before half time.
    NormalFirstHalf,
    /// Indicates the half time period between the first and second halves.
    NormalHalfTime,
    /// Indicates that the second half is about to start.
    NormalSecondHalfPre,
    /// Indicates the second half of the normal game, after half time.
    NormalSecondHalf,
    /// Represents the break before extra time.
    ExtraTimeBreak,
    /// Indicates that the first half of the extra time is about to start.
    ExtraFirstHalfPre,
    /// Represents the first half of the extra time.
    ExtraFirstHalf,
    /// Indicates the half time period between the first and second extra halves.
    ExtraHalfTime,
    /// Indicates that the second half of extra time is about to start.
    ExtraSecondHalfPre,
    /// Represents the second half of the extra time.
    ExtraSecondHalf,
    /// Represents the break period before the penalty shootout.
    PenaltyShootoutBreak,
    /// Represents the penalty shootout stage.
    PenaltyShootout,
    /// Indicates that the game has ended.
    PostGame,
}

pub enum RefereeCommand {
    Halt,
    Stop,
    NormalStart,
    ForceStart,
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    DirectFree(TeamColor),
    Timeout(TeamColor),
    BallPlacement(TeamColor),
}
