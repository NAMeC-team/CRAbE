use crate::data::world::{Team, TeamColor};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct GameState {
    pub ally: Team,
    pub enemy: Team,
    pub positive_half: TeamColor,
}

impl GameState {
    pub fn new(team_color: TeamColor) -> Self {
        Self {
            ally: Team::with_color(team_color),
            enemy: Team::with_color(team_color.opposite()),
            positive_half: team_color.opposite(),
        }
    }
}
