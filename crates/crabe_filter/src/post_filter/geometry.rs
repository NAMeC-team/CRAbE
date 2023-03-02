use std::path::Iter;
use crabe_framework::data::geometry::{Circle, Geometry};
use crabe_framework::data::geometry::goal::Goal;
use crabe_framework::data::geometry::penalty::Penalty;
use crabe_framework::data::world::{RobotMap, World};
use crate::data::{FilterData, TrackedRobot, TrackedRobotMap};
use crate::post_filter::PostFilter;

struct GeometryFilter;

impl PostFilter for GeometryFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let cam_geometry = &filter_data.geometry;
        let geometry = Geometry {
            field_width: cam_geometry.field_width,
            field_length: cam_geometry.field_length,
            ally_goal: Goal { width: 0.0, depth: 0.0 },
            opponent_goal: Goal { width: 0.0, depth: 0.0 },
            ally_penalty: Penalty { width: 0.0, depth: 0.0 },
            opponent_penalty: Penalty { width: 0.0, depth: 0.0 },
            center: Circle { center: Default::default(), radius: 0.0 },
        };

        world.geometry = geometry;
    }
}