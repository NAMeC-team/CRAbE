use chrono::Duration;
use nalgebra::Point2;
use serde::Serialize;
use crate::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet::referee::Point;

#[derive(Clone, Debug)]
pub enum EventOrigin {
    GameController,
    Autorefs(Vec<String>)
}

#[derive(Clone, Debug)]
pub struct BallLeftField {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct AimlessKick {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub kick_location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct Goal {
    pub by_team: TeamColor,
    pub kicking_team: Option<TeamColor>,
    pub kicking_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub kick_location: Option<Point2<f64>>,
    pub max_ball_height: Option<f64>,
    pub num_bots_by_team: Option<u32>,
    pub last_touch_by_team: Option<u64>,
    pub message: Option<String>
}

#[derive(Clone, Debug)]
pub struct BotTooFastInStop {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub speed: Option<f64>
}

#[derive(Clone, Debug)]
pub struct DefenderTooCloseToKickPoint {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>
}

#[derive(Clone, Debug)]
pub struct BotCrashDrawn {
    pub bot_blue: Option<u32>,
    pub bot_yellow: Option<u32>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>
}

#[derive(Clone, Debug)]
pub struct BotCrashUnique {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>
}

#[derive(Clone, Debug)]
pub struct BotPushedBot {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub pushed_distance: Option<f64>
}

#[derive(Clone, Debug)]
pub struct BotTippedOver {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub ball_location: Option<Point2<f64>>
}

#[derive(Clone, Debug)]
pub struct DefenderInDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct DefenderInDefenseAreaPartially {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
    pub ball_location: Option<Point2<f64>>
}


#[derive(Clone, Debug)]
pub struct AttackerTouchedBallInDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct BotKickedBallTooFast {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub initial_ball_speed: Option<f64>,
    pub chipped: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct BotDribbledBallTooFar {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub start: Option<Point2<f64>>,
    pub end: Option<Point2<f64>>
}


#[derive(Clone, Debug)]
pub struct AttackerDoubleTouchedBall {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>
}

#[derive(Clone, Debug)]
pub struct AttackerTooCloseToDefenseArea {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
    pub ball_location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct BotHeldBallDeliberately {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct BotInterferedPlacement {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct MultipleFouls {
    pub by_team: TeamColor,
    pub caused_game_events: Vec<GameEvent>,
}

#[derive(Clone, Debug)]
pub struct NoProgressInGame {
    pub location: Option<Point2<f64>>,
    pub time: Option<Duration>
}

#[derive(Clone, Debug)]
pub struct PlacementFailed {
    pub location: Option<Point2<f64>>,
    pub remaining_distance: f64,
}

#[derive(Clone, Debug)]
pub struct UnsportingBehaviorMinor {
    pub by_team: TeamColor,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct UnsportingBehaviorMajor {
    pub by_team: TeamColor,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct KeeperHeldBall {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>,
    pub duration: Option<Duration>
}

#[derive(Clone, Debug)]
pub struct PlacementSucceeded {
    pub by_team: TeamColor,
    pub time_taken: Option<f64>,
    pub precision: Option<f64>,
    pub distance: Option<f64>
}

#[derive(Clone, Debug)]
pub struct TooManyRobots {
    pub by_team: TeamColor,
    pub num_robots_allowed: Option<u32>,
    pub num_robots_on_field: Option<u32>,
    pub ball_location: Option<Point2<f64>>
}


#[derive(Clone, Debug)]
pub struct BoundaryCrossing {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>
}

#[derive(Clone, Debug)]
pub struct PenaltyKickFailed {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>
}

#[derive(Clone, Debug)]
pub enum GameEvent {
    Unknown,
    BallLeftFieldTouchLine(BallLeftField),
    BallLeftFieldGoalLine(BallLeftField),
    AimlessKick(AimlessKick),
    AttackerTooCloseToDefenseArea(AttackerTooCloseToDefenseArea),
    DefenderInDefenseArea(DefenderInDefenseArea),
    BoundaryCrossing(BoundaryCrossing),
    KeeperHeldBall(KeeperHeldBall),
    BotDribbledBallTooFar(BotDribbledBallTooFar),
    BotPushedBot(BotPushedBot),
    BotHeldBallDeliberately(BotHeldBallDeliberately),
    BotTippedOver(BotTippedOver),
    AttackerTouchedBallInDefenseArea(AttackerTouchedBallInDefenseArea),
    BotKickedBallTooFast(BotKickedBallTooFast),
    BotCrashUnique(BotCrashUnique),
    BotCrashDrawn(BotCrashDrawn),
    DefenderTooCloseToKickPoint(DefenderTooCloseToKickPoint),
    BotTooFastInStop(BotTooFastInStop),
    BotInterferedPlacement(BotInterferedPlacement),
    PossibleGoal(Goal),
    Goal(Goal),
    InvalidGoal(Goal),
    AttackerDoubleTouchedBall(AttackerDoubleTouchedBall),
    PlacementSucceeded(PlacementSucceeded),
    PenaltyKickFailed(PenaltyKickFailed),
    NoProgressInGame(NoProgressInGame),
    PlacementFailed(PlacementFailed),
    MultipleCards(TeamColor),
    MultipleFouls(MultipleFouls),
    TooManyRobots(TooManyRobots),
    BotSubstitution(TeamColor),
    ChallengeFlag(TeamColor),
    EmergencyStop(TeamColor),
    UnsportingBehaviorMinor(UnsportingBehaviorMinor),
    UnsportingBehaviorMajor(UnsportingBehaviorMajor),
}