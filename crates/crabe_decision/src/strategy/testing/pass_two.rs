use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::action::ActionWrapper;
use crate::action::move_to_builder::MoveToBuilder;
use crate::action::order_raw::RawOrder;
use crate::strategy::basics::{intercept, pass};
use crate::strategy::Strategy;

#[derive(Default)]
enum PTState {
    #[default]
    Passing,
    Shooting,
}

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct PassTwo {
    /// The id of the robot that does the pass
    id_passer: u8,

    /// The id of the robot that receives the pass
    id_receiver: u8,
    
    /// strategy state
    state: PTState
}



impl PassTwo {
    pub fn new(id_passer: u8, id_receiver: u8) -> Self {
        Self { id_passer, id_receiver, state: PTState::Passing }
    }
}

impl Strategy for PassTwo {
    fn name(&self) -> &'static str { "PassTwo" }

    fn step(&mut self, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        action_wrapper.clear_all();
        if let Some(ball) = &world.ball {
            if let Some(passer) = world.allies_bot.get(&self.id_passer) {
                if let Some(receiver) = world.allies_bot.get(&self.id_passer) {
                    match self.state {
                        PTState::Passing => {
                            // passer
                            let behind_ball_position = ball.position_2d() + (ball.position_2d() - receiver.pose.position).normalize() * 0.2;
                            let cmd = behind_ball_position - passer.pose.position;
                            action_wrapper.push(
                                self.id_passer,
                                RawOrder::new(Command {
                                    forward_velocity: cmd.x as f32,
                                    left_velocity: cmd.y as f32,
                                    angular_velocity: 0_f32,
                                    ..Command::default()
                                }),
                            );
                            
                            // receiver
                            // action_wrapper.push(
                            //     self.id_receiver,
                            //     MoveToBuilder::new()
                            //         .set_target((ball.position_2d().x))
                            //         .build()
                            // )
                        }
                        PTState::Shooting => {}
                    }
                    
                    // receiver
                    action_wrapper.push(self.id_receiver, intercept(receiver, ball));


                }
            }
        }
        false
    }
}