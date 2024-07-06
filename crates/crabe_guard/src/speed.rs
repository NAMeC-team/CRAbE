use crate::constant::{MAX_ANGULAR, MAX_LINEAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use nalgebra::Vector2;
use log::warn;

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
            // Replacing any NaN values that might be computed to 0.
            // nalgebra docs mention you shouldn't compare with f32::NaN and should use the .is_nan() method instead
            if command.forward_velocity.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in forward_velocity. It has been adjusted to 0.");
                command.forward_velocity = 0.;
            } 
            if command.left_velocity.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in left_velocity. It has been adjusted to 0.");
                command.left_velocity = 0.;
            }

            let direction = Vector2::new(command.forward_velocity, command.left_velocity);
            if direction.norm() > self.max_linear {
                let direction_normalized = direction.normalize() * self.max_linear;
                command.forward_velocity = direction_normalized.x;
                command.left_velocity = direction_normalized.y;
            }


            if command.angular_velocity.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in angular_velocity. It has been adjusted to 0.");
                command.angular_velocity = command
                    .angular_velocity
                    .clamp(-self.max_angular, self.max_angular);
            }

        });
    }
}
