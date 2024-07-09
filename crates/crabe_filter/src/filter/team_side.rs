use std::{collections::HashMap, time::Instant};

use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::filter::Filter;
use chrono::{DateTime, Utc};
use crabe_framework::data::world::{Ball, RobotMap, World};

pub struct TeamSideFilter{
    last_update_robots: HashMap<u8, DateTime<Utc>>,
    last_update_ball: Instant,
}

impl Default for TeamSideFilter {
    fn default() -> Self {
        TeamSideFilter {
            last_update_robots: Default::default(),
            last_update_ball: Instant::now(),
        }
    }
    
}

fn get_duration_millis(t1: DateTime<Utc>, t2: DateTime<Utc>) -> Option<f64> {
    let duration = t2 - t1;
    if duration.num_milliseconds() < 0 {
        return None;
    }
    return Some(duration.num_milliseconds() as f64);
}

fn change_robots_side<T>(tracked_robots: &mut TrackedRobotMap<T>, robots: &RobotMap<T>) {
    tracked_robots.iter_mut().for_each(|(id, tracked)| {
        if let Some(robot) = robots.get(id) {
            if let Some(millis) = get_duration_millis(robot.timestamp, tracked.data.timestamp) {
                if millis <= 0.0 {
                    return;
                }
                println!("Changing side of robot {}", id);
                tracked.data.pose.position.x = -robot.pose.position.x;
                tracked.data.pose.orientation = ( std::f64::consts::PI - robot.pose.orientation ).rem_euclid(2.0 * std::f64::consts::PI);
            }
        }
    })
}

fn change_ball_side(tracked: &mut TrackedBall, ball: &Ball) {
    if let Some(millis) = get_duration_millis(ball.timestamp, tracked.data.timestamp) {
        if millis <= 0.0 {
            return;
        }
        tracked.data.position.x = -ball.position.x;
    }
}

impl Filter for TeamSideFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        let positive_change = world.team_color == world.data.positive_half;
        if !positive_change {return;}

        change_robots_side(&mut filter_data.allies, &world.allies_bot);
        change_robots_side(&mut filter_data.enemies, &world.enemies_bot);
        if let Some(ball) = world.ball.as_ref() {
            change_ball_side(&mut filter_data.ball, ball);
        }
    }
}
