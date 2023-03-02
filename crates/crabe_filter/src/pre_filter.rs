use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::TeamColor;
use crate::data::FilterData;

pub mod vision;

pub trait PreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    );
}
