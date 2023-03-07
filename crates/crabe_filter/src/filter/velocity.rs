use ringbuffer::RingBufferExt;
use crabe_framework::data::world::{Robot, RobotMap, World};
use crate::data::{FilterData, TrackedRobotMap};
use crate::filter::Filter;

pub struct VelocityFilter;

fn update_velocity<T>(cam_robots: &mut TrackedRobotMap<T>) {
    cam_robots.iter_mut().for_each(|(id, tracked)| {
        let mut packets = tracked.packets.iter().rev();
        let current = packets.next();
        let previous = packets.next();
        if let Some(current) = current {
            if let Some(previous) = previous {
                let time = current.frame_info.t_capture - previous.frame_info.t_capture;
                let distance = (current.position - previous.position).norm();
                tracked.data.velocity = distance / (time.num_seconds() as f64);
            }
        }
    })

}

impl Filter for VelocityFilter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World) {
        update_velocity(&mut filter_data.allies);
        update_velocity(&mut filter_data.enemies);
    }
}