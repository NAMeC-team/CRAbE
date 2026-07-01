
use std::thread::current;

use crabe_framework::data::{geometry::Field, referee::TeamInfo, world::{Robot, World}};
use log::info;
use nalgebra::{Point2, Vector2};

use crate::{action::{go_to::GoTo}, exception::Exception, utils::KEEPER_ID};



/// Sizes for the substitutes zones
const SUBSTITUTE_ZONE_WIDTH: f64 = 0.3;
const SUBSTITUTE_ZONE_LENGTH: f64 = 2.;
const SUBSTITUTE_ZONE_DISTANCE_BETWEEN_ROBOTS: f64 = 0.1;

/// Place of the bottom substitute zone relative to the bottom center of the field
const RELATIVE_BOTTOM_SUBSTITUTE_ZONE: Vector2<f64> = Vector2::new(- (SUBSTITUTE_ZONE_LENGTH / 2.), - SUBSTITUTE_ZONE_WIDTH);

/// Place of the top substitute zone relative to the top center of the field
const RELATIVE_TOP_SUBSTITUTE_ZONE: Vector2<f64> = Vector2::new(- (SUBSTITUTE_ZONE_LENGTH / 2.), 0.);

pub struct CardException {
    top_substitute_zone: Point2<f64>,
    bottom_substitute_zone: Point2<f64>,
    slot_size: f64,
    slots_per_zone: u8,
}

impl CardException{
    pub fn new() -> Self {
        let slot_size = 0.09 + SUBSTITUTE_ZONE_DISTANCE_BETWEEN_ROBOTS * 2.;
        Self {
            top_substitute_zone: Point2::new(0., 6. / 2.) + RELATIVE_TOP_SUBSTITUTE_ZONE,
            bottom_substitute_zone: Point2::new(0., -(6. / 2.)) + RELATIVE_BOTTOM_SUBSTITUTE_ZONE,
            slot_size: slot_size,
            slots_per_zone: (SUBSTITUTE_ZONE_LENGTH / slot_size).floor() as u8,
        }
    }

    fn robot_at_slot(&self, slot: &Point2<f64>, world: &World) -> Option<u8>{
        for (id, robot) in &world.allies_bot {
            if robot.distance(slot) < self.slot_size {
                return Some(*id)
            }
        }

        None
    }

    fn slot_from_id(&self, id: u8) -> Option<Point2<f64>>{
        match id /self.slots_per_zone {
            0 => Some(self.bottom_substitute_zone + Vector2::new(self.slot_size * (id as f64) , SUBSTITUTE_ZONE_WIDTH  / 2.)),
            1 => Some(self.top_substitute_zone + Vector2::new(self.slot_size * ((id - self.slots_per_zone) as f64) , SUBSTITUTE_ZONE_WIDTH  / 2.)),
            _ => None,
        }
    }

    fn compute_free_substitute_slot(&self, world: &World, current_robot: u8) -> Option<Point2<f64>> {
        for slot_id in 0..self.slots_per_zone {
            let Some(slot) = self.slot_from_id(slot_id) else {
                info!("Incorrect slot found when computing free slot");
                continue;
            };

            if let Some(id) = self.robot_at_slot(&slot, world) {
                if current_robot != id {
                    continue;
                }
            }

            return Some(slot);
        }

        None
    }

    pub fn go_to_shadow_realm(&mut self, action_wrapper: &mut crate::action::ActionWrapper, world: &World, id: u8) {
        let Some(slot) = self.compute_free_substitute_slot(world, id) else {
            info!("No empty slot found !!");
            return;
        };

        let goto = GoTo::new(slot, 0., false, None, false);
        action_wrapper.clear(id);
        action_wrapper.push(id, goto);
    }
}

impl Exception for CardException {
    fn step(
        &mut self,
        world: &crabe_framework::data::world::World,
        tools_data: &mut crabe_framework::data::tool::ToolData,
        action_wrapper: &mut crate::action::ActionWrapper,
        robots_handled: &mut Vec<u8>,

    ) {
        let Some(ally_info) = &world.data.ally.info else {
            return;
        };

        let mut substituted: Vec<u8> = vec![];

        let nb_substitutes = ally_info.yellow_cards + ally_info.red_cards;
        for sub in 0..nb_substitutes {
            let robot_id: u8 = match robots_handled.get(sub as usize) {
                Some(id) => *id,
                None => {
                    let mut res = None;
                    for robot in 0..7 {
                        if !(substituted.contains(&robot) || robot == KEEPER_ID) { res = Some(robot); break; }
                    }

                    let Some(id) = res else {
                        // no place left ???
                        return;
                    };

                    id
                },
            };
            self.go_to_shadow_realm(action_wrapper, world, robot_id);

            substituted.push(robot_id);
        }

        *robots_handled = substituted;
    }
}
