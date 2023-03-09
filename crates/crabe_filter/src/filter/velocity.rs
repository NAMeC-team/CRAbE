use chrono::{DateTime, Utc};
use ringbuffer::RingBufferExt;
use crabe_framework::data::world::{Ball, Robot, RobotMap, World};
use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::filter::Filter;

pub struct VelocityFilter;

fn get_duration_secs(t1: DateTime<Utc>, t2: DateTime<Utc>) -> Option<f64> {
    let time = t2 - t1;
    if let Ok(duration) = time.to_std() {
        return Some(duration.as_secs_f64());
    }

    return None;
}


fn update_robot_velocity<T>(tracked_robots: &mut TrackedRobotMap<T>, robots: &RobotMap<T>) {
    tracked_robots.iter_mut().for_each(|(id, tracked)| {
        if let Some(robot) = robots.get(id) {
            if let Some(secs) = get_duration_secs(tracked.data.timestamp.clone(), robot.timestamp.clone()) {
                let distance = tracked.data.pose.position - robot.pose.position;
                let angle = tracked.data.pose.orientation - robot.pose.orientation;
                tracked.data.velocity.linear = distance / secs;
                tracked.data.velocity.angular = angle / secs;
            }
        }
    })
}

fn update_ball_velocity(tracked: &mut TrackedBall, ball: &Ball) {
    if let Some(secs) = get_duration_secs(tracked.data.timestamp.clone(), ball.timestamp.clone()) {
        let distance = tracked.data.position - ball.position;
        tracked.data.velocity = distance / secs;
    }
}

impl Filter for VelocityFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        update_robot_velocity(&mut filter_data.allies, &world.allies_bot);
        if let Some(ball) = world.ball.as_ref() {
            update_ball_velocity(&mut filter_data.ball, ball);
        }
    }
}