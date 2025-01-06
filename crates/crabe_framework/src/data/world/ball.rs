use chrono::{DateTime, Utc};
use nalgebra::{distance, Point2, Point3, Vector3};
use serde::Serialize;

use super::{AllyInfo, EnemyInfo, RobotMap, TeamColor, World};

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

/// Returns closest robot to ball in a RobotMap
/// Returns an error if we couldn't compare distances of at least two robots
/// to the ball. The boolean specifies whether the robot map is empty or not
fn closest_to_ball<'a, T>(ball: &Ball, robots: &'a RobotMap<T>) -> Result<(&'a u8, &'a T, f64), bool> {
   let closest = robots.iter()
       .map(|(id, rob)| {
           let ball_pos = ball.position_2d();
           (id, &rob.robot_info, distance(&rob.pose.position, &ball_pos))
       })
       .min_by(|some, other| some.2.total_cmp(&other.2));

    match closest {
        Some((id, info, dist)) => Ok((id, info, dist)),
        None => Err(robots.is_empty())
    }
}

impl Ball {
    /// Returns the position of the ball as a 2D point (x and y-coordinate), with respect to the center of the field.
    pub fn position_2d(&self) -> Point2<f64> {
        Point2::new(self.position.x, self.position.y)
    }

    pub fn closest_ally_robot<'a>(&self, world: &'a World) -> Option<(&'a u8, &'a AllyInfo, f64)> {
        let mut res = None;
        if let Some(ball) = &world.ball {
            if let Ok((id, info, dist)) = closest_to_ball(ball, &world.allies_bot) {
                res = Some((id, info, dist))
            }
        }

        res
    }

    pub fn closest_enemy_robot<'a>(&self, world: &'a World) -> Option<(&'a u8, &'a EnemyInfo, f64)> {
        let mut res = None;
        if let Some(ball) = &world.ball {
            if let Ok((id, info, dist)) = closest_to_ball(ball, &world.enemies_bot) {
                res = Some((id, info, dist))
            }
        }

        res
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
