use nalgebra::Point2;
use uom::si::f32::Length;

#[derive(Debug)]
pub struct Line {
    pub p1: Point2<Length>,
    pub p2: Point2<Length>,
}
