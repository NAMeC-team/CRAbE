use ringbuffer::RingBufferExt;
use crabe_framework::data::world::{Robot, RobotMap, World};
use crate::data::{FilterData, TrackedRobotMap};
use crate::filter::Filter;

pub struct VelocityFilter;

fn update_velocity<T>(tracked_robots: &mut TrackedRobotMap<T>, robots: &RobotMap<T>) {
    tracked_robots.iter_mut().for_each(|(id, tracked)| {
        if let Some(robot) = robots.get(id) {
            let time = tracked.data.timestamp.clone() - robot.timestamp.clone();

            let distance = (tracked.data.position - robot.position);
            let angle = tracked.data.orientation - robot.orientation;
            if let Ok(duration) = time.to_std() {
                let seconds = duration.as_secs_f64();
                tracked.data.linear_velocity = distance / seconds;
                tracked.data.angular_velocity = angle / seconds;
            }

        }
    })
}

impl Filter for VelocityFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        update_velocity(&mut filter_data.allies, &world.allies_bot);
        update_velocity(&mut filter_data.enemies, &world.enemies_bot);
    }
}