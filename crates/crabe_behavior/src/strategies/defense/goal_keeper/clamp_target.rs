use crate::behaviors::{Behavior, Context, Status};

#[derive(Debug)]
pub struct ClampGoalKeepingTarget;

impl Behavior for ClampGoalKeepingTarget {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        // let world = ctx.world();
        //
        // let goal_half_width = world.geometry.ally_goal.width / 2.0;
        // if goal_half_width > world.geometry.robot_radius {
        //     let (min, max) = (-goal_half_width + world.geometry.robot_radius,goal_half_width - world.geometry.robot_radius);
        //     let target = ctx.target_mut();
        //     target.position.y = target.position.y.clamp(min, max);
        //
        // }

        Status::Success
    }
}
