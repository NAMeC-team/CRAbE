use crate::constant::MAX_ROBOTS;
use crate::data::geometry::Geometry;
use nalgebra::{Point2, Point3};
use serde::{Deserialize, Serialize};

pub struct AllyInfo;
pub struct EnemyInfo;

pub struct Robot<T> {
    pub id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub has_ball: bool,
    pub robot_info: T,
}

pub struct Ball {
    pub pos: Point3<f32>,
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
