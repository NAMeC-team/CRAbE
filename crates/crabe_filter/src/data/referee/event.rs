use chrono::Duration;
use crabe_framework::data::world::TeamColor;
use nalgebra::Point2;

#[derive(Clone, Debug)]
pub enum EventOrigin {
    GameController,
    Autorefs(Vec<String>),
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
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BotTooFastInStop {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub speed: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct DefenderTooCloseToKickPoint {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub distance: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct BotCrashDrawn {
    pub bot_blue: Option<u32>,
    pub bot_yellow: Option<u32>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct BotCrashUnique {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub crash_speed: Option<f64>,
    pub speed_diff: Option<f64>,
    pub crash_angle: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct BotPushedBot {
    pub by_team: TeamColor,
    pub violator: Option<u32>,
    pub victim: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub pushed_distance: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct BotTippedOver {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
    pub ball_location: Option<Point2<f64>>,
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
    pub ball_location: Option<Point2<f64>>,
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
    pub end: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct AttackerDoubleTouchedBall {
    pub by_team: TeamColor,
    pub by_bot: Option<u32>,
    pub location: Option<Point2<f64>>,
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
    pub caused_game_events: Vec<OldGameEvent>,
}

#[derive(Clone, Debug)]
pub struct NoProgressInGame {
    pub location: Option<Point2<f64>>,
    pub time: Option<Duration>,
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
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct PlacementSucceeded {
    pub by_team: TeamColor,
    pub time_taken: Option<f64>,
    pub precision: Option<f64>,
    pub distance: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct TooManyRobots {
    pub by_team: TeamColor,
    pub num_robots_allowed: Option<u32>,
    pub num_robots_on_field: Option<u32>,
    pub ball_location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct BoundaryCrossing {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct PenaltyKickFailed {
    pub by_team: TeamColor,
    pub location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub enum OldGameEvent {
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
////////////////////////

/// GameEvent contains exactly one game event.
/// Each game event has optional and required fields.
pub struct GameEvent {
    /// Event type of this Game
    type_event: Option<GameEventType>,
    /// The origins of this game event.
    /// Empty, if it originates from game controller.
    origin: Vec<String>,
    /// Unix timestamp in microseconds when the event was created.
    created_timestamp: Option<u64>,
    /// the event that occurred
    event: Event,
}

/// All game event type.
/// See the protobuf message inside the crate `crabe_protocol` to see which game event is triggered by gc, auto_referee and human.
pub enum GameEventType {
    UnknownGameEventType,
    BallLeftFieldTouchLine,
    BallLeftFieldGoalLine,
    AimlessKick,
    AttackerTooCloseToDefenseArea,
    DefenderInDefenseArea,
    BoundaryCrossing,
    KeeperHeldBall,
    BotDribbledBallTooFar,
    BotPushedBot,
    BotHeldBallDeliberately,
    BotTippedOver,
    AttackerTouchedBallInDefenseArea,
    BotKickedBallTooFast,
    BotCrashUnique,
    BotCrashDrawn,
    DefenderTooCloseToKickPoint,
    BotTooFastInStop,
    BotInterferedPlacement,
    PossibleGoal,
    Goal,
    InvalidGoal = 42,
    AttackerDoubleTouchedBall,
    PlacementSucceeded,
    PenaltyKickFailed,
    NoProgressInGame,
    PlacementFailed,
    MultipleCards,
    MultipleFouls,
    BotSubstitution,
    TooManyRobots,
    ChallengeFlag,
    ChallengeFlagHandled,
    EmergencyStop,
    UnsportingBehaviorMinor,
    UnsportingBehaviorMajor,
    /// Since this place, all of this field is deprecated !
    Prepared,
    IndirectGoal,
    ChippedGoal,
    KickTimeout,
    AttackerTouchedOpponentInDefenseArea,
    AttackerTouchedOpponentInDefenseAreaSkipped,
    BotCrashUniqueSkipped,
    BotPushedBotSkipped,
    DefenderInDefenseAreaPartially,
    MultiplePlacementFailures,
}

pub enum Event {
    // Ball out of field events (Stopping)
    BallLeftFieldTouchLine(BallLeftField),
    BallLeftFieldGoalLine(BallLeftField),
    AimlessKick(AimlessKick),

    // Stopping fouls
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

//////////////////////////////////////////////////////
//               Event Struct Type                  //
//////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct BallLeftField {
    /// The team that last touched the ball.
    pub by_team: TeamColor,
    /// The bot that last touched the ball.
    pub by_bot: Option<u32>,
    /// The location where the ball left the field (in meters).
    pub location: Option<Point2<f64>>,
}

#[derive(Clone, Debug)]
pub struct AimlessKick {
    /// The team that last touched the ball
    pub by_team: TeamColor,
    /// The bot that last touched the ball.
    pub by_bot: Option<u32>,
    /// The location where the ball left the field (in meters).
    pub location: Option<Point2<f64>>,
    /// The location where the ball was last touched (in meters).
    pub kick_location: Option<Point2<f64>>,
}

/// Represents an event where an attacking robot is located too close to the opponent's defense area during a stoppage or free kick.
#[derive(Clone, Debug)]
pub struct AttackerTooCloseToDefenseArea {
    /// The team that found guilty.
    pub by_team: TeamColor,
    /// The bot that is too close to the defense area
    pub by_bot: Option<u32>,
    /// The location of the bot (in meters)
    pub location: Option<Point2<f64>>,
    /// The distance of the bot to the penalty area (in meters).
    pub distance: Option<f64>,
    /// The location of the ball at the moment when this foul occurred (in meters)
    pub ball_location: Option<Point2<f64>>,
}
