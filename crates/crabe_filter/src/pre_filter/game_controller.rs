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
        // TODO: this allocates a ton
        // dbg!(&inbound_data.gc_packet);
        filter_data
            .referee
            .extend(inbound_data.gc_packet.iter().map(|p| p.clone()));
    }
}
