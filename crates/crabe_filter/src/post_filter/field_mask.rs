use crabe_framework::data::world::{Ball, Robot, RobotMap, World};
use crate::data::{FilterData, TrackedBall, TrackedRobotMap};
use crate::FieldMask;
use crate::post_filter::PostFilter;

pub struct FieldMaskFilter {
    field_side: FieldMask,
}

impl FieldMaskFilter {
    pub fn new(field_side: FieldMask) -> FieldMaskFilter {
        FieldMaskFilter { field_side }
    }

    fn filter_robots_by_side<T>(tracked_robots: &mut RobotMap<T>, field_side: &FieldMask) {
        tracked_robots.retain(|_id, robot| {
            match field_side {
                FieldMask::Positive => robot.pose.position.x.is_sign_positive(),
                FieldMask::Negative => robot.pose.position.x.is_sign_negative()
            }
        });
    }

    fn filter_ball_by_side(traked_ball: &mut Option<Ball>, field_side: &FieldMask) {
        if let Some(ball) = traked_ball {
            match field_side {
                FieldMask::Positive => {
                    if ball.position.x.is_sign_negative() {
                        *traked_ball = None;
                    }
                }
                FieldMask::Negative => {
                    if ball.position.x.is_sign_positive() {
                        *traked_ball = None;
                    }
                }
            }
        }
    }
}

impl PostFilter for FieldMaskFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        Self::filter_robots_by_side(&mut world.allies_bot, &self.field_side);
        Self::filter_robots_by_side(&mut world.enemies_bot, &self.field_side);
        Self::filter_ball_by_side(&mut world.ball, &self.field_side);
    }
}