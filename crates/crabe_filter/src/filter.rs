pub mod inactive;
pub mod passthrough;
pub mod velocity_acceleration;
pub mod field_side;

use crate::data::FilterData;
use crabe_framework::data::world::World;

pub trait Filter {
    fn step(&mut self, filter_data: &mut FilterData, world: &World);
}
