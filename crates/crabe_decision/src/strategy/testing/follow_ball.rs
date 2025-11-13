use crate::action::move_to_builder::MoveToBuilder;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

#[derive(Default)]
pub struct FollowBall {
    id: u8,
}

/// A strategy that commands a robot to follow the ball
impl FollowBall {
    /// Creates a new FollowBall instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id}
    }
}

impl Strategy for FollowBall {
    fn name(&self) -> &'static str {
        "FollowBall"
    }

            #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);
        if let Some(ball) = &world.ball {
            if let Some(robot) = world.allies_bot.get(&self.id) {
                let mut moveto = MoveToBuilder::new();
                moveto.set_target(ball.position_2d()).set_orientation(robot.angle_to(ball.position_2d()));
                if robot.has_ball {
                    println!("Robot {} has ball", self.id);
                    // moveto.set_kick(Kick::StraightKick { power: 3. });
                }else{
                    println!("Robot {} doesn't have ball", self.id);
                }
                action_wrapper.push(
                    self.id,
                    moveto.build()
                );
            }
        }
        false
    }
}
