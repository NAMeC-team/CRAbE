use crabe_framework::component::FilterComponent;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::World;

struct FilterPipeline;

impl FilterComponent for FilterPipeline {
    fn step(&mut self, data: InboundData) -> World {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }
}
