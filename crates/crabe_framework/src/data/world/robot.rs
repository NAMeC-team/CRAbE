use chrono::{DateTime, Utc};
use nalgebra::{Point2, Vector2};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Default, Debug)]
pub struct AllyInfo;
#[derive(Serialize, Clone, Default, Debug)]
pub struct EnemyInfo;

#[derive(Serialize, Default, Debug, Clone)]
pub struct RobotVelocity {
    pub linear: Vector2<f64>,
    pub angular: f64,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct RobotAcceleration {
    pub linear: Vector2<f64>,
    pub angular: f64,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Pose {
    pub orientation: f64,
    pub position: Point2<f64>,
}

impl Pose {
    pub fn new(position: Point2<f64>, orientation: f64) -> Pose {
        Pose {
            orientation,
            position,
        }
    }
}

pub type RobotMap<T> = HashMap<u32, Robot<T>>;

#[derive(Serialize, Default, Debug)]
pub struct Robot<T> {
    pub id: u32,
    pub has_ball: bool,
    pub robot_info: T,
    pub pose: Pose,
    pub velocity: RobotVelocity,
    pub acceleration: RobotAcceleration,
    pub timestamp: DateTime<Utc>,
}

impl<T: Clone> Clone for Robot<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            has_ball: self.has_ball,
            robot_info: self.robot_info.clone(),
            pose: self.pose.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            timestamp: self.timestamp,
        }
    }
}
