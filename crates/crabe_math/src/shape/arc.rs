use nalgebra::Point2;
use uom::si::f32::Angle;
use uom::si::f32::Length;

/// An arc in 2D space defined by a center, a radius, and two angles.
#[derive(Debug)]
pub struct Arc {
    /// The center point of the arc.
    pub center: Point2<Length>,
    /// The radius of the arc.
    pub radius: Length,
    /// The starting angle of the arc in radians.
    pub start_angle: Angle,
    /// The ending angle of the arc in radians.
    pub end_angle: Angle,
}
