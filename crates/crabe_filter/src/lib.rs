mod constant;
mod data;
mod filter;
mod post_filter;
mod pre_filter;

use crate::data::FilterData;

use crate::filter::passthrough::PassthroughFilter;
use crate::filter::Filter;
use crate::post_filter::ball::BallFilter;
use crate::post_filter::geometry::GeometryFilter;
use crate::post_filter::robot::RobotFilter;
use crate::post_filter::PostFilter;
use crate::pre_filter::vision::VisionFilter;
use crate::pre_filter::PreFilter;
use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{TeamColor, World};

#[derive(Args)]
pub struct FilterConfig {}

pub struct FilterPipeline {
    pub pre_filters: Vec<Box<dyn PreFilter>>,
    pub filters: Vec<Box<dyn Filter>>,
    pub post_filters: Vec<Box<dyn PostFilter>>,
    pub filter_data: FilterData,
    pub team_color: TeamColor,
}

impl FilterPipeline {
    pub fn with_config(_config: FilterConfig, common_config: &CommonConfig) -> Self {
        Self {
            pre_filters: vec![Box::new(VisionFilter::new())],
            filters: vec![Box::new(PassthroughFilter)],
            post_filters: vec![
                Box::new(RobotFilter),
                Box::new(GeometryFilter),
                Box::new(BallFilter),
            ],
            filter_data: FilterData {
                allies: Default::default(),
                enemies: Default::default(),
                ball: Default::default(),
                geometry: Default::default(),
            },
            team_color: if common_config.yellow {
                TeamColor::Yellow
            } else {
                TeamColor::Blue
            },
        }
    }
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, inbound_data: InboundData, world: &mut World) {
        self.pre_filters
            .iter_mut()
            .for_each(|f| f.step(&inbound_data, &world.team_color, &mut self.filter_data));

        self.filters
            .iter_mut()
            .for_each(|f| f.step(&mut self.filter_data, world));

        self.post_filters
            .iter_mut()
            .for_each(|f| f.step(&self.filter_data, world));
    }

    fn close(&mut self) {}
}
