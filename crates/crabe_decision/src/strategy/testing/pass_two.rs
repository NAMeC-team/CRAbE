use log::info;
use nalgebra::{distance, Point2};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Ball, Robot, World};
use crabe_math::vectors;
use crate::action::ActionWrapper;
use crate::action::move_to::MoveTo;
use crate::action::move_to_builder::MoveToBuilder;
use crate::action::order_raw::RawOrder;
use crate::strategy::basics::{intercept, pass, shoot};
use crate::strategy::offensive::Attacker;
use crate::strategy::Strategy;

#[derive(Default)]
enum PTState {
    #[default]
    Placement,
    Passing,
    Intercept,
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
        Self { id_passer, id_receiver, state: Default::default() }
    }
    fn receiver_waits(&self, action_wrapper: &mut ActionWrapper, ball: &Ball, receiver: &Robot<AllyInfo>) {
        action_wrapper.push(
            self.id_receiver,
            MoveToBuilder::new()
                .set_target(Point2::new((ball.position_2d().x) - 1., 1.5))
                .set_orientation(vectors::angle_to_point(receiver.pose.position, ball.position_2d()))
                .no_avoidance()
                .build()
        );
    }
}



impl Strategy for PassTwo {
    fn name(&self) -> &'static str { "PassTwo" }

    fn step(&mut self, world: &World, tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        action_wrapper.clear_all();
        if let Some(ball) = &world.ball {
            if let Some(passer) = world.allies_bot.get(&self.id_passer) {
                if let Some(receiver) = world.allies_bot.get(&self.id_receiver) {
                    match self.state {
                        PTState::Placement => {
                            // passer
                            let behind_ball_position = ball.position_2d() + (ball.position_2d() - receiver.pose.position).normalize() * 0.4;
                            let cmd = behind_ball_position - passer.pose.position;
                            action_wrapper.push(
                                self.id_passer,
                                MoveToBuilder::new()
                                    .set_target(behind_ball_position)
                                    .set_orientation(vectors::angle_to_point(passer.pose.position, ball.position_2d()))
                                    .set_avoid_ball(true)
                                    .no_avoidance()
                                    .build()
                                // RawOrder::new(Command {
                                //     forward_velocity: cmd.x as f32,
                                //     left_velocity: cmd.y as f32,
                                //     angular_velocity:  as f32,
                                //     ..Command::default()
                                // }),
                            );

                            // receiver
                            self.receiver_waits(action_wrapper, ball, &receiver);

                            let aligned = vectors::vector_from_angle(passer.pose.orientation).normalize()
                                .dot(&(ball.position_2d() - passer.pose.position).normalize());
                            let dist = passer.distance(&behind_ball_position);
                            println!("aligned: {:?} | dist: {:?}", aligned, dist);
                            // update state
                            if receiver.distance(&ball.position_2d()) < 0.1 {
                                self.state = PTState::Intercept;
                                info!("Intercepting ball")
                            } else if aligned > 0.90 && dist < 0.05 {
                                self.state = PTState::Passing;
                                info!("Passing ball")
                            }
                        }

                        PTState::Passing => {
                            self.receiver_waits(action_wrapper, ball, &receiver);
                            action_wrapper.push(
                                self.id_passer,
                                MoveToBuilder::new()
                                    .set_target(ball.position_2d())
                                    .set_orientation(vectors::angle_to_point(passer.pose.position, receiver.pose.position))
                                    .set_kick(
                                        if passer.distance(&ball.position_2d()) < 0.15 { Some(Kick::StraightKick { power: 4. }) }
                                        else { None }
                                    )
                                    .no_avoidance()
                                    .build()
                            );

                            action_wrapper.push(self.id_passer, shoot(passer, ball, &receiver.pose.position.xy(), world));
                            
                            if ball.velocity.norm() > 0.2 {
                                self.state = PTState::Intercept;
                            }
                        }

                        PTState::Intercept => {
                            // receiver
                            action_wrapper.push(self.id_receiver, intercept(receiver, ball));
                            action_wrapper.push(self.id_passer, MoveTo::default());

                            if receiver.distance(&ball.position_2d()) < 0.15 {
                                self.state = PTState::Shooting
                            }
                        }

                        PTState::Shooting => {
                            Attacker::new(self.id_receiver).step(world, tools_data, action_wrapper);
                            action_wrapper.push(self.id_passer, MoveTo::default());
                        }
                    }
                }
            }
        }
        false
    }
}