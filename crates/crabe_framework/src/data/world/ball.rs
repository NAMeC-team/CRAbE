use chrono::{DateTime, Utc};
use nalgebra::{Point3, Vector3};
use serde::Serialize;

#[derive(Serialize, Default, Clone, Debug)]
pub struct Ball {
    pub position: Point3<f64>,
    pub timestamp: DateTime<Utc>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}
