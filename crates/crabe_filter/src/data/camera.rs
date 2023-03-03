use nalgebra::{Point2, Point3};
use uom::si::f32::{Angle, Length};
use uom::typenum::Le;
use crate::data::FrameInfo;

#[derive(Debug)]
pub struct CamBall {
    pub position: Point3<Length>,
    pub frame_info: FrameInfo,
    pub confidence: f32,
}

#[derive(Debug)]
pub struct CamRobot {
    pub id: u32,
    pub frame_info: FrameInfo,
    pub position: Point2<Length>,
    pub orientation: Angle,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct CamGeometry {
    pub field_length: Length,
    pub field_width: Length,
    pub goal_width: Length,
    pub goal_depth: Length,
    // pub last_update: Instant,
}