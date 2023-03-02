pub mod passthrough;

use crabe_framework::data::world::World;
use crate::data::FilterData;

pub trait Filter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World);
}
