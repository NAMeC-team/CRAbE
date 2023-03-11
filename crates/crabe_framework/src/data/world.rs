mod robot;
pub use self::robot::{
    AllyInfo, EnemyInfo, Pose, Robot, RobotAcceleration, RobotMap, RobotVelocity,
};

mod ball;
pub use self::ball::Ball;

mod team;
pub use self::team::{Team, TeamColor};

mod game_state;
pub use self::game_state::GameState;

use crate::config::CommonConfig;
use crate::data::geometry::Geometry;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct World {
    pub state: GameState,
    pub geometry: Geometry,
    pub allies_bot: RobotMap<AllyInfo>,
    pub enemies_bot: RobotMap<EnemyInfo>,
    pub ball: Option<Ball>,
    pub team_color: TeamColor,
}

impl World {
    pub fn with_config(config: &CommonConfig) -> Self {
        let team_color = if config.yellow {
            TeamColor::Yellow
        } else {
            TeamColor::Blue
        };
        Self {
            state: GameState::new(team_color),
            geometry: Default::default(),
            allies_bot: Default::default(),
            enemies_bot: Default::default(),
            ball: None,
            team_color,
        }
    }
}
