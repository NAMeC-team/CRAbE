use chrono::{DateTime, Utc};
use nalgebra::{Point3, Vector3};
use serde::Serialize;

/// The `Ball` struct represents the ball in the SSL game.
#[derive(Serialize, Default, Clone, Debug)]
pub struct Ball {
    /// The position of the ball in 3D space.
    pub position: Point3<f64>,
    /// The timestamp of when the data was captured.
    pub timestamp: DateTime<Utc>,
    /// The velocity of the ball in 3D space.
    pub velocity: Vector3<f64>,
    /// The acceleration of the ball in 3D space.
    pub acceleration: Vector3<f64>,
}
