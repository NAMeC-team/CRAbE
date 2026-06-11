use nalgebra::{Isometry2, Vector2};

pub fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), orientation)
}
pub fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
    frame.inverse()
}
