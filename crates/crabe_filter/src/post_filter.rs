pub mod geometry;
pub mod robot;
pub mod ball;

use crabe_framework::data::world::World;
use crate::data::FilterData;

pub trait PostFilter {
    fn step(
        &mut self,
        filter_data: &FilterData,
        world: &mut World
    );
}