use serde::Serialize;
// TODO : Document
// TODO: This information doesn't represent the goal
#[derive(Serialize, Clone, Debug)]
pub struct Goal {
    pub width: f32,
    pub depth: f32,
}

// TODO : Implement some helper methods
