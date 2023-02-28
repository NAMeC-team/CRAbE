use crate::data::geometry::Geometry;
use nalgebra::{Point2, Point3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub position: Point3<f32>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    NEUTRAL,
    BLUE,
    YELLOW,
}

pub struct Team {
    color: TeamColor,
    name: String,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            color: TeamColor::NEUTRAL,
            name: "UNKNOWN".to_string(),
        }
    }
}

#[derive(Default)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub blue_positive_half: bool,
}

#[derive(Default)]
pub struct World {
    pub state: GameState,
    pub geometry: Geometry,
    pub allies_bot: HashMap<u8, Robot<AllyInfo>>,
    pub enemies_bot: HashMap<u8, Robot<EnemyInfo>>,
    pub ball: Option<Ball>,
}
