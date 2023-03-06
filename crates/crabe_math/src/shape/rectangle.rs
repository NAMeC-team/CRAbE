use nalgebra::Point2;
use uom::si::f32::Length;

pub struct Rectangle {
    pub width: Length,
    pub depth: Length,
    pub left_position: Point2<Length>,
}
