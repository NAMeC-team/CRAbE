use serde::{Deserialize, Serialize};
use crate::data::referee::TeamInfo;

/// The `TeamColor` enum represents the color of a team in the SSL game, either blue or yellow.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum TeamColor {
    Blue,
    Yellow,
}

impl TeamColor {
    /// Returns the opposite color of the current color.
    pub fn opposite(&self) -> Self {
        match self {
            TeamColor::Blue => TeamColor::Yellow,
            TeamColor::Yellow => TeamColor::Blue,
        }
    }
}

/// The `Team` struct represents a team in the SSL game, including its color and optional name.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The color of the team.
    color: TeamColor,
    /// Information about the team, provided by the game controller
    info: Option<TeamInfo>
}

impl Team {
    /// Creates a new team with the given color.
    pub fn with_color(color: TeamColor) -> Self {
        Self { color, info: None }
    }

    /// Updates the stored info about the
    pub fn update_info(&mut self, infos: &TeamInfo) {
        self.info = Some(infos.clone());
    }
}
