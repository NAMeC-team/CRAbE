use serde::Serialize;
use crate::data::world::TeamColor;

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum GameState {
    Halted(HaltedState),
    Stopped(StoppedState),
    Running(RunningState)
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum HaltedState {
    Halt,
    Timeout
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum StoppedState {
    Stop,
    PrepareKickoff(TeamColor),
    PreparePenalty(TeamColor),
    BallPlacement(TeamColor)
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum RunningState {
    KickOff(TeamColor),
    Penalty(TeamColor),
    FreeKick(TeamColor),
    Run
}
