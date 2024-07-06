use chrono::{DateTime, LocalResult, TimeZone, Utc};
use log::error;


/// t_capture is in milliseconds
pub fn create_date_time(t_capture: i64) -> DateTime<Utc> {
    
    match Utc.timestamp_opt(t_capture / 1000, (t_capture % 1000) as u32 * 1_000_000) {
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
