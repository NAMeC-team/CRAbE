use crate::config::CommonConfig;
use crate::data::geometry::Geometry;
use clap::builder::Str;
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    Blue,
    Yellow,
}

impl TeamColor {
    pub fn opposite(&self) -> Self {
        match self {
            TeamColor::Blue => TeamColor::Yellow,
            TeamColor::Yellow => TeamColor::Blue,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Team {
    color: TeamColor,
    name: Option<String>,
}

impl Team {
    pub fn with_color(color: TeamColor) -> Self {
        Self { color, name: None }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub positive_half: TeamColor,
}

impl GameState {
    pub fn new(team_color: TeamColor) -> Self {
        Self {
            ally: Team::with_color(team_color),
            enemy: Team::with_color(team_color.opposite()),
            positive_half: team_color.opposite(),
        }
    }
}

pub type RobotMap<T> = HashMap<u32, Robot<T>>;

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
