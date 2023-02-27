use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::constant::MAX_ROBOTS;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::World;

#[derive(Args)]
pub struct FilterConfig {}

struct CamBall {}

struct CamRobot {}

struct CamField {}

pub struct FilterData {
    allies: [Option<CamRobot>; MAX_ROBOTS], // TODO: Use HASHMAP ?
    opponents: [Option<CamRobot>; MAX_ROBOTS],
}

pub trait Filter {}

pub struct FilterPipeline {
    pub filters: Vec<Box<dyn Filter>>,
    pub filter_data: FilterData,
    pub yellow: bool,
}

impl FilterPipeline {
    pub fn with_config_boxed(_config: FilterConfig, common_config: &CommonConfig) -> Box<Self> {
        Box::new(Self {
            filters: vec![],
            filter_data: FilterData {
                allies: Default::default(),
                opponents: Default::default(),
            },
            yellow: common_config.yellow,
        })
    }
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData) -> Option<World> {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let time = detection.t_capture;

                detection.robots_blue.drain(..).for_each(|r| {});
                detection.robots_yellow.drain(..).for_each(|r| {});
                detection.balls.drain(..).for_each(|b| {});
            }

            if let Some(mut geometry) = packet.geometry {
                dbg!(geometry.field);
            }
        });

        None
    }

    fn close(&mut self) {}
}
