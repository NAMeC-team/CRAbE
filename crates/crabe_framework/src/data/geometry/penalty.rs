use serde::Serialize;
use crabe_math::shape::Rectangle;

/// Represents a penalty area on a soccer field.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    /// The width of the penalty area in meters.
    pub width: f64,
    /// The depth of the penalty area in meters.
    pub depth: f64,
    /// The area covered by this penalty zone
    pub area: Rectangle,
}

impl Penalty {}
