use std::process::exit;

use crabe_framework::data::geometry::Field;
use crabe_framework::data::world::{Ball, RobotMap, World};
use log::{error, info};
use nalgebra::Point2;
use crate::{FieldKind, FieldMask};
use crate::data::FilterData;
use crate::post_filter::PostFilter;


pub struct FieldMaskFilter {
    field_kind: FieldKind,
    field_mask: FieldMask,
}

impl FieldMaskFilter {
    pub fn new(field_mask: FieldMask, field_kind: FieldKind) -> FieldMaskFilter {
        match field_kind {
            FieldKind::Half => if field_mask >= 2 { error!("Wrong field_mask provided: expected a value between 0 and 1, got {}", field_mask); },
            FieldKind::Quarter => if field_mask >= 4 { error!("Wrong field_mask provided: expected a value between 0 and 3, got {}", field_mask); },
        };
        FieldMaskFilter { field_kind, field_mask }
    }

    //those two functions could probably be made generic, but a quarter terrain is plenty, no need for that
    fn position_in_quarter_field(&self, position: Point2<f64>) -> bool {

        let x_res =
            if self.field_mask == 0 || self.field_mask == 2 {
                position.x.is_sign_negative()
            } else {
                position.x.is_sign_positive()
            };

        let y_res =
            if self.field_mask == 0 || self.field_mask == 1 {
                position.y.is_sign_positive()
            } else {
                position.y.is_sign_negative()
            };

        x_res && y_res
    }

    fn position_in_half_field(&self, position: Point2<f64>) -> bool {
        if self.field_mask == 0 {
            position.x.is_sign_negative()
        } else {
            position.x.is_sign_positive()
        }
    }

    fn position_in_field(&self, position: Point2<f64>) -> bool {
        match self.field_kind {
            FieldKind::Half => self.position_in_half_field(position),
            FieldKind::Quarter => self.position_in_quarter_field(position),
        }
    }

    fn filter_robots_by_side<T>(&self, tracked_robots: &mut RobotMap<T>) {
        tracked_robots.retain(|_id, robot| self.position_in_field(robot.pose.position));
    }

    fn filter_ball_by_side(&self, tracked_ball: &mut Option<Ball>) {
        let Some(ball) = tracked_ball else {
            return;
        };

        // Why the fuck is the ball in 3 dimensions ?
        if !self.position_in_field(ball.position.xy()) { *tracked_ball = None; };
    }
}

impl PostFilter for FieldMaskFilter {
    fn step(&mut self, _filter_data: &FilterData, world: &mut World) {
        self.filter_robots_by_side(&mut world.allies_bot);
        self.filter_robots_by_side(&mut world.enemies_bot);
        self.filter_ball_by_side(&mut world.ball);
    }
}
