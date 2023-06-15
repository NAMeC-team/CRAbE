use crate::data::referee::{Referee, RefereeCommand, Stage, TeamInfo};
use crate::data::FilterData;
use crate::pre_filter::common::create_date_time;
use crate::pre_filter::PreFilter;
use chrono::Duration;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet;
use nalgebra::Point2;

use crabe_protocol::protobuf::game_controller_packet::referee::{
    Command as ProtocolCommand, Command, Point as ProtocolPoint, Stage as ProtocolStage,
};
use crabe_protocol::protobuf::game_controller_packet::Referee as ProtocolReferee;

pub struct GameControllerPreFilter;

impl GameControllerPreFilter {}

fn to_team_infos(infos: game_controller_packet::referee::TeamInfo) -> TeamInfo {
    let _infos = TeamInfo {
        name: infos.name.into(),
        score: infos.score,
        red_cards: infos.red_cards,
        yellow_card_times: infos.yellow_card_times.into(),
        yellow_cards: infos.yellow_cards,
        timeouts: infos.timeouts,
        timeout_time: infos.timeout_time,
        goalkeeper: infos.goalkeeper,
        foul_counter: infos.foul_counter,
        ball_placement_failures: infos.ball_placement_failures,
        can_place_ball: infos.can_place_ball,
        max_allowed_bots: infos.max_allowed_bots,
        bot_substitution_intent: infos.bot_substitution_intent,
        ball_placement_failures_reached: infos.ball_placement_failures_reached,
        bot_substitution_allowed: infos.bot_substitution_allowed,
    };
    _infos
}

fn map_stage(stage: ProtocolStage) -> Stage {
    match stage {
        ProtocolStage::NormalFirstHalfPre => Stage::NormalFirstHalfPre,
        ProtocolStage::NormalFirstHalf => Stage::NormalFirstHalf,
        ProtocolStage::NormalHalfTime => Stage::NormalHalfTime,
        ProtocolStage::NormalSecondHalfPre => Stage::NormalSecondHalfPre,
        ProtocolStage::NormalSecondHalf => Stage::NormalSecondHalf,
        ProtocolStage::ExtraTimeBreak => Stage::ExtraTimeBreak,
        ProtocolStage::ExtraFirstHalfPre => Stage::ExtraFirstHalfPre,
        ProtocolStage::ExtraFirstHalf => Stage::ExtraFirstHalf,
        ProtocolStage::ExtraHalfTime => Stage::ExtraHalfTime,
        ProtocolStage::ExtraSecondHalfPre => Stage::ExtraSecondHalfPre,
        ProtocolStage::ExtraSecondHalf => Stage::ExtraSecondHalf,
        ProtocolStage::PenaltyShootoutBreak => Stage::PenaltyShootoutBreak,
        ProtocolStage::PenaltyShootout => Stage::PenaltyShootout,
        ProtocolStage::PostGame => Stage::PostGame,
    }
}

fn map_command(incoming: ProtocolCommand) -> RefereeCommand {
    match incoming {
        Command::Halt => RefereeCommand::Halt,
        Command::Stop => RefereeCommand::Stop,
        Command::NormalStart => RefereeCommand::NormalStart,
        Command::ForceStart => RefereeCommand::ForceStart,
        Command::PrepareKickoffYellow => RefereeCommand::PrepareKickoff(TeamColor::Yellow),
        Command::PrepareKickoffBlue => RefereeCommand::PrepareKickoff(TeamColor::Blue),
        Command::PreparePenaltyYellow => RefereeCommand::PreparePenalty(TeamColor::Yellow),
        Command::PreparePenaltyBlue => RefereeCommand::PreparePenalty(TeamColor::Blue),
        Command::DirectFreeYellow => RefereeCommand::DirectFree(TeamColor::Yellow),
        Command::DirectFreeBlue => RefereeCommand::DirectFree(TeamColor::Blue),
        Command::TimeoutYellow => RefereeCommand::Timeout(TeamColor::Yellow),
        Command::TimeoutBlue => RefereeCommand::Timeout(TeamColor::Blue),
        Command::BallPlacementYellow => RefereeCommand::BallPlacement(TeamColor::Yellow),
        Command::BallPlacementBlue => RefereeCommand::BallPlacement(TeamColor::Blue),
        Command::IndirectFreeYellow
        | Command::IndirectFreeBlue
        | Command::GoalYellow
        | Command::GoalBlue => RefereeCommand::Deprecated,
    }
}

fn map_point(point: ProtocolPoint) -> Point2<f64> {
    Point2::new(point.x.into(), point.y.into())
}

struct RefereeDeserializationError;

fn map_protobuf_referee(
    packet: ProtocolReferee,
    team_color: &TeamColor,
) -> Result<Referee, RefereeDeserializationError> {
    let (ally, enemy) = match team_color {
        TeamColor::Yellow => (packet.yellow, packet.blue),
        TeamColor::Blue => (packet.blue, packet.yellow),
    };

    Ok(Referee {
        source_identifier: packet.source_identifier,
        match_type: None, // TODO : Finish
        packet_timestamp: create_date_time((packet.packet_timestamp / 1_000_000) as i64),
        stage: ProtocolStage::from_i32(packet.stage)
            .map(map_stage)
            .ok_or(RefereeDeserializationError)?,
        stage_time_left: packet
            .stage_time_left
            .map(|d| Duration::microseconds(d as i64)),
        command: ProtocolCommand::from_i32(packet.command)
            .map(map_command)
            .ok_or(RefereeDeserializationError)?,
        command_counter: packet.command_counter,
        command_timestamp: create_date_time((packet.command_timestamp / 1_000_000) as i64),
        ally: to_team_infos(ally),   // TODO : Rename and check
        enemy: to_team_infos(enemy), // TODO : Rename and check
        designated_position: packet.designated_position.map(map_point),
        positive_half: packet.blue_team_on_positive_half.map(|b| {
            if b {
                TeamColor::Blue
            } else {
                TeamColor::Yellow
            }
        }),
        next_command: packet
            .next_command
            .map(|c| ProtocolCommand::from_i32(c))
            .flatten()
            .map(map_command),
        game_events: vec![],          // TODO:
        game_event_proposals: vec![], // TODO:
        current_action_time_remaining: packet
            .current_action_time_remaining
            .map(|d| Duration::microseconds(d as i64)),
    })
    // Ok(Referee {})
}

impl PreFilter for GameControllerPreFilter {
    fn step(
        &mut self,
        inbound_data: &mut InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        filter_data.referee.extend(
            inbound_data
                .gc_packet
                .drain(..)
                .filter_map(|packet| map_protobuf_referee(packet, team_color).ok()),
        );
    }
}
