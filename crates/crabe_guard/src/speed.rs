use crate::constant::{MIN_LINEAR, MAX_LINEAR, MIN_ANGULAR, MAX_ANGULAR, MAX_DRIBBLER, EPSILON_LINEAR , EPSILON_ANGULAR};
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use nalgebra::Vector2;
use log::{info, warn};
use crabe_framework::data::world::game_state::GameState;

pub struct SpeedGuard {
    min_linear: f32,
    max_linear: f32,
    min_angular: f32,
    max_angular: f32,
    max_dribbler: f32,
}

impl SpeedGuard {
    pub fn new(min_linear : f32,max_linear: f32,min_angular: f32, max_angular: f32,max_dribbler: f32) -> Self {
        Self {
            min_linear,
            max_linear,
            min_angular,
            max_angular,
            max_dribbler
        }
    }
}

impl Default for SpeedGuard {
    fn default() -> Self {
        Self {
            min_linear : MIN_LINEAR,
            max_linear: MAX_LINEAR,
            min_angular : MIN_ANGULAR,
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

            let mut direction = Vector2::new(command.forward_velocity, command.left_velocity);
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

            let mut x_abs = direction.x.abs();
            let mut y_abs = direction.y.abs();

            if direction.norm() != 0. && (x_abs < self.max_angular || y_abs < self.max_angular) {
                if x_abs > EPSILON_LINEAR && x_abs < self.min_angular {
                    let fact = self.min_angular / x_abs;
                    direction.x *= fact;
                    direction.y *= fact;
                }


                y_abs = direction.y.abs();
                if y_abs > EPSILON_LINEAR && y_abs < self.min_angular {
                    let fact = self.min_angular / y_abs;
                    direction.x *= fact;
                    direction.y *= fact;
                }

                y_abs = direction.y.abs();
                x_abs = direction.x.abs();

                if x_abs < EPSILON_LINEAR {
                    direction.x = 0.;
                }
                if y_abs < EPSILON_LINEAR {
                    direction.y = 0.;
                }


                command.forward_velocity = direction.x;
                command.left_velocity = direction.y;
            }

            if command.angular_velocity.is_nan() {
                warn!("An attempt was made to send NaN instead of a valid value in angular_velocity. It has been adjusted to 0.");
                command.angular_velocity = 0.;

            } else if command.angular_velocity > self.max_angular || command.angular_velocity < -self.max_angular {
                warn!("Angular speed out of range. It has been clamped from {:?} to {:?}", -self.max_angular, self.max_angular);
                command.angular_velocity = command
                    .angular_velocity
                    .clamp(-self.max_angular, self.max_angular);
            } else if command.angular_velocity != 0.0 && EPSILON_ANGULAR < command.angular_velocity.abs() && command.angular_velocity.abs() < self.min_angular {
                command.angular_velocity = self.min_angular * command.angular_velocity.signum();
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
