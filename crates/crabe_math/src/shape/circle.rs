use nalgebra::Point2;
use serde::Serialize;
use uom::si::f32::Length;

/// Represents a circle in 2D space defined by its center point and radius.
#[derive(Serialize, Clone, Debug)]
pub struct Circle {
    /// The center point of the circle.
    pub center: Point2<Length>,
    /// The radius of the circle.
    pub radius: Length,
}
