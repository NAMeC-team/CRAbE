use nalgebra::Point2;
use crabe_math::shape::{Line, Rectangle};
use serde::Serialize;

/// Represents a goal on a soccer field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// Describes the 4 points of the goal area, in meters
    pub area: Rectangle,
    /// Center of the goal, starts from the front of the goal to its inside, parallel to the goal posts
    pub center_line: Line,
    /// The front line of the goal, measured from the origin of the field, in meters.
    pub front_line: Line,
}

impl Goal {
    pub fn new(goal_width: f64, goal_depth: f64, top_left_position: Point2<f64>, positive: bool) -> Goal {
        let area = Rectangle::new(
            goal_depth, goal_width, top_left_position
        );

        let depth = area.width;
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
            area,
            center_line,
            front_line,
        }
    }

    /// The width of the goal, in meters, as defined per the SSL rulebook
    pub fn width(&self) -> &f64 {
        return &self.area.height;
    }

    /// The depth of the goal, in meters, as defined per the SSL rulebook
    pub fn depth(&self) -> &f64 {
        return &self.area.width;
    }
}