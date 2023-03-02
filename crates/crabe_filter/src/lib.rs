mod data;
mod filters;
mod pre_filter;
mod constant;

use crate::data::FilterData;

use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{TeamColor, World};
use crate::pre_filter::PreFilter;

#[derive(Args)]
pub struct FilterConfig {}

pub trait Filter {
    fn step(&mut self, filter_data: &mut FilterData, world: &mut World);
}

pub struct FilterPipeline {
    pub pre_filters: Vec<Box<dyn PreFilter>>,
    pub filters: Vec<Box<dyn Filter>>,
    pub filter_data: FilterData,
    pub team_color: TeamColor,
}

impl FilterPipeline {
    pub fn with_config(_config: FilterConfig, common_config: &CommonConfig) -> Self {
        Self {
            pre_filters: vec![],
            filters: vec![],
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
            .for_each(|f| f.step(
                &inbound_data,
                &world.team_color,
                &mut self.filter_data
            ));

        self.filters
            .iter_mut()
            .for_each(|f| f.step(&mut self.filter_data, world));
    }

    fn close(&mut self) {}
}
