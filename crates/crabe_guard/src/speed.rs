use crate::constant::{MAX_ANGULAR, MAX_DRIBBLER, MAX_LINEAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use nalgebra::Vector2;
use log::warn;
use crabe_framework::data::world::game_state::GameState;

pub struct SpeedGuard {
    max_linear: f32,
    max_angular: f32,
    max_dribbler: f32,
}

impl SpeedGuard {
    pub fn new(max_linear: f32, max_angular: f32,max_dribbler: f32) -> Self {
        Self {
            max_linear,
            max_angular,
            max_dribbler
        }
    }
}

impl Default for SpeedGuard {
    fn default() -> Self {
        Self {
            max_linear: MAX_LINEAR,
            max_angular: MAX_ANGULAR,
            max_dribbler: MAX_DRIBBLER,
        }
    }
}

impl Guard for SpeedGuard {
    fn guard(
        &mut self,
        world: &World,
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
            let mut max_speed = self.max_linear;
            match world.data.ref_orders.state {
                GameState::Stopped(_) => {
                    max_speed = 1.;// world.data.ref_orders.speed_limit;
                }
                _ => {}
            }

            if direction.norm() > max_speed {
                let direction_normalized = direction.normalize() * max_speed;
                command.forward_velocity = direction_normalized.x;
                command.left_velocity = direction_normalized.y;
            }


            

            if command.angular_velocity.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in angular_velocity. It has been adjusted to 0.");
                command.angular_velocity = command
                    .angular_velocity
                    .clamp(-self.max_angular, self.max_angular);
            }


            if command.dribbler.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in dribbler. It has been adjusted to 0.");
                command.dribbler = 0.;
            }

            if command.dribbler > self.max_dribbler {
                warn!("An attempt was made to send a dribbler speed higher than the maximum allowed. It has been adjusted to the maximum allowed value. Wich is {}", self.max_dribbler);
                command.dribbler = self.max_dribbler;
            }


        });
    }
}
