use chrono::{DateTime, Utc};
use nalgebra::{Point2, Point3, Vector3};
use serde::Serialize;

use super::TeamColor;

/// The `Ball` struct represents the ball in the SSL game.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ball {
    /// The position of the ball in 3D space in meters, with respect to the center of the field.
    pub position: Point3<f64>,
    /// The timestamp of when the data was captured.
    pub timestamp: DateTime<Utc>,
    /// The velocity of the ball in 3D space in meters per second.
    pub velocity: Vector3<f64>,
    /// The acceleration of the ball in 3D space in meters per second squared.
    pub acceleration: Vector3<f64>,
    /// The team color of the team that currently possesses the ball.
    pub possession: Option<TeamColor>,
    /// The last touch of the ball by a robot.
    pub last_touch: Option<BallTouchInfo>,
}


impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Point3::new(10000.,10000., 10000.),
            timestamp: Default::default(),
            velocity: Default::default(),
            acceleration:  Default::default(),
            possession:  Default::default(),
            last_touch:  Default::default(),
        }
    }
}

impl Ball {
    /// Returns the position of the ball as a 2D point (x and y-coordinate), with respect to the center of the field.
    pub fn position_2d(&self) -> Point2<f64> {
        Point2::new(self.position.x, self.position.y)
    }
}

/// The `BallTouchInfo` struct represents the last touch of the ball by a robot.
/// It contains the id of the robot that touched the ball, the timestamp of the touch and the position of the ball at the time of the touch.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BallTouchInfo {
    /// The id of the robot that touched the ball.
    pub robot_id: u8,
    /// The team color of the robot that touched the ball.
    pub team_color: TeamColor,
    /// The timestamp of the touch.
    pub timestamp: DateTime<Utc>,
    /// The position of the ball at the time of the touch.
    pub position: Point3<f64>,
}
