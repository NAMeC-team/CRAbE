use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{Robot,AllyInfo, World};
use nalgebra::Point2;

pub struct MarkDangerousOpponentAction;

impl TacticalAction for MarkDangerousOpponentAction {
    fn evaluate(&self, robot: &Robot<AllyInfo>, world: &World) -> Option<ActionMetric> {
        let ball_pos = world.ball.as_ref()?.position_2d();
        
        let our_goal_x = if world.geometry.field.length  > 0.0 {
            -world.geometry.field.length / 2.0
        } else {
            -4.5
        };
        let goal = Point2::new(our_goal_x, 0.0);
        
        // Trouve l'adversaire le plus dangereux (exclut le porteur)
        let dangerous_opponent = world.enemies_bot.iter()
            .filter(|(_, enemy)| enemy.distance(&ball_pos) > 0.15) // Pas le porteur
            .min_by_key(|(_, enemy)| {
                let ball_dist = enemy.distance(&ball_pos);
                let goal_dist = enemy.distance(&goal);
                OrderedFloat((ball_dist * 0.5 + goal_dist * 0.5) as f32)
            })?;
        
        let distance = robot.distance(&dangerous_opponent.1.pose.position) as f32;
        
        if distance < 5.0 {
            Some(ActionMetric::Distance(OrderedFloat(distance)))
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
        // Implémentation similaire à mark_ball_carrier
        let ball_pos = world.ball.as_ref().map(|b| b.position_2d());
        if ball_pos.is_none() {
            return;
        }
        
        let target = robot.pose.position; // Placeholder
        let angle = robot.pose.orientation;
        
        action_wrapper.push(
            robot_id,
            MoveTo::new_all_params(target, angle, 0.0, false, None, true, true),
        );
    }
    
    fn action_id(&self) -> ActionId {
        ActionId::MarkDangerousOpponent
    }
    
    fn corresponding_role(&self) -> Option<RoleId> {
        Some(RoleId::OpponentMarker)
    }
}