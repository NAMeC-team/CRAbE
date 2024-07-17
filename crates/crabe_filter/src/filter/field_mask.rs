use crabe_framework::data::world::{Robot, World};
use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::FieldMask;
use crate::filter::Filter;

pub struct FieldMaskFilter {
    field_side: FieldMask,
}

impl FieldMaskFilter {
    pub fn new(field_side: FieldMask) -> FieldMaskFilter {
        FieldMaskFilter { field_side }
    }

    fn filter_robots_by_side<T>(tracked_robots: &mut TrackedRobotMap<T>, field_side: &FieldMask) {
        tracked_robots.retain(|_id, robot| {
            match field_side {
                FieldMask::Positive => robot.data.pose.position.x.is_sign_positive(),
                FieldMask::Negative => robot.data.pose.position.x.is_sign_negative()
            }
        });
    }
}

impl Filter for FieldMaskFilter {
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        Self::filter_robots_by_side(&mut filter_data.allies, &self.field_side);
        Self::filter_robots_by_side(&mut filter_data.enemies, &self.field_side);
    }
}