use crate::action::move_to::MoveTo;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;

pub fn create_mark_dangerous_opponent_action() -> TacticalAction {
    TacticalAction::new(
        ActionId::MarkDangerousOpponent,
        Some(RoleId::OpponentMarker),
        |robot: &Robot<AllyInfo>, world: &World| {
            let ball_pos = world.ball.as_ref()?.position_2d();
            
            let our_goal_x = if world.geometry.field.length > 0.0 {
                -world.geometry.field.length / 2.0
            } else {
                -4.5
            };
            let goal = Point2::new(our_goal_x, 0.0);
            
            // Trouve l'adversaire le plus dangereux (exclut le porteur)
            let dangerous_opponent = world.enemies_bot.iter()
                .filter(|(_, enemy)| enemy.distance(&ball_pos) > 0.15)
                .min_by_key(|(_, enemy)| {
                    let ball_dist = enemy.distance(&ball_pos);
                    let goal_dist = enemy.distance(&goal);
                    OrderedFloat((ball_dist * 0.5 + goal_dist * 0.5) as f32)
                })?;
            
            let distance = robot.distance(&dangerous_opponent.1.pose.position) as f32;
            
            if distance < 5.0 {
                // Calculer le MoveTo (placeholder à améliorer)
                let target = robot.pose.position;
                let angle = robot.pose.orientation;
                
                let move_to = MoveTo::new_all_params(target, angle, 0.0, false, None, true , true);
                
                Some((ActionMetric::Distance(OrderedFloat(distance)), move_to))
            } else {
                None
            }
        },
    )
}