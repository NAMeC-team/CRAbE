use crate::action::move_to::MoveTo;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, World};
use crabe_math::shape::{Circle, Line, Rectangle};
use crabe_math::vectors::{self, rotate_vector};
use nalgebra::{distance, Point2};

const INNACURACY: f64 = 0.2; // Using to avoid bugs when robot is already in the target position and turn around the point

pub fn move_away(
    robot_pos: Point2<f64>,
    away_point: Point2<f64>,
    world: &World,
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
                true,
                true,
            ))
        }
        let dir = (robot_pos - away_point).normalize()* (distance+INNACURACY);
        let mut target = away_point + dir;
        let field_rect = Rectangle::new(world.geometry.field.length - world.geometry.robot_radius*2., world.geometry.field.width - world.geometry.robot_radius*2., Point2::new(-(world.geometry.field.length- world.geometry.robot_radius)/2., -(world.geometry.field.width - world.geometry.robot_radius)/2.));
        let mut angle = 0.;
        while !field_rect.is_inside(target) && angle < 2.*std::f64::consts::PI{
            let dir = rotate_vector(dir, angle);
            angle += 0.1;
            target = away_point + dir;
        }
        return Some(
            MoveTo::new(target,
             orientation, 
            0., 
            false, 
            None, 
            true,
            true
        ))
        
    }
    None
    
}