use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::filter::Filter;
use chrono::{DateTime, Utc};
use crabe_framework::data::world::{Ball, RobotMap, World};
use nalgebra::{Point2, Point3};

pub struct VelocityAccelerationFilter;

fn get_duration_secs(t1: DateTime<Utc>, t2: DateTime<Utc>) -> Option<f64> {
    let time = t2 - t1;
    if let Ok(duration) = time.to_std() {
        return Some(duration.as_secs_f64());
    }

    return None;
}

fn to_side(p: Point2<f64>, positive_half: bool) -> Point2<f64>{
    if positive_half{
        return p
    }
    return Point2::new(
        -p.x,-p.y
    )
}
fn to_side3(p: Point3<f64>, positive_half: bool) -> Point3<f64>{
    if positive_half{
        return p
    }
    return Point3::new(
        -p.x,-p.y,p.z
    )
}

fn update_robot_vel_accel<T>(tracked_robots: &mut TrackedRobotMap<T>, robots: &RobotMap<T>, positive_half: bool) {
    tracked_robots.iter_mut().for_each(|(id, tracked)| {
        if let Some(robot) = robots.get(id) {
            if let Some(secs) =
                get_duration_secs(tracked.data.timestamp.clone(), robot.timestamp.clone())
            {
                let distance = tracked.data.pose.position - to_side(robot.pose.position, positive_half);
                let angle = tracked.data.pose.orientation - robot.pose.orientation;
                tracked.data.velocity.linear = distance / 0.16;
                tracked.data.velocity.angular = angle / 0.16;

                let linear_diff = tracked.data.velocity.linear - robot.velocity.linear;
                tracked.data.acceleration.linear = linear_diff / 0.16;
                let angular_diff = tracked.data.velocity.angular - robot.velocity.angular;
                tracked.data.acceleration.angular = angular_diff / 0.16;
            }
        }
    })
}

fn update_ball_vel_accel(tracked: &mut TrackedBall, ball: &Ball, positive_half: bool) {//TODO : secs is equal to 0 everytime
    if let Some(secs) = get_duration_secs(tracked.data.timestamp.clone(), ball.timestamp.clone()) {//
        if secs <= 0. {//I use 0.16 for an aproximation of 60 frame per seconds
            let distance = tracked.data.position - to_side3(ball.position, positive_half);
            tracked.data.velocity = distance / 0.16;
            let vel_diff = tracked.data.velocity - ball.velocity;
            tracked.data.acceleration = vel_diff / 0.16;
        }else{
            let distance = tracked.data.position - to_side3(ball.position, positive_half);
            tracked.data.velocity = distance / secs;
            let vel_diff = tracked.data.velocity - ball.velocity;
            tracked.data.acceleration = vel_diff / secs;
        }
    }
}

impl Filter for VelocityAccelerationFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        update_robot_vel_accel(&mut filter_data.allies, &world.allies_bot, world.data.positive_half != world.team_color);
        if let Some(ball) = world.ball.as_ref() {
            if let Some(tracked_ball) = &mut filter_data.ball {
                update_ball_vel_accel(tracked_ball, ball, world.data.positive_half != world.team_color);
            }
        }
    }
}
