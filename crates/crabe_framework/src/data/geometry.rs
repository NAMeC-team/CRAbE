use crate::data::geometry::goal::Goal;
use crate::data::geometry::penalty::Penalty;

use nalgebra::Point2;
use uom::num_traits::Zero;
use uom::si::f32::Length;
use uom::si::length::meter;
use serde::Serialize;

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
pub struct Geometry {
    pub field_width: Length,
    pub field_length: Length,
    pub ally_goal: Goal,
    pub opponent_goal: Goal,
    pub ally_penalty: Penalty,
    pub opponent_penalty: Penalty,
    pub center: Circle,
}

// TODO: Add default dimension of field Division B
impl Default for Geometry {
    fn default() -> Self {
        Self {
            field_width: Length::new::<meter>(9.0),
            field_length: Length::new::<meter>(6.0),
            ally_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            opponent_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            ally_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
            },
            opponent_penalty: Penalty {
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
