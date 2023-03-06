use crate::data::camera::CamGeometry;
use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::geometry::goal::Goal;
use crabe_framework::data::geometry::penalty::Penalty;
use crabe_framework::data::geometry::{Field, Geometry};
use crabe_framework::data::world::World;
use crabe_math::shape::circle::Circle;
use nalgebra::Point2;
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

fn geometry_to_penalty(cam_geometry: &CamGeometry, positive: bool) -> Penalty {
    let factor = if positive { 1.0 } else { -1.0 };
    cam_geometry
        .field_lines
        .get("LeftFieldLeftPenaltyStretch")
        .map(|line| {
            dbg!(line.line.p1.x - line.line.p2.x);
            dbg!(line.line.p1.x);
            dbg!(line.line.p2.x);
            Penalty {
                width: 2.0 * line.line.p1.y.abs(),
                depth: (line.line.p1.x - line.line.p2.x).abs(),
                left_position: Point2::new(
                    factor * cam_geometry.field_length,
                    factor * cam_geometry.goal_width,
                ),
            }
        })
        .unwrap_or_else(|| Penalty {
            width: cam_geometry
                .penalty_area_width
                .unwrap_or(Length::new::<meter>(2.0)),
            depth: cam_geometry
                .penalty_area_depth
                .unwrap_or(Length::new::<meter>(1.0)),
            left_position: Point2::new(
                factor * cam_geometry.field_length,
                factor * cam_geometry.goal_width,
            ),
        })
}

fn geometry_to_goal(cam_geometry: &CamGeometry, positive: bool) -> Goal {
    let factor = if positive { 1.0 } else { -1.0 };
    Goal {
        width: cam_geometry.goal_width,
        depth: cam_geometry.goal_depth,
        left_position: Point2::new(
            factor * cam_geometry.field_length,
            factor * cam_geometry.goal_width,
        ),
    }
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
            ally_goal: geometry_to_goal(cam_geometry, false),
            enemy_goal: geometry_to_goal(cam_geometry, true),
            ally_penalty: geometry_to_penalty(cam_geometry, false),
            enemy_penalty: geometry_to_penalty(cam_geometry, true),
            center: geometry_to_center(cam_geometry),
        };

        world.geometry = geometry;
    }
}
