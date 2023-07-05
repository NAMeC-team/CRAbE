use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::constants::{KEEPER_ID, PIVOT_ID, DEFENDER1_ID, DEFENDER2_ID, ATTACKER1_ID, ATTACKER2_ID};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;
use crabe_math::shape::Line;
use std::f64::consts::PI;

/// The PrepareFreeKickEnemy struct represents a strategy that commands the team to set in the PrepareFreeKickEnemy formation
/// It is used when the team is in favor of the kick-off
#[derive(Default)]
pub struct PrepareFreeKickEnemy {
}

impl PrepareFreeKickEnemy {
    /// Creates a new PrepareFreeKickEnemy instance with the desired robot id.
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for PrepareFreeKickEnemy {
    /// Executes the PrepareFreeKickEnemy strategy.
    ///
    /// This strategy commands all the robots to move in position for
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clean(KEEPER_ID);
        action_wrapper.clean(PIVOT_ID);
        action_wrapper.clean(ATTACKER1_ID);
        action_wrapper.clean(ATTACKER2_ID);
        action_wrapper.clean(DEFENDER1_ID);
        action_wrapper.clean(DEFENDER2_ID);

        let ball_pos = match world.ball.clone() {
            None => {
                Point2::new(0.0,0.0)
            }
            Some(ball) => {
                ball.position.xy()
            }
        };    
        if let Some(bappe) = world.allies_bot.get(&DEFENDER2_ID) {
            action_wrapper.push(DEFENDER2_ID, MoveTo::new(Point2::new(-3.5, -0.3), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
        };
        if let Some(bappe) = world.allies_bot.get(&PIVOT_ID) {
            if ball_pos.x>-2.5{
                action_wrapper.push(PIVOT_ID, MoveTo::new(Point2::new(ball_pos.x-1.8, ball_pos.y+0.5), vectors::angle_to_point(ball_pos, bappe.pose.position),0.0 , None, false,false));
            } else {
                action_wrapper.push(PIVOT_ID, MoveTo::new(Point2::new(-3.5, 0.3), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
            }    
        };
        if let Some(bappe) = world.allies_bot.get(&DEFENDER1_ID) {
            action_wrapper.push(DEFENDER1_ID, MoveTo::new(Point2::new(-3.5, 0.3), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
        };
        if let Some(bappe) = world.allies_bot.get(&ATTACKER2_ID) {
            let mut shoot_dir = Line::new(ball_pos,Point2::new(world.geometry.ally_goal.bottom_left_position.x,world.geometry.ally_goal.bottom_left_position.y));
            let mut wall_dir = Line::new(Point2::new(ball_pos.x-0.6,5.),Point2::new(ball_pos.x-0.6,-5.));
            if let Some(intersection) = wall_dir.intersection_line(&shoot_dir) {
                action_wrapper.push(ATTACKER2_ID, MoveTo::new(Point2::new(intersection.x, intersection.y), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
            } else{
                action_wrapper.push(ATTACKER2_ID, MoveTo::new(Point2::new(-3.25, 3.), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
            }
        };
        if let Some(bappe) = world.allies_bot.get(&ATTACKER1_ID) {
            if ball_pos.x>-2.5{
                action_wrapper.push(ATTACKER1_ID, MoveTo::new(Point2::new(ball_pos.x-1.5, ball_pos.y), vectors::angle_to_point(ball_pos, bappe.pose.position),0.0 , None, false,false));
            } else {
                action_wrapper.push(ATTACKER1_ID, MoveTo::new(Point2::new(-3.5, -1.), vectors::angle_to_point(ball_pos, bappe.pose.position),0.0 , None, false,false));
            }
        };
        if let Some(bappe) = world.allies_bot.get(&KEEPER_ID) {
            action_wrapper.push(KEEPER_ID, MoveTo::new(Point2::new(-4.5, 0.0), vectors::angle_to_point(ball_pos, bappe.pose.position), 0.0,None, false, false));
        };
        false
    }
    fn name(&self) -> &'static str {
        return "PrepareFreeKickEnemy";
    }
}

