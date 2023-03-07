use nalgebra::Point2;

/// An arc in 2D space defined by a center, a radius in meters, and two angles in radians.
#[derive(Debug)]
pub struct Arc {
    /// The center point of the arc in meters.
    pub center: Point2<f64>,
    /// The radius of the arc in meters.
    pub radius: f64,
    /// The starting angle of the arc in radians.
    pub start_angle: f64,
    /// The ending angle of the arc in radians.
    pub end_angle: f64,
}