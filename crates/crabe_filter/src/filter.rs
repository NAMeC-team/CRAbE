pub mod passthrough;
mod velocity;

use crate::data::FilterData;
use crabe_framework::data::world::World;

pub trait Filter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World);
}
