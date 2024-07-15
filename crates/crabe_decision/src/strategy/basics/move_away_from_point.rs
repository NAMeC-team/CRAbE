use crate::action::move_to::MoveTo;
use crabe_framework::data::world::{AllyInfo, Ball, Robot};
use crabe_math::shape::Line;
use crabe_math::vectors;
use nalgebra::{distance, Point2};

pub fn move_away(
    robot_pos: Point2<f64>,
    away_point: Point2<f64>,
    distance:f64,
) -> Option<MoveTo>{
    // LOOK AT BALL
    let orientation = vectors::angle_to_point(robot_pos,away_point);

    let dist_robot_target = (robot_pos - away_point).norm();
    if dist_robot_target < distance{

        let target = away_point + (robot_pos - away_point).normalize();

        return Some(
            MoveTo::new(away_point + (robot_pos - away_point).normalize() * distance
            , orientation, 
            0., 
            false, 
            None, 
            true
        ))
    }
    None
    
}