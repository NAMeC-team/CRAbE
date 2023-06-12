use crate::data::FilterData;
use crate::data::referee::Referee;
use crate::pre_filter::PreFilter;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet::{self, referee};

pub struct GameControllerPreFilter;

impl GameControllerPreFilter {}

fn convert_referee_protobuf(packet: &game_controller_packet::Referee) -> Referee {
    let referee = Referee {
        source_identifier: todo!(),
        match_type: todo!(),
        packet_timestamp: todo!(),
        stage: todo!(),
        stage_time_left: todo!(),
        command: todo!(),
        command_counter: todo!(),
        command_timestamp: todo!(),
        ally: todo!(),
        enemy: todo!(),
        designated_position: todo!(),
        positive_half: todo!(),
        next_command: todo!(),
        game_events: todo!(),
        game_event_proposals: todo!(),
        current_action_time_remaining: todo!(),
    };
    referee
}

impl PreFilter for GameControllerPreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        _team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        inbound_data.gc_packet.iter().for_each(|gc_packet|{
            convert_referee_protobuf(gc_packet);
        });
        // TODO: The referee message needs to be inside our own framework.

        /*filter_data
        .referee
        .extend(inbound_data.gc_packet.iter().cloned());*/
    }
}
