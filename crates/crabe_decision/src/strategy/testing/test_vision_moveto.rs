use nalgebra::{distance, Point2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, RobotMap, World};
use crate::action::{Actions, ActionWrapper};
use crate::action::move_to::MoveTo;
use crate::action::order_raw::RawOrder;
use crate::strategy::Strategy;

const DIST_TARGET_REACHED: f64 = 0.25;

#[derive(Debug)]
enum TestVisionMoveToStatus {
    Placement,
    MovingForward,
    MovingBackwards,
}

pub struct TestVisionMoveTo {
    ids: Vec<u8>,
    status: TestVisionMoveToStatus,
    positive_half: bool,
}

impl TestVisionMoveTo {
    pub fn new(ids: Vec<u8>, positive_half: bool) -> Self {
        Self {
            ids,
            status: TestVisionMoveToStatus::Placement,
            positive_half,
        }
    }
}

impl Strategy for TestVisionMoveTo {
    fn step(&mut self, world: &World, _: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        // WARNING : Not clearing the action_wrapper
        // action_wrapper.clear();
        let sign = if self.positive_half { 1. } else { -1. };
        let mut finished = world.allies_bot.len() > 0;
        match self.status {
            TestVisionMoveToStatus::Placement => {
                world.allies_bot.iter()
                    .filter(|(ally_id, _)| self.ids.contains(ally_id))
                    .for_each(|(ally_id, ally_info)| {
                        let target = Point2::new(*ally_id as f64 * sign, 0.);
                        action_wrapper.push(*ally_id, MoveTo::new(
                            target,
                            0., 0., None, false, false),
                        );
                        finished = finished && distance(&ally_info.pose.position, &target) <= DIST_TARGET_REACHED;
                    });

                if finished {
                    self.status = TestVisionMoveToStatus::MovingForward;
                }
            }

            TestVisionMoveToStatus::MovingForward => {
                let mut change_status = true;
                world.allies_bot.iter()
                    .filter(|(ally_id, _)| self.ids.contains(ally_id))
                    .for_each(|(ally_id, ally_info)| {
                        let target_forward = Point2::new(*ally_id as f64 * sign, 1.);
                        action_wrapper.push(*ally_id, MoveTo::new(
                            target_forward,
                            0., 0., None, false, false),
                        );
                        change_status = distance(&target_forward, &ally_info.pose.position) <= DIST_TARGET_REACHED
                });
                self.status = if change_status { TestVisionMoveToStatus::MovingBackwards } else { TestVisionMoveToStatus::MovingForward };
            }

            TestVisionMoveToStatus::MovingBackwards => {
                let mut change_status = true;
                world.allies_bot.iter()
                    .filter(|(ally_id, _)| self.ids.contains(ally_id))
                    .for_each(|(ally_id, ally_info)| {
                    let target_backwards = Point2::new(*ally_id as f64 * sign, -1.);
                    action_wrapper.push(*ally_id, MoveTo::new(
                        target_backwards,
                        0., 0., None, false, false),
                    );
                    change_status = distance(&target_backwards, &ally_info.pose.position) <= DIST_TARGET_REACHED
                });
                self.status = if change_status { TestVisionMoveToStatus::MovingForward } else { TestVisionMoveToStatus::MovingBackwards };
            }
        }
        dbg!(&self.status);
        false
    }

    fn name(&self) -> &'static str {
        "TestVisionMoveTo"
    }
}