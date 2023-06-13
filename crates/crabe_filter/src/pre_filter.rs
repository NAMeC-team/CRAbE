use crate::data::FilterData;
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use log::error;

pub mod game_controller;
pub mod vision;

pub fn create_date_time(time: f64) -> DateTime<Utc> {
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

pub trait PreFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    );
}
