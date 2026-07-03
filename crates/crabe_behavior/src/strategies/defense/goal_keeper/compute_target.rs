use crate::behaviors::blackboard::{InputPort, OutputPort};
use crate::behaviors::{Behavior, Context, Status};
use crate::require;
use crate::strategies::defense::goal_keeper::{
    follow_enemy_direction, follow_enemy_to_ball_trajectory, follow_velocity_trajectory,
};
use crate::utils::closest_bot_to_point;
use crabe_framework::data::world::{Pose, Robot};
use crabe_math::vectors;
use nalgebra::Point2;

#[derive(Debug)]
pub struct ComputeGoalKeepingTarget {
    target: OutputPort<Pose>,
}

impl ComputeGoalKeepingTarget {
    pub fn new(target: OutputPort<Pose>) -> Self {
        Self { target }
    }
}

impl Behavior for ComputeGoalKeepingTarget {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        let robot = require!(ctx.robot());
        let world = ctx.world();
        let goal_center = world.geometry.ally_goal.line.center();

        // default target is goal center, orientation towards ball
        let mut target_position = goal_center;
        let mut orientation_target = Point2::new(0.0, 0.0);

        if let Some(ball) = &world.ball {
            let ball_pos = ball.position_2d();
            orientation_target = ball_pos;

            // Follow ball velocity
            if let Some(intersection) = follow_velocity_trajectory(ball, world) {
                target_position = intersection;
            } else if let Some(closest_enemy) =
                closest_bot_to_point(world.enemies_bot.values().collect(), ball_pos)
            {
                if let Some(intersection) =
                    follow_enemy_to_ball_trajectory(ball, world, closest_enemy)
                {
                    target_position = intersection;
                } else if let Some(intersection) = follow_enemy_direction(world, closest_enemy) {
                    target_position = intersection;
                } else {
                    target_position.y = ball_pos.y;
                }
            } else {
                target_position.y = ball_pos.y;
            }
        }

        let robot_pos = robot.position();
        ctx.set(
            &self.target,
            Pose {
                position: target_position,
                orientation: vectors::angle_to_point(robot_pos, orientation_target),
            },
        );

        Status::Success
    }
}
