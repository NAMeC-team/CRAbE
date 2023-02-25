use serde::{Deserialize, Serialize};

pub const MAX_ROBOTS: usize = 16;

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub struct Field {
    pub width: f32,
    pub length: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    pub penalty_depth: f32,
    pub penalty_width: f32,
    pub center_radius: f32,
}

pub struct AllyInfo;
pub struct EnemyInfo;

pub struct Robot<T> {
    robot_info: T,

}



pub struct World {
    pub field: Option<Field>,
    pub allies: [Option<Robot<AllyInfo>>; MAX_ROBOTS],
    pub enemies: [Option<Robot<EnemyInfo>>; MAX_ROBOTS],
}