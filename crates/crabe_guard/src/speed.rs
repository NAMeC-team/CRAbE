use crate::constant::{MAX_ANGULAR, MAX_LINEAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;

pub struct SpeedGuard {
    max_linear: f32,
    max_angular: f32,
}

impl SpeedGuard {
    pub fn new(max_linear: f32, max_angular: f32) -> Self {
        Self {
            max_linear,
            max_angular,
        }
    }
}

impl Default for SpeedGuard {
    fn default() -> Self {
        Self {
            max_linear: MAX_LINEAR,
            max_angular: MAX_ANGULAR,
        }
    }
}

impl Guard for SpeedGuard {
    fn guard(
        &mut self,
        _world: &World,
        commands: &mut CommandMap,
        _tool_commands: &mut ToolCommands,
    ) {
        commands.iter_mut().for_each(|(_id, command)| {
            command.forward_velocity = command
                .forward_velocity
                .clamp(-self.max_linear, self.max_linear);
            command.left_velocity = command
                .left_velocity
                .clamp(-self.max_linear, self.max_linear);
            command.angular_velocity = command
                .angular_velocity
                .clamp(-self.max_angular, self.max_angular);
        });
    }
}
