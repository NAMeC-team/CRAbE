use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::filter::Filter;
use chrono::{DateTime, Utc};
use crabe_framework::data::world::{Ball, RobotMap, World};

pub struct VelocityAccelerationFilter;

fn get_duration_millis(t1: DateTime<Utc>, t2: DateTime<Utc>) -> Option<f64> {
    let duration = t2 - t1;
    if duration.num_milliseconds() < 0 {
        return None;
    }
    return Some(duration.num_milliseconds() as f64);
}

fn update_robot_vel_accel<T>(tracked_robots: &mut TrackedRobotMap<T>, robots: &RobotMap<T>) {
    tracked_robots.iter_mut().for_each(|(id, tracked)| {
        if let Some(robot) = robots.get(id) {
            if let Some(millis) = get_duration_millis(robot.timestamp, tracked.data.timestamp) {
                if millis <= 0.0 {
                    return;
                }
                let distance = tracked.data.pose.position - robot.pose.position;
                let angle = tracked.data.pose.orientation - robot.pose.orientation;
                tracked.data.velocity.linear = distance / millis * 1000.0;
                tracked.data.velocity.angular = angle / millis * 1000.0;

                let linear_diff = tracked.data.velocity.linear - robot.velocity.linear;
                tracked.data.acceleration.linear = linear_diff / millis * 1000.0;
                let angular_diff = tracked.data.velocity.angular - robot.velocity.angular;
                tracked.data.acceleration.angular = angular_diff / millis * 1000.0;
            }
        }
    })
}

fn update_ball_vel_accel(tracked: &mut TrackedBall, ball: &Ball) {
    if let Some(millis) = get_duration_millis(ball.timestamp, tracked.data.timestamp) {
        if millis <= 0.0 {
            return;
        }
        let distance = tracked.data.position - ball.position;
        tracked.data.velocity = distance / millis * 1000.0;
        let vel_diff = tracked.data.velocity - ball.velocity;
        tracked.data.acceleration = vel_diff / millis * 1000.0;
    }
}

impl Filter for VelocityAccelerationFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        update_robot_vel_accel(&mut filter_data.allies, &world.allies_bot);
        update_robot_vel_accel(&mut filter_data.enemies, &world.enemies_bot);
        if let Some(ball) = world.ball.as_ref() {
            if let Some(ball_tracked) = filter_data.ball.as_mut() {
                update_ball_vel_accel(ball_tracked, ball);
            }
        }
    }
}
