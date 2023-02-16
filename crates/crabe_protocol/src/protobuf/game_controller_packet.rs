/// RobotId is the combination of a team and a robot id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotId {
    /// the robot number
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    /// the team that the robot belongs to
    #[prost(enumeration = "Team", optional, tag = "2")]
    pub team: ::core::option::Option<i32>,
}
/// Team is either blue or yellow
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Team {
    /// team not set
    Unknown = 0,
    /// yellow team
    Yellow = 1,
    /// blue team
    Blue = 2,
}
impl Team {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Team::Unknown => "UNKNOWN",
            Team::Yellow => "YELLOW",
            Team::Blue => "BLUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "YELLOW" => Some(Self::Yellow),
            "BLUE" => Some(Self::Blue),
            _ => None,
        }
    }
}
/// Division denotes the current division, which influences some rules
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Division {
    DivUnknown = 0,
    DivA = 1,
    DivB = 2,
}
impl Division {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Division::DivUnknown => "DIV_UNKNOWN",
            Division::DivA => "DIV_A",
            Division::DivB => "DIV_B",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIV_UNKNOWN" => Some(Self::DivUnknown),
            "DIV_A" => Some(Self::DivA),
            "DIV_B" => Some(Self::DivB),
            _ => None,
        }
    }
}
/// A vector with two dimensions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2 {
    #[prost(float, required, tag = "1")]
    pub x: f32,
    #[prost(float, required, tag = "2")]
    pub y: f32,
}
/// A vector with three dimensions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(float, required, tag = "1")]
    pub x: f32,
    #[prost(float, required, tag = "2")]
    pub y: f32,
    #[prost(float, required, tag = "3")]
    pub z: f32,
}
/// GameEvent contains exactly one game event
/// Each game event has optional and required fields. The required fields are mandatory to process the event.
/// Some optional fields are only used for visualization, others are required to determine the ball placement position.
/// If fields are missing that are required for the ball placement position, no ball placement command will be issued.
/// Fields are marked optional to make testing and extending of the protocol easier.
/// An autoRef should ideally set all fields, except if there are good reasons to not do so.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameEvent {
    #[prost(enumeration = "game_event::Type", optional, tag = "40")]
    pub r#type: ::core::option::Option<i32>,
    /// The origins of this game event.
    /// Empty, if it originates from game controller.
    /// Contains autoRef name(s), if it originates from one or more autoRefs.
    /// Ignored if sent by autoRef to game controller.
    #[prost(string, repeated, tag = "41")]
    pub origin: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the event that occurred
    #[prost(
        oneof = "game_event::Event",
        tags = "6, 7, 11, 19, 31, 43, 13, 17, 24, 26, 27, 15, 18, 22, 21, 29, 28, 20, 39, 8, 44, 14, 5, 45, 2, 3, 32, 34, 37, 38, 46, 47, 35, 36, 1, 9, 10, 12, 16, 42, 23, 25, 30, 33"
    )]
    pub event: ::core::option::Option<game_event::Event>,
}
/// Nested message and enum types in `GameEvent`.
pub mod game_event {
    /// the ball left the field normally
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BallLeftField {
        /// the team that last touched the ball
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that last touched the ball
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location where the ball left the field \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    /// the ball left the field via goal line and a team committed an aimless kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AimlessKick {
        /// the team that last touched the ball
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that last touched the ball
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location where the ball left the field \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the location where the ball was last touched \[m\]
        #[prost(message, optional, tag = "4")]
        pub kick_location: ::core::option::Option<super::Vector2>,
    }
    /// a team shot a goal
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Goal {
        /// the team that scored the goal
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the team that shot the goal (different from by_team for own goals)
        #[prost(enumeration = "super::Team", optional, tag = "6")]
        pub kicking_team: ::core::option::Option<i32>,
        /// the bot that shot the goal
        #[prost(uint32, optional, tag = "2")]
        pub kicking_bot: ::core::option::Option<u32>,
        /// the location where the ball entered the goal \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the location where the ball was kicked (for deciding if this was a valid goal) \[m\]
        #[prost(message, optional, tag = "4")]
        pub kick_location: ::core::option::Option<super::Vector2>,
        /// the maximum height the ball reached during the goal kick (for deciding if this was a valid goal) \[m\]
        #[prost(float, optional, tag = "5")]
        pub max_ball_height: ::core::option::Option<f32>,
        /// number of robots of scoring team when the ball entered the goal (for deciding if this was a valid goal)
        #[prost(uint32, optional, tag = "7")]
        pub num_robots_by_team: ::core::option::Option<u32>,
        /// The UNIX timestamp \[μs\] when the scoring team last touched the ball
        #[prost(uint64, optional, tag = "8")]
        pub last_touch_by_team: ::core::option::Option<u64>,
        /// An additional message with e.g. a reason for invalid goals
        #[prost(string, optional, tag = "9")]
        pub message: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// the ball entered the goal directly during an indirect free kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndirectGoal {
        /// the team that tried to shoot the goal
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that kicked the ball - at least the team must be set
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location where the ball entered the goal \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the location where the ball was kicked \[m\]
        #[prost(message, optional, tag = "4")]
        pub kick_location: ::core::option::Option<super::Vector2>,
    }
    /// the ball entered the goal, but was initially chipped
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChippedGoal {
        /// the team that tried to shoot the goal
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that kicked the ball
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location where the ball entered the goal \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the location where the ball was kicked \[m\]
        #[prost(message, optional, tag = "4")]
        pub kick_location: ::core::option::Option<super::Vector2>,
        /// the maximum height \[m\] of the ball, before it entered the goal and since the last kick \[m\]
        #[prost(float, optional, tag = "5")]
        pub max_ball_height: ::core::option::Option<f32>,
    }
    /// a bot moved too fast while the game was stopped
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotTooFastInStop {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that was too fast
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the bot speed \[m/s\]
        #[prost(float, optional, tag = "4")]
        pub speed: ::core::option::Option<f32>,
    }
    /// a bot of the defending team got too close to the kick point during a free kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefenderTooCloseToKickPoint {
        /// the team that was found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that violates the distance to the kick point
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the distance \[m\] from bot to the kick point (including the minimum radius)
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
    }
    /// two robots crashed into each other with similar speeds
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotCrashDrawn {
        /// the bot of the yellow team
        #[prost(uint32, optional, tag = "1")]
        pub bot_yellow: ::core::option::Option<u32>,
        /// the bot of the blue team
        #[prost(uint32, optional, tag = "2")]
        pub bot_blue: ::core::option::Option<u32>,
        /// the location of the crash (center between both bots) \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the calculated crash speed \[m/s\] of the two bots
        #[prost(float, optional, tag = "4")]
        pub crash_speed: ::core::option::Option<f32>,
        /// the difference \[m/s\] of the velocity of the two bots
        #[prost(float, optional, tag = "5")]
        pub speed_diff: ::core::option::Option<f32>,
        /// the angle \[rad\] in the range [0, π] of the bot velocity vectors
        /// an angle of 0 rad (  0°) means, the bots barely touched each other
        /// an angle of π rad (180°) means, the bots crashed frontal into each other
        #[prost(float, optional, tag = "6")]
        pub crash_angle: ::core::option::Option<f32>,
    }
    /// two robots crashed into each other and one team was found guilty to due significant speed difference
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotCrashUnique {
        /// the team that caused the crash
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that caused the crash
        #[prost(uint32, optional, tag = "2")]
        pub violator: ::core::option::Option<u32>,
        /// the bot of the opposite team that was involved in the crash
        #[prost(uint32, optional, tag = "3")]
        pub victim: ::core::option::Option<u32>,
        /// the location of the crash (center between both bots) \[m\]
        #[prost(message, optional, tag = "4")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the calculated crash speed vector \[m/s\] of the two bots
        #[prost(float, optional, tag = "5")]
        pub crash_speed: ::core::option::Option<f32>,
        /// the difference \[m/s\] of the velocity of the two bots
        #[prost(float, optional, tag = "6")]
        pub speed_diff: ::core::option::Option<f32>,
        /// the angle \[rad\] in the range [0, π] of the bot velocity vectors
        /// an angle of 0 rad (  0°) means, the bots barely touched each other
        /// an angle of π rad (180°) means, the bots crashed frontal into each other
        #[prost(float, optional, tag = "7")]
        pub crash_angle: ::core::option::Option<f32>,
    }
    /// a bot pushed another bot over a significant distance
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotPushedBot {
        /// the team that pushed the other team
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that pushed the other bot
        #[prost(uint32, optional, tag = "2")]
        pub violator: ::core::option::Option<u32>,
        /// the bot of the opposite team that was pushed
        #[prost(uint32, optional, tag = "3")]
        pub victim: ::core::option::Option<u32>,
        /// the location of the push (center between both bots) \[m\]
        #[prost(message, optional, tag = "4")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the pushed distance \[m\]
        #[prost(float, optional, tag = "5")]
        pub pushed_distance: ::core::option::Option<f32>,
    }
    /// a bot tipped over
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotTippedOver {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that tipped over
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the location of the ball at the moment when this foul occurred \[m\]
        #[prost(message, optional, tag = "4")]
        pub ball_location: ::core::option::Option<super::Vector2>,
    }
    /// a defender other than the keeper was fully located inside its own defense and touched the ball
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefenderInDefenseArea {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that is inside the penalty area
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the distance \[m\] from bot case to the nearest point outside the defense area
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
    }
    /// a defender other than the keeper was partially located inside its own defense area and touched the ball
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefenderInDefenseAreaPartially {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that is partially inside the penalty area
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the distance \[m\] that the bot is inside the penalty area
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
        /// the location of the ball at the moment when this foul occurred \[m\]
        #[prost(message, optional, tag = "5")]
        pub ball_location: ::core::option::Option<super::Vector2>,
    }
    /// an attacker touched the ball inside the opponent defense area
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackerTouchedBallInDefenseArea {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that is inside the penalty area
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the distance \[m\] that the bot is inside the penalty area
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
    }
    /// a bot kicked the ball too fast
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotKickedBallTooFast {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that kicked too fast
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the ball at the time of the highest speed \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the absolute initial ball speed (kick speed) \[m/s\]
        #[prost(float, optional, tag = "4")]
        pub initial_ball_speed: ::core::option::Option<f32>,
        /// was the ball chipped?
        #[prost(bool, optional, tag = "5")]
        pub chipped: ::core::option::Option<bool>,
    }
    /// a bot dribbled to ball too far
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotDribbledBallTooFar {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that dribbled too far
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location where the dribbling started \[m\]
        #[prost(message, optional, tag = "3")]
        pub start: ::core::option::Option<super::Vector2>,
        /// the location where the maximum dribbling distance was reached \[m\]
        #[prost(message, optional, tag = "4")]
        pub end: ::core::option::Option<super::Vector2>,
    }
    /// an attacker touched the opponent robot inside defense area
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackerTouchedOpponentInDefenseArea {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that touched the opponent robot
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the bot of the opposite team that was touched
        #[prost(uint32, optional, tag = "4")]
        pub victim: ::core::option::Option<u32>,
        /// the location of the contact point between both bots \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    /// an attacker touched the ball multiple times when it was not allowed to
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackerDoubleTouchedBall {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that touched the ball twice
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the ball when it was first touched \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    /// an attacker was located too near to the opponent defense area during stop or free kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttackerTooCloseToDefenseArea {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that is too close to the defense area
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the distance \[m\] of the bot to the penalty area
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
        /// the location of the ball at the moment when this foul occurred \[m\]
        #[prost(message, optional, tag = "5")]
        pub ball_location: ::core::option::Option<super::Vector2>,
    }
    /// a bot held the ball for too long
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotHeldBallDeliberately {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that holds the ball
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the ball \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the duration \[s\] that the bot hold the ball
        #[prost(float, optional, tag = "4")]
        pub duration: ::core::option::Option<f32>,
    }
    /// a bot interfered the ball placement of the other team
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotInterferedPlacement {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the bot that interfered the placement
        #[prost(uint32, optional, tag = "2")]
        pub by_bot: ::core::option::Option<u32>,
        /// the location of the bot \[m\]
        #[prost(message, optional, tag = "3")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    /// a team collected multiple cards (yellow and red), which results in a penalty kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultipleCards {
        /// the team that received multiple yellow cards
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
    }
    /// a team collected multiple fouls, which results in a yellow card
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultipleFouls {
        /// the team that collected multiple fouls
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the list of game events that caused the multiple fouls
        #[prost(message, repeated, tag = "2")]
        pub caused_game_events: ::prost::alloc::vec::Vec<super::GameEvent>,
    }
    /// a team failed to place the ball multiple times in a row
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiplePlacementFailures {
        /// the team that failed multiple times
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
    }
    /// timeout waiting for the attacking team to perform the free kick
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KickTimeout {
        /// the team that that should have kicked
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the location of the ball \[m\]
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the time \[s\] that was waited
        #[prost(float, optional, tag = "3")]
        pub time: ::core::option::Option<f32>,
    }
    /// game was stuck
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoProgressInGame {
        /// the location of the ball
        #[prost(message, optional, tag = "1")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the time \[s\] that was waited
        #[prost(float, optional, tag = "2")]
        pub time: ::core::option::Option<f32>,
    }
    /// ball placement failed
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlacementFailed {
        /// the team that failed
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the remaining distance \[m\] from ball to placement position
        #[prost(float, optional, tag = "2")]
        pub remaining_distance: ::core::option::Option<f32>,
    }
    /// a team was found guilty for minor unsporting behavior
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsportingBehaviorMinor {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// an explanation of the situation and decision
        #[prost(string, required, tag = "2")]
        pub reason: ::prost::alloc::string::String,
    }
    /// a team was found guilty for major unsporting behavior
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnsportingBehaviorMajor {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// an explanation of the situation and decision
        #[prost(string, required, tag = "2")]
        pub reason: ::prost::alloc::string::String,
    }
    /// a keeper held the ball in its defense area for too long
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeeperHeldBall {
        /// the team that found guilty
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the location of the ball \[m\]
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::Vector2>,
        /// the duration \[s\] that the keeper hold the ball
        #[prost(float, optional, tag = "3")]
        pub duration: ::core::option::Option<f32>,
    }
    /// a team successfully placed the ball
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlacementSucceeded {
        /// the team that did the placement
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the time \[s\] taken for placing the ball
        #[prost(float, optional, tag = "2")]
        pub time_taken: ::core::option::Option<f32>,
        /// the distance \[m\] between placement location and actual ball position
        #[prost(float, optional, tag = "3")]
        pub precision: ::core::option::Option<f32>,
        /// the distance \[m\] between the initial ball location and the placement position
        #[prost(float, optional, tag = "4")]
        pub distance: ::core::option::Option<f32>,
    }
    /// both teams are prepared - all conditions are met to continue (with kickoff or penalty kick)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Prepared {
        /// the time \[s\] taken for preparing
        #[prost(float, optional, tag = "1")]
        pub time_taken: ::core::option::Option<f32>,
    }
    /// bots are being substituted by a team
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BotSubstitution {
        /// the team that substitutes robots
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
    }
    /// A challenge flag, requested by a team previously, is flagged
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChallengeFlag {
        /// the team that requested the challenge flag
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
    }
    /// An emergency stop, requested by team previously, occurred
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmergencyStop {
        /// the team that substitutes robots
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
    }
    /// a team has too many robots on the field
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TooManyRobots {
        /// the team that has too many robots
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// number of robots allowed at the moment
        #[prost(int32, optional, tag = "2")]
        pub num_robots_allowed: ::core::option::Option<i32>,
        /// number of robots currently on the field
        #[prost(int32, optional, tag = "3")]
        pub num_robots_on_field: ::core::option::Option<i32>,
        /// the location of the ball at the moment when this foul occurred \[m\]
        #[prost(message, optional, tag = "4")]
        pub ball_location: ::core::option::Option<super::Vector2>,
    }
    /// a robot chipped the ball over the field boundary out of the playing surface
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoundaryCrossing {
        /// the team that has too many robots
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the location of the ball \[m\]
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    /// the penalty kick failed (by time or by keeper)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PenaltyKickFailed {
        /// the team that last touched the ball
        #[prost(enumeration = "super::Team", required, tag = "1")]
        pub by_team: i32,
        /// the location of the ball at the moment of this event \[m\]
        #[prost(message, optional, tag = "2")]
        pub location: ::core::option::Option<super::Vector2>,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        UnknownGameEventType = 0,
        /// triggered by autoRef
        BallLeftFieldTouchLine = 6,
        /// triggered by autoRef
        BallLeftFieldGoalLine = 7,
        /// triggered by autoRef
        AimlessKick = 11,
        /// triggered by autoRef
        AttackerTooCloseToDefenseArea = 19,
        /// triggered by autoRef
        DefenderInDefenseArea = 31,
        /// triggered by autoRef
        BoundaryCrossing = 41,
        /// triggered by GC
        KeeperHeldBall = 13,
        /// triggered by autoRef
        BotDribbledBallTooFar = 17,
        /// triggered by human ref
        BotPushedBot = 24,
        /// triggered by human ref
        BotHeldBallDeliberately = 26,
        /// triggered by human ref
        BotTippedOver = 27,
        /// triggered by autoRef
        AttackerTouchedBallInDefenseArea = 15,
        /// triggered by autoRef
        BotKickedBallTooFast = 18,
        /// triggered by autoRef
        BotCrashUnique = 22,
        /// triggered by autoRef
        BotCrashDrawn = 21,
        /// triggered by autoRef
        DefenderTooCloseToKickPoint = 29,
        /// triggered by autoRef
        BotTooFastInStop = 28,
        /// triggered by autoRef
        BotInterferedPlacement = 20,
        /// triggered by autoRef
        PossibleGoal = 39,
        /// triggered by GC
        Goal = 8,
        /// triggered by GC
        InvalidGoal = 42,
        /// triggered by autoRef
        AttackerDoubleTouchedBall = 14,
        /// triggered by autoRef
        PlacementSucceeded = 5,
        /// triggered by GC and autoRef
        PenaltyKickFailed = 43,
        /// triggered by GC
        NoProgressInGame = 2,
        /// triggered by GC
        PlacementFailed = 3,
        /// triggered by GC
        MultipleCards = 32,
        /// triggered by GC
        MultipleFouls = 34,
        /// triggered by GC
        BotSubstitution = 37,
        /// triggered by GC
        TooManyRobots = 38,
        /// triggered by GC
        ChallengeFlag = 44,
        /// triggered by GC
        EmergencyStop = 45,
        /// triggered by human ref
        UnsportingBehaviorMinor = 35,
        /// triggered by human ref
        UnsportingBehaviorMajor = 36,
        Prepared = 1,
        IndirectGoal = 9,
        ChippedGoal = 10,
        KickTimeout = 12,
        AttackerTouchedOpponentInDefenseArea = 16,
        AttackerTouchedOpponentInDefenseAreaSkipped = 40,
        BotCrashUniqueSkipped = 23,
        BotPushedBotSkipped = 25,
        DefenderInDefenseAreaPartially = 30,
        MultiplePlacementFailures = 33,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::UnknownGameEventType => "UNKNOWN_GAME_EVENT_TYPE",
                Type::BallLeftFieldTouchLine => "BALL_LEFT_FIELD_TOUCH_LINE",
                Type::BallLeftFieldGoalLine => "BALL_LEFT_FIELD_GOAL_LINE",
                Type::AimlessKick => "AIMLESS_KICK",
                Type::AttackerTooCloseToDefenseArea => {
                    "ATTACKER_TOO_CLOSE_TO_DEFENSE_AREA"
                }
                Type::DefenderInDefenseArea => "DEFENDER_IN_DEFENSE_AREA",
                Type::BoundaryCrossing => "BOUNDARY_CROSSING",
                Type::KeeperHeldBall => "KEEPER_HELD_BALL",
                Type::BotDribbledBallTooFar => "BOT_DRIBBLED_BALL_TOO_FAR",
                Type::BotPushedBot => "BOT_PUSHED_BOT",
                Type::BotHeldBallDeliberately => "BOT_HELD_BALL_DELIBERATELY",
                Type::BotTippedOver => "BOT_TIPPED_OVER",
                Type::AttackerTouchedBallInDefenseArea => {
                    "ATTACKER_TOUCHED_BALL_IN_DEFENSE_AREA"
                }
                Type::BotKickedBallTooFast => "BOT_KICKED_BALL_TOO_FAST",
                Type::BotCrashUnique => "BOT_CRASH_UNIQUE",
                Type::BotCrashDrawn => "BOT_CRASH_DRAWN",
                Type::DefenderTooCloseToKickPoint => "DEFENDER_TOO_CLOSE_TO_KICK_POINT",
                Type::BotTooFastInStop => "BOT_TOO_FAST_IN_STOP",
                Type::BotInterferedPlacement => "BOT_INTERFERED_PLACEMENT",
                Type::PossibleGoal => "POSSIBLE_GOAL",
                Type::Goal => "GOAL",
                Type::InvalidGoal => "INVALID_GOAL",
                Type::AttackerDoubleTouchedBall => "ATTACKER_DOUBLE_TOUCHED_BALL",
                Type::PlacementSucceeded => "PLACEMENT_SUCCEEDED",
                Type::PenaltyKickFailed => "PENALTY_KICK_FAILED",
                Type::NoProgressInGame => "NO_PROGRESS_IN_GAME",
                Type::PlacementFailed => "PLACEMENT_FAILED",
                Type::MultipleCards => "MULTIPLE_CARDS",
                Type::MultipleFouls => "MULTIPLE_FOULS",
                Type::BotSubstitution => "BOT_SUBSTITUTION",
                Type::TooManyRobots => "TOO_MANY_ROBOTS",
                Type::ChallengeFlag => "CHALLENGE_FLAG",
                Type::EmergencyStop => "EMERGENCY_STOP",
                Type::UnsportingBehaviorMinor => "UNSPORTING_BEHAVIOR_MINOR",
                Type::UnsportingBehaviorMajor => "UNSPORTING_BEHAVIOR_MAJOR",
                Type::Prepared => "PREPARED",
                Type::IndirectGoal => "INDIRECT_GOAL",
                Type::ChippedGoal => "CHIPPED_GOAL",
                Type::KickTimeout => "KICK_TIMEOUT",
                Type::AttackerTouchedOpponentInDefenseArea => {
                    "ATTACKER_TOUCHED_OPPONENT_IN_DEFENSE_AREA"
                }
                Type::AttackerTouchedOpponentInDefenseAreaSkipped => {
                    "ATTACKER_TOUCHED_OPPONENT_IN_DEFENSE_AREA_SKIPPED"
                }
                Type::BotCrashUniqueSkipped => "BOT_CRASH_UNIQUE_SKIPPED",
                Type::BotPushedBotSkipped => "BOT_PUSHED_BOT_SKIPPED",
                Type::DefenderInDefenseAreaPartially => {
                    "DEFENDER_IN_DEFENSE_AREA_PARTIALLY"
                }
                Type::MultiplePlacementFailures => "MULTIPLE_PLACEMENT_FAILURES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_GAME_EVENT_TYPE" => Some(Self::UnknownGameEventType),
                "BALL_LEFT_FIELD_TOUCH_LINE" => Some(Self::BallLeftFieldTouchLine),
                "BALL_LEFT_FIELD_GOAL_LINE" => Some(Self::BallLeftFieldGoalLine),
                "AIMLESS_KICK" => Some(Self::AimlessKick),
                "ATTACKER_TOO_CLOSE_TO_DEFENSE_AREA" => {
                    Some(Self::AttackerTooCloseToDefenseArea)
                }
                "DEFENDER_IN_DEFENSE_AREA" => Some(Self::DefenderInDefenseArea),
                "BOUNDARY_CROSSING" => Some(Self::BoundaryCrossing),
                "KEEPER_HELD_BALL" => Some(Self::KeeperHeldBall),
                "BOT_DRIBBLED_BALL_TOO_FAR" => Some(Self::BotDribbledBallTooFar),
                "BOT_PUSHED_BOT" => Some(Self::BotPushedBot),
                "BOT_HELD_BALL_DELIBERATELY" => Some(Self::BotHeldBallDeliberately),
                "BOT_TIPPED_OVER" => Some(Self::BotTippedOver),
                "ATTACKER_TOUCHED_BALL_IN_DEFENSE_AREA" => {
                    Some(Self::AttackerTouchedBallInDefenseArea)
                }
                "BOT_KICKED_BALL_TOO_FAST" => Some(Self::BotKickedBallTooFast),
                "BOT_CRASH_UNIQUE" => Some(Self::BotCrashUnique),
                "BOT_CRASH_DRAWN" => Some(Self::BotCrashDrawn),
                "DEFENDER_TOO_CLOSE_TO_KICK_POINT" => {
                    Some(Self::DefenderTooCloseToKickPoint)
                }
                "BOT_TOO_FAST_IN_STOP" => Some(Self::BotTooFastInStop),
                "BOT_INTERFERED_PLACEMENT" => Some(Self::BotInterferedPlacement),
                "POSSIBLE_GOAL" => Some(Self::PossibleGoal),
                "GOAL" => Some(Self::Goal),
                "INVALID_GOAL" => Some(Self::InvalidGoal),
                "ATTACKER_DOUBLE_TOUCHED_BALL" => Some(Self::AttackerDoubleTouchedBall),
                "PLACEMENT_SUCCEEDED" => Some(Self::PlacementSucceeded),
                "PENALTY_KICK_FAILED" => Some(Self::PenaltyKickFailed),
                "NO_PROGRESS_IN_GAME" => Some(Self::NoProgressInGame),
                "PLACEMENT_FAILED" => Some(Self::PlacementFailed),
                "MULTIPLE_CARDS" => Some(Self::MultipleCards),
                "MULTIPLE_FOULS" => Some(Self::MultipleFouls),
                "BOT_SUBSTITUTION" => Some(Self::BotSubstitution),
                "TOO_MANY_ROBOTS" => Some(Self::TooManyRobots),
                "CHALLENGE_FLAG" => Some(Self::ChallengeFlag),
                "EMERGENCY_STOP" => Some(Self::EmergencyStop),
                "UNSPORTING_BEHAVIOR_MINOR" => Some(Self::UnsportingBehaviorMinor),
                "UNSPORTING_BEHAVIOR_MAJOR" => Some(Self::UnsportingBehaviorMajor),
                "PREPARED" => Some(Self::Prepared),
                "INDIRECT_GOAL" => Some(Self::IndirectGoal),
                "CHIPPED_GOAL" => Some(Self::ChippedGoal),
                "KICK_TIMEOUT" => Some(Self::KickTimeout),
                "ATTACKER_TOUCHED_OPPONENT_IN_DEFENSE_AREA" => {
                    Some(Self::AttackerTouchedOpponentInDefenseArea)
                }
                "ATTACKER_TOUCHED_OPPONENT_IN_DEFENSE_AREA_SKIPPED" => {
                    Some(Self::AttackerTouchedOpponentInDefenseAreaSkipped)
                }
                "BOT_CRASH_UNIQUE_SKIPPED" => Some(Self::BotCrashUniqueSkipped),
                "BOT_PUSHED_BOT_SKIPPED" => Some(Self::BotPushedBotSkipped),
                "DEFENDER_IN_DEFENSE_AREA_PARTIALLY" => {
                    Some(Self::DefenderInDefenseAreaPartially)
                }
                "MULTIPLE_PLACEMENT_FAILURES" => Some(Self::MultiplePlacementFailures),
                _ => None,
            }
        }
    }
    /// the event that occurred
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "6")]
        BallLeftFieldTouchLine(BallLeftField),
        #[prost(message, tag = "7")]
        BallLeftFieldGoalLine(BallLeftField),
        #[prost(message, tag = "11")]
        AimlessKick(AimlessKick),
        #[prost(message, tag = "19")]
        AttackerTooCloseToDefenseArea(AttackerTooCloseToDefenseArea),
        #[prost(message, tag = "31")]
        DefenderInDefenseArea(DefenderInDefenseArea),
        #[prost(message, tag = "43")]
        BoundaryCrossing(BoundaryCrossing),
        #[prost(message, tag = "13")]
        KeeperHeldBall(KeeperHeldBall),
        #[prost(message, tag = "17")]
        BotDribbledBallTooFar(BotDribbledBallTooFar),
        #[prost(message, tag = "24")]
        BotPushedBot(BotPushedBot),
        #[prost(message, tag = "26")]
        BotHeldBallDeliberately(BotHeldBallDeliberately),
        #[prost(message, tag = "27")]
        BotTippedOver(BotTippedOver),
        #[prost(message, tag = "15")]
        AttackerTouchedBallInDefenseArea(AttackerTouchedBallInDefenseArea),
        #[prost(message, tag = "18")]
        BotKickedBallTooFast(BotKickedBallTooFast),
        #[prost(message, tag = "22")]
        BotCrashUnique(BotCrashUnique),
        #[prost(message, tag = "21")]
        BotCrashDrawn(BotCrashDrawn),
        #[prost(message, tag = "29")]
        DefenderTooCloseToKickPoint(DefenderTooCloseToKickPoint),
        #[prost(message, tag = "28")]
        BotTooFastInStop(BotTooFastInStop),
        #[prost(message, tag = "20")]
        BotInterferedPlacement(BotInterferedPlacement),
        #[prost(message, tag = "39")]
        PossibleGoal(Goal),
        #[prost(message, tag = "8")]
        Goal(Goal),
        #[prost(message, tag = "44")]
        InvalidGoal(Goal),
        #[prost(message, tag = "14")]
        AttackerDoubleTouchedBall(AttackerDoubleTouchedBall),
        #[prost(message, tag = "5")]
        PlacementSucceeded(PlacementSucceeded),
        #[prost(message, tag = "45")]
        PenaltyKickFailed(PenaltyKickFailed),
        #[prost(message, tag = "2")]
        NoProgressInGame(NoProgressInGame),
        #[prost(message, tag = "3")]
        PlacementFailed(PlacementFailed),
        #[prost(message, tag = "32")]
        MultipleCards(MultipleCards),
        #[prost(message, tag = "34")]
        MultipleFouls(MultipleFouls),
        #[prost(message, tag = "37")]
        BotSubstitution(BotSubstitution),
        #[prost(message, tag = "38")]
        TooManyRobots(TooManyRobots),
        #[prost(message, tag = "46")]
        ChallengeFlag(ChallengeFlag),
        #[prost(message, tag = "47")]
        EmergencyStop(EmergencyStop),
        #[prost(message, tag = "35")]
        UnsportingBehaviorMinor(UnsportingBehaviorMinor),
        #[prost(message, tag = "36")]
        UnsportingBehaviorMajor(UnsportingBehaviorMajor),
        /// replaced by ready_to_continue flag
        #[prost(message, tag = "1")]
        Prepared(Prepared),
        /// obsolete
        #[prost(message, tag = "9")]
        IndirectGoal(IndirectGoal),
        /// replaced by the meta-information in the possible_goal event
        #[prost(message, tag = "10")]
        ChippedGoal(ChippedGoal),
        /// obsolete
        #[prost(message, tag = "12")]
        KickTimeout(KickTimeout),
        /// rule removed
        #[prost(message, tag = "16")]
        AttackerTouchedOpponentInDefenseArea(AttackerTouchedOpponentInDefenseArea),
        /// obsolete
        #[prost(message, tag = "42")]
        AttackerTouchedOpponentInDefenseAreaSkipped(
            AttackerTouchedOpponentInDefenseArea,
        ),
        /// obsolete
        #[prost(message, tag = "23")]
        BotCrashUniqueSkipped(BotCrashUnique),
        /// can not be used as long as autoRefs do not judge pushing
        #[prost(message, tag = "25")]
        BotPushedBotSkipped(BotPushedBot),
        /// rule removed
        #[prost(message, tag = "30")]
        DefenderInDefenseAreaPartially(DefenderInDefenseAreaPartially),
        /// the referee msg already indicates this
        #[prost(message, tag = "33")]
        MultiplePlacementFailures(MultiplePlacementFailures),
    }
}
/// Each UDP packet contains one of these messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Referee {
    /// A random UUID of the source that is kept constant at the source while running
    /// If multiple sources are broadcasting to the same communication, this id can be used to identify individual sources
    #[prost(string, optional, tag = "18")]
    pub source_identifier: ::core::option::Option<::prost::alloc::string::String>,
    /// The match type is a meta information about the current match that helps to process the logs after a competition
    #[prost(enumeration = "MatchType", optional, tag = "19", default = "UnknownMatch")]
    pub match_type: ::core::option::Option<i32>,
    /// The UNIX timestamp when the packet was sent, in microseconds.
    /// Divide by 1,000,000 to get a time_t.
    #[prost(uint64, required, tag = "1")]
    pub packet_timestamp: u64,
    #[prost(enumeration = "referee::Stage", required, tag = "2")]
    pub stage: i32,
    /// The number of microseconds left in the stage.
    /// The following stages have this value; the rest do not:
    /// NORMAL_FIRST_HALF
    /// NORMAL_HALF_TIME
    /// NORMAL_SECOND_HALF
    /// EXTRA_TIME_BREAK
    /// EXTRA_FIRST_HALF
    /// EXTRA_HALF_TIME
    /// EXTRA_SECOND_HALF
    /// PENALTY_SHOOTOUT_BREAK
    ///
    /// If the stage runs over its specified time, this value
    /// becomes negative.
    #[prost(sint32, optional, tag = "3")]
    pub stage_time_left: ::core::option::Option<i32>,
    #[prost(enumeration = "referee::Command", required, tag = "4")]
    pub command: i32,
    /// The number of commands issued since startup (mod 2^32).
    #[prost(uint32, required, tag = "5")]
    pub command_counter: u32,
    /// The UNIX timestamp when the command was issued, in microseconds.
    /// This value changes only when a new command is issued, not on each packet.
    #[prost(uint64, required, tag = "6")]
    pub command_timestamp: u64,
    /// Information about the two teams.
    #[prost(message, required, tag = "7")]
    pub yellow: referee::TeamInfo,
    #[prost(message, required, tag = "8")]
    pub blue: referee::TeamInfo,
    #[prost(message, optional, tag = "9")]
    pub designated_position: ::core::option::Option<referee::Point>,
    /// Information about the direction of play.
    /// True, if the blue team will have it's goal on the positive x-axis of the ssl-vision coordinate system.
    /// Obviously, the yellow team will play on the opposite half.
    #[prost(bool, optional, tag = "10")]
    pub blue_team_on_positive_half: ::core::option::Option<bool>,
    /// The command that will be issued after the current stoppage and ball placement to continue the game.
    #[prost(enumeration = "referee::Command", optional, tag = "12")]
    pub next_command: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "16")]
    pub game_events: ::prost::alloc::vec::Vec<GameEvent>,
    #[prost(message, repeated, tag = "17")]
    pub game_event_proposals: ::prost::alloc::vec::Vec<GameEventProposalGroup>,
    /// The time in microseconds that is remaining until the current action times out
    /// The time will not be reset. It can get negative.
    /// An autoRef would raise an appropriate event, if the time gets negative.
    /// Possible actions where this time is relevant:
    ///   * free kicks
    ///   * kickoff, penalty kick, force start
    ///   * ball placement
    #[prost(int32, optional, tag = "15")]
    pub current_action_time_remaining: ::core::option::Option<i32>,
}
/// Nested message and enum types in `Referee`.
pub mod referee {
    /// Information about a single team.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TeamInfo {
        /// The team's name (empty string if operator has not typed anything).
        #[prost(string, required, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The number of goals scored by the team during normal play and overtime.
        #[prost(uint32, required, tag = "2")]
        pub score: u32,
        /// The number of red cards issued to the team since the beginning of the game.
        #[prost(uint32, required, tag = "3")]
        pub red_cards: u32,
        /// The amount of time (in microseconds) left on each yellow card issued to the team.
        /// If no yellow cards are issued, this array has no elements.
        /// Otherwise, times are ordered from smallest to largest.
        #[prost(uint32, repeated, tag = "4")]
        pub yellow_card_times: ::prost::alloc::vec::Vec<u32>,
        /// The total number of yellow cards ever issued to the team.
        #[prost(uint32, required, tag = "5")]
        pub yellow_cards: u32,
        /// The number of timeouts this team can still call.
        /// If in a timeout right now, that timeout is excluded.
        #[prost(uint32, required, tag = "6")]
        pub timeouts: u32,
        /// The number of microseconds of timeout this team can use.
        #[prost(uint32, required, tag = "7")]
        pub timeout_time: u32,
        /// The pattern number of this team's goalkeeper.
        #[prost(uint32, required, tag = "8")]
        pub goalkeeper: u32,
        /// The total number of countable fouls that act towards yellow cards
        #[prost(uint32, optional, tag = "9")]
        pub foul_counter: ::core::option::Option<u32>,
        /// The number of consecutive ball placement failures of this team
        #[prost(uint32, optional, tag = "10")]
        pub ball_placement_failures: ::core::option::Option<u32>,
        /// Indicate if the team is able and allowed to place the ball
        #[prost(bool, optional, tag = "12")]
        pub can_place_ball: ::core::option::Option<bool>,
        /// The maximum number of bots allowed on the field based on division and cards
        #[prost(uint32, optional, tag = "13")]
        pub max_allowed_bots: ::core::option::Option<u32>,
        /// The team has submitted an intent to substitute one or more robots at the next chance
        #[prost(bool, optional, tag = "14")]
        pub bot_substitution_intent: ::core::option::Option<bool>,
        /// Indicate if the team reached the maximum allowed ball placement failures and is thus not allowed to place the ball anymore
        #[prost(bool, optional, tag = "15")]
        pub ball_placement_failures_reached: ::core::option::Option<bool>,
    }
    /// The coordinates of the Designated Position. These are measured in
    /// millimetres and correspond to SSL-Vision coordinates. These fields are
    /// always either both present (in the case of a ball placement command) or
    /// both absent (in the case of any other command).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        #[prost(float, required, tag = "1")]
        pub x: f32,
        #[prost(float, required, tag = "2")]
        pub y: f32,
    }
    /// These are the "coarse" stages of the game.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Stage {
        /// The first half is about to start.
        /// A kickoff is called within this stage.
        /// This stage ends with the NORMAL_START.
        NormalFirstHalfPre = 0,
        /// The first half of the normal game, before half time.
        NormalFirstHalf = 1,
        /// Half time between first and second halves.
        NormalHalfTime = 2,
        /// The second half is about to start.
        /// A kickoff is called within this stage.
        /// This stage ends with the NORMAL_START.
        NormalSecondHalfPre = 3,
        /// The second half of the normal game, after half time.
        NormalSecondHalf = 4,
        /// The break before extra time.
        ExtraTimeBreak = 5,
        /// The first half of extra time is about to start.
        /// A kickoff is called within this stage.
        /// This stage ends with the NORMAL_START.
        ExtraFirstHalfPre = 6,
        /// The first half of extra time.
        ExtraFirstHalf = 7,
        /// Half time between first and second extra halves.
        ExtraHalfTime = 8,
        /// The second half of extra time is about to start.
        /// A kickoff is called within this stage.
        /// This stage ends with the NORMAL_START.
        ExtraSecondHalfPre = 9,
        /// The second half of extra time.
        ExtraSecondHalf = 10,
        /// The break before penalty shootout.
        PenaltyShootoutBreak = 11,
        /// The penalty shootout.
        PenaltyShootout = 12,
        /// The game is over.
        PostGame = 13,
    }
    impl Stage {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Stage::NormalFirstHalfPre => "NORMAL_FIRST_HALF_PRE",
                Stage::NormalFirstHalf => "NORMAL_FIRST_HALF",
                Stage::NormalHalfTime => "NORMAL_HALF_TIME",
                Stage::NormalSecondHalfPre => "NORMAL_SECOND_HALF_PRE",
                Stage::NormalSecondHalf => "NORMAL_SECOND_HALF",
                Stage::ExtraTimeBreak => "EXTRA_TIME_BREAK",
                Stage::ExtraFirstHalfPre => "EXTRA_FIRST_HALF_PRE",
                Stage::ExtraFirstHalf => "EXTRA_FIRST_HALF",
                Stage::ExtraHalfTime => "EXTRA_HALF_TIME",
                Stage::ExtraSecondHalfPre => "EXTRA_SECOND_HALF_PRE",
                Stage::ExtraSecondHalf => "EXTRA_SECOND_HALF",
                Stage::PenaltyShootoutBreak => "PENALTY_SHOOTOUT_BREAK",
                Stage::PenaltyShootout => "PENALTY_SHOOTOUT",
                Stage::PostGame => "POST_GAME",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORMAL_FIRST_HALF_PRE" => Some(Self::NormalFirstHalfPre),
                "NORMAL_FIRST_HALF" => Some(Self::NormalFirstHalf),
                "NORMAL_HALF_TIME" => Some(Self::NormalHalfTime),
                "NORMAL_SECOND_HALF_PRE" => Some(Self::NormalSecondHalfPre),
                "NORMAL_SECOND_HALF" => Some(Self::NormalSecondHalf),
                "EXTRA_TIME_BREAK" => Some(Self::ExtraTimeBreak),
                "EXTRA_FIRST_HALF_PRE" => Some(Self::ExtraFirstHalfPre),
                "EXTRA_FIRST_HALF" => Some(Self::ExtraFirstHalf),
                "EXTRA_HALF_TIME" => Some(Self::ExtraHalfTime),
                "EXTRA_SECOND_HALF_PRE" => Some(Self::ExtraSecondHalfPre),
                "EXTRA_SECOND_HALF" => Some(Self::ExtraSecondHalf),
                "PENALTY_SHOOTOUT_BREAK" => Some(Self::PenaltyShootoutBreak),
                "PENALTY_SHOOTOUT" => Some(Self::PenaltyShootout),
                "POST_GAME" => Some(Self::PostGame),
                _ => None,
            }
        }
    }
    /// These are the "fine" states of play on the field.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Command {
        /// All robots should completely stop moving.
        Halt = 0,
        /// Robots must keep 50 cm from the ball.
        Stop = 1,
        /// A prepared kickoff or penalty may now be taken.
        NormalStart = 2,
        /// The ball is dropped and free for either team.
        ForceStart = 3,
        /// The yellow team may move into kickoff position.
        PrepareKickoffYellow = 4,
        /// The blue team may move into kickoff position.
        PrepareKickoffBlue = 5,
        /// The yellow team may move into penalty position.
        PreparePenaltyYellow = 6,
        /// The blue team may move into penalty position.
        PreparePenaltyBlue = 7,
        /// The yellow team may take a direct free kick.
        DirectFreeYellow = 8,
        /// The blue team may take a direct free kick.
        DirectFreeBlue = 9,
        /// The yellow team may take an indirect free kick.
        IndirectFreeYellow = 10,
        /// The blue team may take an indirect free kick.
        IndirectFreeBlue = 11,
        /// The yellow team is currently in a timeout.
        TimeoutYellow = 12,
        /// The blue team is currently in a timeout.
        TimeoutBlue = 13,
        /// The yellow team just scored a goal.
        /// For information only.
        /// For rules compliance, teams must treat as STOP.
        /// Deprecated: Use the score field from the team infos instead. That way, you can also detect revoked goals.
        GoalYellow = 14,
        /// The blue team just scored a goal. See also GOAL_YELLOW.
        GoalBlue = 15,
        /// Equivalent to STOP, but the yellow team must pick up the ball and
        /// drop it in the Designated Position.
        BallPlacementYellow = 16,
        /// Equivalent to STOP, but the blue team must pick up the ball and drop
        /// it in the Designated Position.
        BallPlacementBlue = 17,
    }
    impl Command {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Command::Halt => "HALT",
                Command::Stop => "STOP",
                Command::NormalStart => "NORMAL_START",
                Command::ForceStart => "FORCE_START",
                Command::PrepareKickoffYellow => "PREPARE_KICKOFF_YELLOW",
                Command::PrepareKickoffBlue => "PREPARE_KICKOFF_BLUE",
                Command::PreparePenaltyYellow => "PREPARE_PENALTY_YELLOW",
                Command::PreparePenaltyBlue => "PREPARE_PENALTY_BLUE",
                Command::DirectFreeYellow => "DIRECT_FREE_YELLOW",
                Command::DirectFreeBlue => "DIRECT_FREE_BLUE",
                Command::IndirectFreeYellow => "INDIRECT_FREE_YELLOW",
                Command::IndirectFreeBlue => "INDIRECT_FREE_BLUE",
                Command::TimeoutYellow => "TIMEOUT_YELLOW",
                Command::TimeoutBlue => "TIMEOUT_BLUE",
                Command::GoalYellow => "GOAL_YELLOW",
                Command::GoalBlue => "GOAL_BLUE",
                Command::BallPlacementYellow => "BALL_PLACEMENT_YELLOW",
                Command::BallPlacementBlue => "BALL_PLACEMENT_BLUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HALT" => Some(Self::Halt),
                "STOP" => Some(Self::Stop),
                "NORMAL_START" => Some(Self::NormalStart),
                "FORCE_START" => Some(Self::ForceStart),
                "PREPARE_KICKOFF_YELLOW" => Some(Self::PrepareKickoffYellow),
                "PREPARE_KICKOFF_BLUE" => Some(Self::PrepareKickoffBlue),
                "PREPARE_PENALTY_YELLOW" => Some(Self::PreparePenaltyYellow),
                "PREPARE_PENALTY_BLUE" => Some(Self::PreparePenaltyBlue),
                "DIRECT_FREE_YELLOW" => Some(Self::DirectFreeYellow),
                "DIRECT_FREE_BLUE" => Some(Self::DirectFreeBlue),
                "INDIRECT_FREE_YELLOW" => Some(Self::IndirectFreeYellow),
                "INDIRECT_FREE_BLUE" => Some(Self::IndirectFreeBlue),
                "TIMEOUT_YELLOW" => Some(Self::TimeoutYellow),
                "TIMEOUT_BLUE" => Some(Self::TimeoutBlue),
                "GOAL_YELLOW" => Some(Self::GoalYellow),
                "GOAL_BLUE" => Some(Self::GoalBlue),
                "BALL_PLACEMENT_YELLOW" => Some(Self::BallPlacementYellow),
                "BALL_PLACEMENT_BLUE" => Some(Self::BallPlacementBlue),
                _ => None,
            }
        }
    }
}
/// List of matching proposals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameEventProposalGroup {
    /// The proposed game event.
    #[prost(message, repeated, tag = "1")]
    pub game_event: ::prost::alloc::vec::Vec<GameEvent>,
    /// Whether the proposal group was accepted
    #[prost(bool, optional, tag = "2")]
    pub accepted: ::core::option::Option<bool>,
}
/// MatchType is a meta information about the current match for easier log processing
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchType {
    /// not set
    UnknownMatch = 0,
    /// match is part of the group phase
    GroupPhase = 1,
    /// match is part of the elimination phase
    EliminationPhase = 2,
    /// a friendly match, not part of a tournament
    Friendly = 3,
}
impl MatchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchType::UnknownMatch => "UNKNOWN_MATCH",
            MatchType::GroupPhase => "GROUP_PHASE",
            MatchType::EliminationPhase => "ELIMINATION_PHASE",
            MatchType::Friendly => "FRIENDLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_MATCH" => Some(Self::UnknownMatch),
            "GROUP_PHASE" => Some(Self::GroupPhase),
            "ELIMINATION_PHASE" => Some(Self::EliminationPhase),
            "FRIENDLY" => Some(Self::Friendly),
            _ => None,
        }
    }
}
