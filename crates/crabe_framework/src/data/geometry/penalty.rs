use serde::Serialize;
// TODO : Document
// TODO: This information doesn't represent the penalty
#[derive(Serialize, Clone, Debug)]
pub struct Penalty {
    pub width: f32,
    pub depth: f32,
}

// TODO : Implement some helper methods
