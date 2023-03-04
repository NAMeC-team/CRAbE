use crate::data::geometry::Geometry;
use nalgebra::{Point2, Point3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uom::si::f32::{Angle, Length};

#[derive(Serialize, Clone, Default, Debug)]
pub struct AllyInfo;
#[derive(Serialize, Clone, Default, Debug)]
pub struct EnemyInfo;

#[derive(Serialize, Default, Debug)]
pub struct Robot<T> {
    pub id: u32,
    pub position: Point2<Length>,
    pub orientation: Angle,
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
            robot_info: self.robot_info.clone(),
        }
    }
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct Ball {
    pub position: Point3<Length>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub enum TeamColor {
    #[default]
    Neutral,
    Blue,
    Yellow,
}

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Default, Debug)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub positive_half: TeamColor,
}

pub type RobotMap<T> = HashMap<u32, Robot<T>>;

#[derive(Serialize, Clone, Default, Debug)]
pub struct World {
    pub state: GameState,
    pub geometry: Geometry,
    pub allies_bot: RobotMap<AllyInfo>,
    pub enemies_bot: RobotMap<EnemyInfo>,
    pub ball: Option<Ball>,
    pub team_color: TeamColor,
}
