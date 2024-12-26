use crate::data::world::TeamColor;
use serde::Serialize;

/// Defines the possible game states of the match
/// Some states are not associated with commands sent from the referee
/// These states are marked with the [Non-official] tag
///
/// Some commands and events link to a similar state,
/// for example some fouls lead to a Stop state.
/// When states are regrouped together, they are marked
/// by the [Collection] tag
///
/// The [Extension] tag means that there was more than one possibility for an
/// event, and that we required to split it into multiple states
/// For example, let's say the ball leaves the field by
/// touching a goal line, and the ball was played on the side of Team A,
/// the opponent being Team B. This event can lead to either a goal kick
/// (if it is Team B's fault) or a corner kick (if it is Team A's fault)
#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameState {
    Halted(HaltedState),
    Stopped(StoppedState),
    Running(RunningState),
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HaltedState {
    /// A halt command has been issued
    Halt,
    /// A team is having a timeout
    Timeout(TeamColor),
}

/// A list of the most common Stop states that may occur
/// during a game. Note that some states are behind a collection
/// FoulStop, because they occur more rarely (such as a bot tipping over)
/// The most common ones have been taken out of the collection, like the
/// ball leaves field events. This is designed to explicitly define what
/// the strategies should focus on, and what they can safely "ignore"
/// to play a full game properly.
///
/// This distinction is purely subjective, but also comes from experience
/// TODO: the layout of the `TeamColor` values is weird
///  sometimes, the TeamColor is the team responsible for the error
///  sometimes it's the opposite. Which one is better ?
#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StoppedState {
    
    /// The team `TeamColor` is going to do their kickoff.
    /// dev note: we use this state directly after a goal is scored
    PrepareKickoff(TeamColor),

    /// The team `TeamColor` must prepare for a penalty kick
    PreparePenalty(TeamColor),

    /// The team `TeamColor` is trying to place the ball automatically
    /// without the help of a human to pursue the game
    BallPlacement(TeamColor),

    /// Generic stop command, issued when robots must slow down after
    /// a foul, for example. Can be issued manually
    Stop,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RunningState {
    /// The team `TeamColor` is doing their kickoff
    /// Everyone can move, but only `TeamColor` is allowed
    /// to perform the first ball touch
    KickOff(TeamColor),

    /// The team `TeamColor` has a robot ready to score a penalty
    /// towards the goalkeeper of the enemy team
    Penalty(TeamColor),

    /// The team `TeamColor` can freely kick the ball once
    FreeKick(TeamColor),

    /// Generic running command, when no special event has occurred
    /// Can be issued manually
    Run,
}