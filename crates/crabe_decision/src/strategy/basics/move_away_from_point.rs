use crate::action::move_to::MoveTo;
use crabe_framework::data::world::{AllyInfo, Ball, Robot};
use crabe_math::shape::Line;
use crabe_math::vectors;
use nalgebra::{distance, Point2};

const INNACURACY: f64 = 0.2; // Using to avoid bugs when robot is already in the target position and turn around the point

pub fn move_away(
    robot_pos: Point2<f64>,
    away_point: Point2<f64>,
    distance:f64,
) -> Option<MoveTo>{
    // LOOK AT BALL
    let orientation = vectors::angle_to_point(robot_pos,away_point);

    let dist_robot_target = (robot_pos - away_point).norm();
    if dist_robot_target < distance{
        if dist_robot_target == 0. {
            return Some(
                MoveTo::new( 
                Point2::new(100.,100.),
                0., 
                0., 
                false, 
                None, 
                true
            ))
        }
        
        let target = away_point + (robot_pos - away_point).normalize();

        return Some(
            MoveTo::new(target * (distance+INNACURACY),
             orientation, 
            0., 
            false, 
            None, 
            true
        ))
    }
    None
    
}