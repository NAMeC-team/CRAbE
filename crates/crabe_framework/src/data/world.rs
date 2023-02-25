use crate::data::geometry::Geometry;
use serde::{Deserialize, Serialize};

pub const MAX_ROBOTS: usize = 16;

pub struct AllyInfo;
pub struct EnemyInfo;

pub struct Robot<T> {
    id: u32,
    // pos : 2D
    orientation: f32,
    has_ball: bool,
    robot_info: T,
}

pub struct Ball {
    // pos : 3D
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    BLUE,
    YELLOW,
}

pub struct Team {
    color: TeamColor,
    name: String,
}

pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub blue_positive_half: bool,
}

pub struct World {
    pub state: GameState,
    pub geometry: Geometry, // TODO : Add default value
    pub allies_bot: [Option<Robot<AllyInfo>>; MAX_ROBOTS],
    pub enemies_bot: [Option<Robot<EnemyInfo>>; MAX_ROBOTS],
    pub ball: Ball,
}
