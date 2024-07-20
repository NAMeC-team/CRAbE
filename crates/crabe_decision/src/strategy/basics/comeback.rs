use crate::action::move_to::MoveTo;
use crate::utils::closest_bot_to_point;
use crabe_framework::data::world::{self, AllyInfo, Robot};
use crabe_math::vectors;
use nalgebra::Point2;

/// Takes the robot back to a safe place on its own field.
///
/// # Arguments
/// - `robot`: The robot that will move back to a safe place
/// - `world`: The current state of the world
///
/// # Returns
/// A `MoveTo` action that will make the robot move to a safe place on the field (mainly for kick-off)
pub fn comeback(
    robot: &Robot<AllyInfo>,
    world: &world::World,
) -> MoveTo {
    let orientation = vectors::angle_to_point(robot.pose.position, Point2::new(0.0, 0.0));
    let target: nalgebra::OPoint<f64, nalgebra::Const<2>> = Point2::new(-0.6, robot.pose.position.y);
    
    // Determine if there is a nearby ally bot at the target position
    if let Some(closest_bot) = closest_bot_to_point(world.allies_bot.values().collect(), target) {
        let proximity_threshold = 0.5;
        let is_close = (closest_bot.pose.position.x - target.x).abs() < proximity_threshold
            && (closest_bot.pose.position.y - target.y).abs() < proximity_threshold;
        
        if is_close && closest_bot.id != robot.id {
            let new_target = if target.y > 0.0 {
                Point2::new(target.x, target.y + 1.0)
            } else {
                Point2::new(target.x, target.y - 1.0)
            };
            return MoveTo::new(new_target, orientation, 0.0, false, None, true, true);
        }
    }
    
    MoveTo::new(target, orientation, 0.0, false, None, true , true)
}
