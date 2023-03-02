use crate::data::geometry::Geometry;
use nalgebra::{Point2, Point3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct AllyInfo;
#[derive(Clone, Default)]
pub struct EnemyInfo;

#[derive(Default)]
pub struct Robot<T> {
    pub id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub has_ball: bool,
    pub robot_info: T,
}

impl<T: Clone> Clone for Robot<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            position: self.position,
            orientation: self.orientation,
            has_ball: self.has_ball,
            robot_info: self.robot_info.clone()
        }
    }
}

#[derive(Default)]
pub struct Ball {
    pub position: Point3<f32>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub enum TeamColor {
    #[default]
    Neutral,
    Blue,
    Yellow,
}

pub struct Team {
    color: TeamColor,
    name: String,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            color: TeamColor::Neutral,
            name: "UNKNOWN".to_string(),
        }
    }
}

#[derive(Default)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub positive_half: TeamColor,
}

pub type RobotMap<T> = HashMap<u8, Robot<T>>;

#[derive(Default)]
pub struct World {
    pub state: GameState,
    pub geometry: Geometry,
    pub allies_bot: RobotMap<AllyInfo>,
    pub enemies_bot: RobotMap<EnemyInfo>,
    pub ball: Option<Ball>,
    pub team_color: TeamColor
}
