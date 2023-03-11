use crabe_math::shape::circle::Circle;
use nalgebra::Point2;
use serde::Serialize;

mod goal;
pub use self::goal::Goal;
mod penalty;
pub use self::penalty::Penalty;

#[derive(Serialize, Clone, Debug)]
pub struct Field {
    pub width: f64,
    pub length: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct Geometry {
    pub boundary_width: f64,
    pub field: Field,
    pub ally_goal: Goal,
    pub enemy_goal: Goal,
    pub ally_penalty: Penalty,
    pub enemy_penalty: Penalty,
    pub center: Circle,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            boundary_width: 0.3,
            field: Field {
                width: 9.0,
                length: 6.0,
            },
            ally_goal: Goal {
                width: 1.0,
                depth: 0.18,
                top_left_position: Point2::new(-4.68, -0.5),
            },
            enemy_goal: Goal {
                width: 1.0,
                depth: 0.18,
                top_left_position: Point2::new(4.68, 0.5),
            },
            ally_penalty: Penalty {
                width: 2.0,
                depth: 1.0,
                top_left_position: Point2::new(-4.5, -1.0),
            },
            enemy_penalty: Penalty {
                width: 2.0,
                depth: 1.0,
                top_left_position: Point2::new(4.5, 1.0),
            },
            center: Circle {
                center: Point2::new(0.0, 0.0),
                radius: 0.5,
            },
        }
    }
}
