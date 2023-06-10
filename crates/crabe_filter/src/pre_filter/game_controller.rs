use crate::data::FilterData;
use crate::pre_filter::PreFilter;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;

pub struct GameControllerPreFilter;

impl GameControllerPreFilter {}

impl PreFilter for GameControllerPreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        _team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        /// TODO: The referee message needs to be inside our own framework.
        filter_data
            .referee
            .extend(inbound_data.gc_packet.iter().cloned());
    }
}
