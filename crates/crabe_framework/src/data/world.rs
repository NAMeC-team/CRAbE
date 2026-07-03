mod robot;

use std::time::Instant;
use chrono::{DateTime, Utc};
pub use self::robot::{
    AllyInfo, EnemyInfo, Pose, Robot, RobotAcceleration, RobotMap, RobotVelocity,
};

use serde_with::serde_as;

mod ball;
pub use self::ball::{Ball, BallTouchInfo};

mod team;
pub use self::team::{Team, TeamColor};

pub mod game_data;
pub mod game_state;
pub mod stage_info;

pub use self::game_data::GameData;

use crate::config::CommonConfig;
use crate::data::geometry::Geometry;

use serde::Serialize;
pub use crate::data::world::robot::KEEPER_ID;

/// The `World` struct represents the state of the world in the SSL game,
/// containing information about the game state, the field geometry, the robots and the ball.
#[serde_as]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct World {
    /// The current state of the game.
    pub data: GameData,
    /// The geometry of the field, including its dimensions and the positions of goals and other areas.
    pub geometry: Geometry,
    /// A map of all the ally robots in the game, identified by their unique ID.
    #[serde_as(as = "Vec<(_, _)>")]
    pub allies_bot: RobotMap<AllyInfo>,
    /// A map of all the enemy robots in the game, identified by their unique ID.
    #[serde_as(as = "Vec<(_, _)>")]
    pub enemies_bot: RobotMap<EnemyInfo>,
    /// The current position and state of the ball, if it is visible.
    pub ball: Option<Ball>,
    /// The team color of our team.
    pub team_color: TeamColor,
    #[serde(skip)]
    pub timestamp: Instant,
    pub keeper_id: u8,
}

impl World {
    /// Creates a new `World` instance based on a given `CommonConfig` instance.
    pub fn with_config(config: &CommonConfig) -> Self {
        let team_color = if config.yellow {
            TeamColor::Yellow
        } else {
            TeamColor::Blue
        };
        Self {
            data: GameData::new(team_color, config),
            geometry: Default::default(),
            allies_bot: Default::default(),
            enemies_bot: Default::default(),
            ball: None,
            team_color,
            timestamp: Instant::now(), // TODO: update,
            keeper_id: KEEPER_ID
        }
    }
}
