use nalgebra::{Point2, Point3};
use crate::data::FrameInfo;

#[derive(Debug)]
pub struct CamBall {
    pub position: Point3<f32>,
    pub frame_info: FrameInfo,
    pub confidence: f32,
}

#[derive(Debug)]
pub struct CamRobot {
    pub id: u32,
    pub frame_info: FrameInfo,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct CamGeometry {
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    // pub last_update: Instant,
}