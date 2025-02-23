use crate::data::world::TeamColor;
use serde::Serialize;

/// Defines the possible game states of the match.
/// Additional states that are not specified in the rulebook
/// have been added to help take decisions in strategy making.
/// These additional states are marked with the tag [Enriched]
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
    
    /// [Enriched]
    /// The ball left the field via a goal line, and will lead to
    /// a corner kick that will be performed by `TeamColor`
    PrepareCornerKick(TeamColor),
    
    /// [Enriched]
    /// The ball left the field via a goal line, and will lead to
    /// a goal kick that will be performed by `TeamColor`
    PrepareGoalKick(TeamColor)
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

    /// Generic running command, when no special event has occurred.
    /// Can be issued manually
    Run,
    
    /// [Enriched]
    /// The team `TeamColor` is performing a corner kick
    CornerKick(TeamColor),
    
    /// [Enriched]
    /// The team `TeamColor` is performing a goal kick
    GoalKick(TeamColor)
}