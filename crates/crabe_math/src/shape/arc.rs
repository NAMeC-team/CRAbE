use nalgebra::Point2;

/// An arc in 2D space defined by a center, a radius, and two angles.
///
/// Note that the `center` and `radius` fields should have the same units of
/// measurement, and the `start_angle` and `end_angle` fields should also have
/// the same units of measurement.
#[derive(Debug)]
pub struct Arc {
    /// The center point of the arc.
    pub center: Point2<f64>,
    /// The radius of the arc.
    pub radius: f64,
    /// The starting angle of the arc .
    pub start: f64,
    /// The ending angle of the arc.
    pub end: f64,
}

impl Arc {
    /// Create a new arc with the given center, radius, and angles (start and end).
    pub fn new(center: Point2<f64>, radius: f64, start: f64, end: f64) -> Self {
        Self {
            center,
            radius,
            start,
            end,
        }
    }
}