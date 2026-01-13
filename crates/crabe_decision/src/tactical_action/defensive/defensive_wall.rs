use crate::action::move_to::MoveTo;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;

pub fn create_defensive_wall_action() -> TacticalAction {
    TacticalAction::new(
        ActionId::FormDefensiveWall,
        Some(RoleId::DefensiveWall),
        |robot: &Robot<AllyInfo>, world: &World| {
            // Calculer position du mur
            let ball_pos = world.ball.as_ref()
                .map(|b| b.position_2d())
                .unwrap_or(Point2::new(0.0, 0.0));
            
            let our_goal_x = if world.geometry.field.length > 0.0 {
                -world.geometry.field.length / 2.0
            } else {
                -4.5
            };
            let goal = Point2::new(our_goal_x, 0.0);
            
            let ideal_pos = Point2::new(
                goal.x + (ball_pos.x - goal.x) * 0.4,
                goal.y + (ball_pos.y - goal.y) * 0.4,
            );
            
            let distance = robot.distance(&ideal_pos) as f32;
            let angle = (ball_pos.y - ideal_pos.y).atan2(ball_pos.x - ideal_pos.x);
            
            let move_to = MoveTo::new_all_params(ideal_pos, angle, 0.0, false, None, true, true);
            
            // Toujours possible (action par défaut)
            Some((ActionMetric::Distance(OrderedFloat(distance)), move_to))
        },
    )
}