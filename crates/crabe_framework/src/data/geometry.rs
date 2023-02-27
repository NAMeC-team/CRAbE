use crate::data::geometry::goal::Goal;
use crate::data::geometry::penalty::Penalty;

use nalgebra::Point2;

pub mod goal;
pub mod penalty;

// TODO : Document
// TODO : Move this on another files.
#[derive(Debug)]
pub struct Circle {
    pub center: Point2<f32>,
    pub radius: f32,
}

#[derive(Debug)]
pub struct Geometry {
    pub field_width: f32,
    pub field_length: f32,
    pub ally_goal: Goal,
    pub opponent_goal: Goal,
    pub ally_penalty: Penalty,
    pub opponent_penalty: Penalty,
    pub center: Circle,
}