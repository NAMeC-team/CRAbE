use crate::data::FrameInfo;
use crabe_math::shape::arc::Arc;
use crabe_math::shape::line::Line;
use nalgebra::{Point2, Point3};
use uom::si::f32::{Angle, Length};

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

#[derive(Debug)]
pub struct CamFieldLine {
    pub thickness: f32,
    pub line: Line,
}

#[derive(Debug)]
pub struct CamFieldArc {
    pub thickness: f32,
    pub arc: Arc,
}

#[derive(Debug, Default)]
pub struct CamGeometry {
    pub field_length: Length,
    pub field_width: Length,
    pub goal_width: Length,
    pub goal_depth: Length,
    pub boundary_width: Length,
    pub field_lines: Vec<CamFieldLine>,
    pub field_arcs: Vec<CamFieldArc>, // pub last_update: Instant,
}
