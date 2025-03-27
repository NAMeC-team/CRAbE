pub mod ball;
pub mod geometry;
pub mod robot;
pub mod game_controller;
pub mod field_mask;
pub mod robot_has_ball;

use crate::data::FilterData;
use crabe_framework::data::world::World;

pub trait PostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World);
}
