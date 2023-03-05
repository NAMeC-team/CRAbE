use nalgebra::Point2;
use uom::si::f32::Angle;
use uom::si::f32::Length;

#[derive(Debug)]
pub struct Arc {
    pub center: Point2<Length>,
    pub radius: Length,
    pub start_angle: Angle,
    pub end_angle: Angle,
}
