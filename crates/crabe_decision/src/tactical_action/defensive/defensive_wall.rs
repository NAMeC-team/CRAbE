use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::decision::{ActionMetric, OrderedFloat, RoleId};
use crate::tactical_action::{ActionId, TacticalAction};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;

pub struct DefensiveWallAction;

impl TacticalAction for DefensiveWallAction {
    fn evaluate(&self, robot: &Robot<AllyInfo>, _world: &World) -> Option<ActionMetric> {
        // Toujours possible (action par défaut)
        let ideal_pos = self.calculate_wall_position(_world);
        let distance = robot.distance(&ideal_pos) as f32;
        Some(ActionMetric::Distance(OrderedFloat(distance)))
    }
    
    fn execute(
        &self,
        robot_id: u8,
        _robot: &Robot<AllyInfo>,
        world: &World,
        action_wrapper: &mut ActionWrapper,
        _tools_data: &mut ToolData,
    ) {
        let target = self.calculate_wall_position(world);
        let ball_pos = world.ball.as_ref().map(|b| b.position_2d())
            .unwrap_or(Point2::new(0.0, 0.0));
        
        let angle = (ball_pos.y - target.y).atan2(ball_pos.x - target.x);
        
        action_wrapper.push(
            robot_id,
            MoveTo::new_all_params(target, angle, 0.0, false, None, true,true),
        );
    }
    
    fn action_id(&self) -> ActionId {
        ActionId::FormDefensiveWall
    }
    
    fn corresponding_role(&self) -> Option<RoleId> {
        Some(RoleId::DefensiveWall)
    }
}

impl DefensiveWallAction {
    fn calculate_wall_position(&self, world: &World) -> Point2<f64> {
        let ball_pos = world.ball.as_ref()
            .map(|b| b.position_2d())
            .unwrap_or(Point2::new(0.0, 0.0));
        
        let our_goal_x = if world.geometry.field.length > 0.0 {
            -world.geometry.field.length / 2.0
        } else {
            -4.5
        };
        let goal = Point2::new(our_goal_x, 0.0);
        
        Point2::new(
            goal.x + (ball_pos.x - goal.x) * 0.4,
            goal.y + (ball_pos.y - goal.y) * 0.4,
        )
    }
}