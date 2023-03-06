use crate::data::FrameInfo;
use crabe_math::shape::arc::Arc;
use crabe_math::shape::line::Line;
use nalgebra::{Point2, Point3};
use std::collections::HashMap;

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
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    pub boundary_width: f32,
    pub field_lines: HashMap<String, CamFieldLine>,
    pub field_arcs: HashMap<String, CamFieldArc>, // pub last_update: Instant,
    pub penalty_area_depth: Option<f32>,
    pub penalty_area_width: Option<f32>,
    pub center_circle_radius: Option<f32>,
    pub line_thickness: Option<f32>,
    pub goal_center_to_penalty_mark: Option<f32>,
    pub goal_height: Option<f32>,
    pub ball_radius: Option<f32>,
    pub max_robot_radius: Option<f32>,
}
