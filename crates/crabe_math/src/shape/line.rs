use nalgebra::Point2;

/// A line segment in 2D space, defined by two points with positions in meters.
#[derive(Debug)]
pub struct Line {
    /// The first point of the line segment with position in meters.
    pub p1: Point2<f32>,
    /// The second point of the line segment with position in meters.
    pub p2: Point2<f32>,
}