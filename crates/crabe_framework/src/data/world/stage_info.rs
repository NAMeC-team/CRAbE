use chrono::Duration;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::data::referee::Stage;

#[derive(Clone, Debug)]
pub struct StageInfo {
    /// The current match stage (first half, second half, half-time...)
    pub stage: Stage,
    /// Time left for the current stage
    pub time_left: Option<Duration>,
}

impl Serialize for StageInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut ser = serializer.serialize_struct("StageInfo", 2)?;
        ser.serialize_field("stage", &self.stage)?;
        let seconds = match &self.time_left {
            None => { 0 }
            Some(time) => { time.num_seconds() }
        };
        ser.serialize_field("time_left_secs", &seconds)?;
        ser.end()
    }
}

impl Default for StageInfo {
    fn default() -> Self {
        Self {
            stage: Stage::NormalFirstHalfPre,
            time_left: None,
        }
    }
}