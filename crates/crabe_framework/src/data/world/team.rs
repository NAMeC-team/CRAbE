use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TeamColor {
    Blue,
    Yellow,
}

impl TeamColor {
    pub fn opposite(&self) -> Self {
        match self {
            TeamColor::Blue => TeamColor::Yellow,
            TeamColor::Yellow => TeamColor::Blue,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Team {
    color: TeamColor,
    name: Option<String>,
}

impl Team {
    pub fn with_color(color: TeamColor) -> Self {
        Self { color, name: None }
    }
}
