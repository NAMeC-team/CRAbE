use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;

pub struct MarkBallCarrierAction;

impl TacticalAction for MarkBallCarrierAction {
    fn evaluate(&self, robot: &Robot<AllyInfo>, world: &World) -> Option<ActionMetric> {
        // Trouver l'adversaire avec la balle
        let ball_carrier = world.enemies_bot.iter()
            .find(|(_, enemy)| {
                if let Some(ball) = &world.ball {
                    enemy.distance(&ball.position_2d()) < 0.15 // 15cm = possession
                } else {
                    false
                }
            })?;
        
        let distance = robot.distance(&ball_carrier.1.pose.position);
        
        // Peut marquer seulement si à moins de 4m
        if distance < 4.0 {
            Some(ActionMetric::Distance(OrderedFloat(distance as f32)))
        } else {
            None
        }
    }
    
    fn execute(
        &self,
        robot_id: u8,
        robot: &Robot<AllyInfo>,
        world: &World,
        action_wrapper: &mut ActionWrapper,
        _tools_data: &mut ToolData,
    ) {
        let ball_carrier = world.enemies_bot.iter()
            .find(|(_, enemy)| {
                if let Some(ball) = &world.ball {
                    enemy.distance(&ball.position_2d()) < 0.15
                } else {
                    false
                }
            });
        
        if let Some((_, enemy)) = ball_carrier {
            // Position entre l'adversaire et notre but
            let our_goal_x = if world.geometry.field.length  > 0.0 {
                -world.geometry.field.length  / 2.0
            } else {
                -4.5
            };
            let goal_pos = Point2::new(our_goal_x, 0.0);
            
            let dir_x = goal_pos.x - enemy.pose.position.x;
            let dir_y = goal_pos.y - enemy.pose.position.y;
            let norm = (dir_x * dir_x + dir_y * dir_y).sqrt();
            
            let target = Point2::new(
                enemy.pose.position.x + (dir_x / norm) * 0.5,
                enemy.pose.position.y + (dir_y / norm) * 0.5,
            );
            
            let angle = (enemy.pose.position.y - robot.pose.position.y)
                .atan2(enemy.pose.position.x - robot.pose.position.x);
            
            action_wrapper.push(
                robot_id,
                MoveTo::new_all_params(target, angle, 0.0, false, None, true, true),
            );
        }
    }
    
    fn action_id(&self) -> ActionId {
        ActionId::MarkBallCarrier
    }
    
    fn corresponding_role(&self) -> Option<RoleId> {
        Some(RoleId::BallCarrierMarker)
    }
}