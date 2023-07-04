use crate::action::ActionWrapper;
use crate::constants::KEEPER_ID;
use crate::strategy::Strategy;
use crate::strategy::attacker::Shooter;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World};
use super::Defend;

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct Defender {
    /// The id of the robot to move.
    id: u8,
    left: bool,
    strategy: Box<dyn Strategy>
}
impl Default for Defender {
    fn default() -> Self {
        Self { id: KEEPER_ID,left:false, strategy: Box::new(Defend::new(KEEPER_ID, false))}
    }
}
impl Defender {
    /// Creates a new Defender instance with the desired robot id.
    pub fn new(id: u8, left: bool) -> Self {
        Self { 
            id, 
            left,
            strategy: Box::new(Defend::new(id, false))
        }
    }
}

impl Strategy for Defender {
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
        action_wrapper.clean(self.id);       
        let robot = match world.allies_bot.get(&self.id) {
            None => {
                return false;
            }
            Some(robot) => {
                robot
            }
        };
        if let Some(ball) = &world.ball{
            let ball_pos = ball.position_2d();
            if dbg!((robot.pose.position - ball_pos).norm()) < 0.4{
                if self.strategy.name() != "Shooter" {
                    self.strategy = Box::new(Shooter::new(self.id));
                }
            }else{
                if self.strategy.name() != "Defend" {
                    self.strategy = Box::new(Defend::new(self.id, self.left));
                }
            }
        }
        self.strategy.step(world, tools_data, action_wrapper)
    }
    fn name(&self) -> &'static str {
        return "Defender"
    }
}
