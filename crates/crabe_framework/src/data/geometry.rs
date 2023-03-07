use crate::data::geometry::goal::Goal;
use crate::data::geometry::penalty::Penalty;

use crabe_math::shape::circle::Circle;
use nalgebra::Point2;
use serde::Serialize;

pub mod goal;
pub mod penalty;

#[derive(Serialize, Clone, Debug)]
pub struct Field {
    pub width: f64,
    pub length: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Geometry {
    pub boundary_width: f64,
    pub field: Field,
    pub ally_goal: Goal,
    pub enemy_goal: Goal,
    pub ally_penalty: Penalty,
    pub enemy_penalty: Penalty,
    pub center: Circle,
}

// TODO: Add default dimension of field Division B
impl Default for Geometry {
    fn default() -> Self {
        Self {
            boundary_width: Default::default(),
            field: Field {
                width: 9.0,
                length: 6.0,
            },
            ally_goal: Goal {
                width: 0.0,
                depth: 0.0,
                top_left_position: Default::default(),
            },
            enemy_goal: Goal {
                width: 0.0,
                depth: 0.0,
                top_left_position: Default::default(),
            },
            ally_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
                top_left_position: Default::default(),
            },
            enemy_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
                top_left_position: Default::default(),
            },
            center: Circle {
                center: Default::default(),
                radius: 0.0,
            },
        }
    }
}
