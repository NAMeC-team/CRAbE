use nalgebra::Point2;
use serde::Serialize;

// TODO : Document
#[derive(Serialize, Clone, Debug)]
pub struct Penalty {
    pub width: f64,
    pub depth: f64,
    pub top_left_position: Point2<f64>,
}

// TODO : Implement some helper methods
