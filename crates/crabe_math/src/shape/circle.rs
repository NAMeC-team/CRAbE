use nalgebra::Point2;
use serde::Serialize;

/// Represents a circle in 2D space defined by its center point and radius.
#[derive(Serialize, Clone, Debug)]
pub struct Circle {
    /// The center point of the circle in meters.
    pub center: Point2<f64>,
    /// The radius of the circle in meters.
    pub radius: f64,
}