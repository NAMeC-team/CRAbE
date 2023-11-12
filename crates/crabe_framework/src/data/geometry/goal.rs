use nalgebra::Point2;
use crabe_math::shape::{Line, Rectangle};
use serde::Serialize;

/// Represents a goal on a soccer field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The width of the goal, in meters.
    pub width: f64,
    /// The depth of the goal, in meters.
    pub depth: f64,
    /// Describes the 4 points of the goal area, in meters
    pub area: Rectangle,
    /// Center of the goal, starts from the front of the goal to its inside, parallel to the goal posts
    pub center_line: Line,
    /// The front line of the goal, measured from the origin of the field,
    /// in meters.
    pub front_line: Line,
}

impl Goal {
    pub fn new(width: f64, depth: f64, area: Rectangle, positive: bool) -> Goal {

        let front_line: Line;
        let center_line: Line;

        if positive {
            center_line = Line {
                start: Point2::new(area.center.x + depth, area.center.y + depth),
                end: Point2::new(area.center.x - depth, area.center.y - depth),
            };
            front_line = Line {
                start: area.top_left,
                end: area.bottom_left
            }
        } else {
            center_line = Line {
                start: Point2::new(area.center.x - depth, area.center.y - depth),
                end: Point2::new(area.center.x + depth, area.center.y + depth),
            };
            front_line = Line {
                start: area.top_right,
                end: area.bottom_right
            }
        };

        Self {
            width,
            depth,
            area,
            center_line,
            front_line,
        }
    }
}