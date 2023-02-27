use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct FilterConfig {}

struct CamBall {}
struct CamRobot {}
struct CamField {}

pub struct FilterData {}

pub trait Filter {}

pub struct FilterPipeline {
    pub filters: Vec<Box<dyn Filter>>,
    pub filter_data: FilterData,
}

impl FilterPipeline {
    pub fn with_config_boxed(_config: FilterConfig) -> Box<Self> {
        Box::new(Self {
            filters: vec![],
            filter_data: FilterData {},
        })
    }
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, _data: InboundData) -> Option<World> {
        None
    }

    fn close(&mut self) {}
}
