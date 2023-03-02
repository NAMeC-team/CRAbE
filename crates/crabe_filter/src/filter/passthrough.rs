use chrono::format::Item;
use ringbuffer::RingBufferRead;
use crabe_framework::data::world::{Robot, World};
use crate::data::{FilterData, TrackedRobot};
use crate::filter::Filter;

fn robot_passthrough<'a, T: 'a + Default>(
    robots: impl Iterator<Item=(&'a u32, &'a mut TrackedRobot<T>)>
) {
    robots.for_each(|(id, r)| {
        let last_packet = r.packets.drain().last();
        if let Some(packet) = last_packet {
            r.data = Robot {
                id: packet.id,
                position: packet.position,
                orientation: packet.orientation,
                has_ball: false,
                robot_info: T::default(),
            }
        }
    } )
}

pub struct PassthroughFilter;

impl Filter for PassthroughFilter {
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        robot_passthrough(filter_data.allies.iter_mut());
        robot_passthrough(filter_data.enemies.iter_mut());
    }
}