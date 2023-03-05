use crate::data::camera::CamGeometry;
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::geometry::goal::Goal;
use crabe_framework::data::geometry::penalty::Penalty;
use crabe_framework::data::geometry::{Field, Geometry};
use crabe_framework::data::world::World;
use crabe_math::shape::circle::Circle;
use uom::num_traits::Zero;
use uom::si::f32::Length;
use uom::si::length::meter;

pub struct GeometryFilter;

fn geometry_to_center(cam_geometry: &CamGeometry) -> Circle {
    cam_geometry
        .field_arcs
        .get("CenterCircle")
        .map(|circle| Circle {
            center: circle.arc.center,
            radius: circle.arc.radius,
        })
        .unwrap_or_else(|| Circle {
            center: Default::default(),
            radius: cam_geometry
                .center_circle_radius
                .unwrap_or(Length::new::<meter>(0.5)),
        })
}

impl PostFilter for GeometryFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let cam_geometry = &filter_data.geometry;

        let geometry = Geometry {
            boundary_width: cam_geometry.boundary_width,
            field: Field {
                width: cam_geometry.field_width,
                length: cam_geometry.field_length,
            },
            ally_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            enemy_goal: Goal {
                width: 0.0,
                depth: 0.0,
            },
            ally_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
            },
            enemy_penalty: Penalty {
                width: 0.0,
                depth: 0.0,
            },
            center: geometry_to_center(cam_geometry),
        };
        // dbg!(&cam_geometry);
        // dbg!(&geometry);

        world.geometry = geometry;
    }
}
