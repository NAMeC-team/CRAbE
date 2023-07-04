use crabe_framework::data::world::{Robot, World};
use crate::data::{FilterData, TrackedRobotMap};
use crate::FieldSide;
use crate::filter::Filter;
use crate::post_filter::PostFilter;

pub struct FieldSideFilter {
    field_side: FieldSide,
}

impl FieldSideFilter {
    pub fn new(field_side: FieldSide) -> FieldSideFilter {
        FieldSideFilter { field_side }
    }

    fn filter_by_side<T>(tracked_robots: &mut TrackedRobotMap<T>, field_side: &FieldSide) {
        println!("{:?}", field_side);
        tracked_robots.retain(|_id, robot| {
            match field_side {
                FieldSide::Positive => robot.data.pose.position.x.is_sign_positive(),
                FieldSide::Negative => robot.data.pose.position.x.is_sign_negative()
            }
        });
    }
}

impl Filter for FieldSideFilter {
    fn step(&mut self, filter_data: &mut FilterData, _world: &World) {
        Self::filter_by_side(&mut filter_data.allies, &self.field_side);
        Self::filter_by_side(&mut filter_data.enemies, &self.field_side);
    }
}