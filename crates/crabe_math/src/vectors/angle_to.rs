use std::ops::Sub;

use nalgebra::Point2;

pub fn angle_to_point(p1: Point2<f64>, p2: Point2<f64>) -> f64{
    let dir = p2.sub(p1);
    return (-dir.y).atan2(-dir.x);
}