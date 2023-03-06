use nalgebra::Point2;
use serde::Serialize;

// TODO : Document
#[derive(Serialize, Clone, Debug)]
pub struct Penalty {
    pub width: f32,
    pub depth: f32,
    pub top_left_position: Point2<f32>,
}

// TODO : Implement some helper methods
