use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::World;

#[derive(Args)]
struct FilterConfig {}

struct CamBall {}
struct CamRobot {}
struct CamField {}

struct FilterData {}

trait Filter {}

struct FilterPipeline {
    filters: Vec<Box<dyn Filter>>,
    filter_data: FilterData,
}

impl FilterPipeline {
    fn with_config_boxed(_config: FilterConfig) -> Box<Self> {
        Box::new(Self {
            filters: vec![],
            filter_data: FilterData {},
        })
    }
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, data: InboundData) -> Option<World> {

        None
    }

    fn close(&mut self) {}
}
