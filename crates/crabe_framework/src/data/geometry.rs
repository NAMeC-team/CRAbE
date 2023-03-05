use crate::data::geometry::goal::Goal;
use crate::data::geometry::penalty::Penalty;

use nalgebra::Point2;
use serde::Serialize;
use uom::num_traits::Zero;
use uom::si::f32::Length;
use uom::si::length::meter;

pub mod goal;
pub mod penalty;

// TODO : Document
// TODO : Move this on another files.
#[derive(Serialize, Clone, Debug)]
pub struct Circle {
    pub center: Point2<Length>,
    pub radius: Length,
}

#[derive(Serialize, Clone, Debug)]
pub struct Field {
    pub width: Length,
    pub length: Length,
}

#[derive(Serialize, Clone, Debug)]
pub struct Geometry {
    pub boundary_width: Length,
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
                width: Length::new::<meter>(9.0),
                length: Length::new::<meter>(6.0),
            },
            ally_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            enemy_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            ally_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
            },
            enemy_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
            },
            center: Circle {
                center: Default::default(),
                radius: Length::zero(),
            },
        }
    }
}
