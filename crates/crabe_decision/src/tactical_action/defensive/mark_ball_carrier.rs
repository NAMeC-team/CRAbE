use crate::action::move_to::MoveTo;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;

pub fn create_mark_ball_carrier_action() -> TacticalAction {
    TacticalAction::new(
        ActionId::MarkBallCarrier,
        Some(RoleId::BallCarrierMarker),
        |robot: &Robot<AllyInfo>, world: &World| {
            // Trouver l'adversaire avec la balle
            let ball_carrier = world.enemies_bot.iter()
                .find(|(_, enemy)| {
                    if let Some(ball) = &world.ball {
                        enemy.distance(&ball.position_2d()) < 0.15
                    } else {
                        false
                    }
                })?;
            
            let distance = robot.distance(&ball_carrier.1.pose.position) as f32;
            
            // Peut marquer seulement si à moins de 4m
            if distance < 4.0 {
                // Calculer le MoveTo
                let our_goal_x = if world.geometry.field.length > 0.0 {
                    -world.geometry.field.length / 2.0
                } else {
                    -4.5
                };
                let goal_pos = Point2::new(our_goal_x, 0.0);
                
                let dir_x = goal_pos.x - ball_carrier.1.pose.position.x;
                let dir_y = goal_pos.y - ball_carrier.1.pose.position.y;
                let norm = (dir_x * dir_x + dir_y * dir_y).sqrt();
                
                let target = Point2::new(
                    ball_carrier.1.pose.position.x + (dir_x / norm) * 0.5,
                    ball_carrier.1.pose.position.y + (dir_y / norm) * 0.5,
                );
                
                let angle = (ball_carrier.1.pose.position.y - robot.pose.position.y)
                    .atan2(ball_carrier.1.pose.position.x - robot.pose.position.x);
                
                let move_to = MoveTo::new_all_params(target, angle, 0.0, false, None, true, true);
                
                Some((ActionMetric::Distance(OrderedFloat(distance)), move_to))
            } else {
                None
            }
        },
    )
}