use crabe_math::shape::Line;
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
    /// The bottom-left corner of the goal, measured from the origin of the field,
    /// in meters.
    pub bottom_left_position: Point2<f64>,
    /// The bottom-right corner of the goal, measured from the origin of the field,
    /// in meters.
    pub bottom_right_position: Point2<f64>,
    /// The top-right corner of the goal, measured from the origin of the field,
    /// in meters.
    pub top_right_position: Point2<f64>,
    /// The front line of the goal, measured from the origin of the field,
    /// in meters.
    pub front_line: Line
}

impl Goal {}
