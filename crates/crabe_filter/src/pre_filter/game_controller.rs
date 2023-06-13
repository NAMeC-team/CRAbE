use crate::data::referee::{MatchType, Referee, RefereeCommand, Stage, TeamInfo};
use crate::data::FilterData;
use crate::pre_filter::PreFilter;
use chrono::{DateTime, Duration, LocalResult, TimeZone, Utc};
use log::error;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crabe_protocol::protobuf::game_controller_packet;
use nalgebra::Point2;

pub struct GameControllerPreFilter;

impl GameControllerPreFilter {}

fn create_date_time(time: f64) -> DateTime<Utc> {
    match Utc.timestamp_opt((time) as i64, 0) {
        LocalResult::Single(dt) => dt,
        LocalResult::None => {
            let now_utc = Utc::now();
            error!("Invalid timestamp, using current time: {}", now_utc);
            now_utc
        }
        LocalResult::Ambiguous(dt_min, dt_max) => {
            let dt_midpoint = dt_min + (dt_max - dt_min) / 2;
            error!("Ambiguous timestamp resolved to midpoint: {}", dt_midpoint);
            dt_midpoint
        }
    }
}

fn get_command(command: i32) -> RefereeCommand {
    match command {
        command => match command {
            0 => RefereeCommand::Halt,
            1 => RefereeCommand::Stop,
            2 => RefereeCommand::NormalStart,
            3 => RefereeCommand::ForceStart,
            4 => RefereeCommand::PrepareKickoff(TeamColor::Yellow),
            5 => RefereeCommand::PrepareKickoff(TeamColor::Blue),
            6 => RefereeCommand::PreparePenalty(TeamColor::Yellow),
            7 => RefereeCommand::PreparePenalty(TeamColor::Blue),
            8 => RefereeCommand::DirectFree(TeamColor::Yellow),
            9 => RefereeCommand::DirectFree(TeamColor::Blue),
            10 => RefereeCommand::IndirectFree(TeamColor::Yellow),
            11 => RefereeCommand::IndirectFree(TeamColor::Blue),
            12 => RefereeCommand::Timeout(TeamColor::Yellow),
            13 => RefereeCommand::Timeout(TeamColor::Blue),
            14 => RefereeCommand::Goal(TeamColor::Yellow),
            15 => RefereeCommand::Goal(TeamColor::Blue),
            16 => RefereeCommand::BallPlacement(TeamColor::Yellow),
            17 => RefereeCommand::BallPlacement(TeamColor::Blue),
            _ => RefereeCommand::Unknow,
        },
    }
}
fn to_team_infos(infos: &game_controller_packet::referee::TeamInfo) -> TeamInfo {
    let _infos = TeamInfo {
        name: infos.name.clone(),
        score: infos.score,
        red_cards: infos.red_cards,
        yellow_card_times: infos.yellow_card_times.clone(),
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
fn convert_referee_protobuf(
    packet: &game_controller_packet::Referee,
    team_color: &TeamColor,
) -> Referee {
    dbg!(packet);
    let ally = match team_color {
        TeamColor::Yellow => to_team_infos(&packet.yellow),
        TeamColor::Blue => to_team_infos(&packet.blue),
    };
    let enemy = match team_color.opposite() {
        TeamColor::Yellow => to_team_infos(&packet.yellow),
        TeamColor::Blue => to_team_infos(&packet.blue),
    };
    let _referee = Referee {
        source_identifier: packet.source_identifier.clone(),
        match_type: match packet.match_type {
            Some(match_type) => match match_type {
                1 => Some(MatchType::GroupPhase),
                2 => Some(MatchType::EliminationPhase),
                3 => Some(MatchType::Friendly),
                _ => Some(MatchType::UnknownMatch),
            },
            None => None,
        },
        packet_timestamp: create_date_time(packet.packet_timestamp as f64),
        stage: match packet.stage {
            stage => match stage {
                0 => Stage::NormalFirstHalfPre,
                1 => Stage::NormalFirstHalf,
                2 => Stage::NormalHalfTime,
                3 => Stage::NormalSecondHalfPre,
                4 => Stage::NormalSecondHalf,
                5 => Stage::ExtraTimeBreak,
                6 => Stage::ExtraFirstHalfPre,
                7 => Stage::ExtraFirstHalf,
                8 => Stage::ExtraHalfTime,
                9 => Stage::ExtraSecondHalfPre,
                10 => Stage::ExtraSecondHalf,
                11 => Stage::PenaltyShootoutBreak,
                12 => Stage::PenaltyShootout,
                13 => Stage::PostGame,
                _ => Stage::Unknow,
            },
        },
        stage_time_left: match packet.stage_time_left {
            Some(duration_value) => Some(Duration::seconds(duration_value as i64)),
            None => Some(Duration::zero()),
        },
        command: get_command(packet.command),
        command_counter: packet.command_counter,
        command_timestamp: create_date_time(packet.command_timestamp as f64),
        ally,
        enemy,
        designated_position: match &packet.designated_position {
            Some(position) => match position {
                _ => Some(Point2::new(position.x as f64, position.y as f64)),
            },
            None => None,
        },
        positive_half: match packet.blue_team_on_positive_half {
            Some(blue_positive) => {
                if blue_positive {
                    Some(TeamColor::Blue)
                } else {
                    Some(TeamColor::Yellow)
                }
            }
            None => None,
        },
        next_command: match packet.next_command {
            Some(command) => Some(get_command(command)),
            None => None,
        },
        game_events: vec![],
        game_event_proposals: vec![],
        current_action_time_remaining: match packet.current_action_time_remaining {
            Some(duration_value) => Some(Duration::seconds(duration_value as i64)),
            None => Some(Duration::zero()),
        },
    };
    _referee
}

impl PreFilter for GameControllerPreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        inbound_data.gc_packet.iter().for_each(|gc_packet| {
            filter_data.referee.push(convert_referee_protobuf(gc_packet, team_color));
        });
        // TODO: The referee message needs to be inside our own framework.

        /*filter_data
        .referee
        .extend(inbound_data.gc_packet.iter().cloned());*/
    }
}
