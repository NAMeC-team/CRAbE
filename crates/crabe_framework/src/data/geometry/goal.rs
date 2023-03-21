use nalgebra::Point2;
use serde::Serialize;

/// Represents a goal on a soccer field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The width of the goal, in meters.
    pub width: f64,
    /// The depth of the goal, in meters.
    pub depth: f64,
    /// The top-left corner of the goal, measured from the origin of the field,
    /// in meters.
    pub top_left_position: Point2<f64>,
}

impl Goal {}
