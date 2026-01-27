use crate::action::move_to_builder::MoveToBuilder;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use std::f64::consts::{PI, TAU};
use crabe_framework::data::output::Command;
use crate::action::order_raw::RawOrder;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct Square {
    /// The id of the robot to move.
    id: u8,
}

impl Square {
    /// Creates a new Square instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id}
    }
}
fn angle_difference(alpha1: f64, alpha2: f64) -> f64 {
    let diff = alpha1 - alpha2;
    match diff {
        d if d > PI => d - TAU,
        d if d < -PI => d + TAU,
        d => d,
    }
}
impl Strategy for Square {
    fn name(&self) -> &'static str {
        "Square"
    }

                /// Executes the Square strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a square shape in a
    /// counter-clockwise direction.
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
        if let Some(ball) = &world.ball {
            if let Some(rob_info) = world.allies_bot.get(&self.id) {
                // let target = Point2::new(-4., 0.5) - rob_info.pose.position;
                let target = (ball.position.xy() - rob_info.pose.position) * 2.;
                let orientation = rob_info.angle_to(ball.position_2d());
                let z = angle_difference(orientation, rob_info.pose.orientation) * 3.;
                action_wrapper.push(
                    self.id,
                    RawOrder::new(Command {
                        forward_velocity: target.x as f32,
                        left_velocity: target.y as f32,
                        angular_velocity: z as f32,
                        ..Command::default()
                    }),
                );
            }
        }
        false
    }
}
