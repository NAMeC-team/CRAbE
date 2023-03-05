use nalgebra::Point2;
use uom::si::f32::Length;

/// A line segment in 2D space, defined by two points.
#[derive(Debug)]
pub struct Line {
    /// The first point of the line segment.
    pub p1: Point2<Length>,
    /// The second point of the line segment.
    pub p2: Point2<Length>,
}
